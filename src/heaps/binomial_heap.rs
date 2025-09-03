use crate::{has_length::HasLength, heaps::heap::{Heap, HeapType}};

#[derive(Clone)]
pub struct BinomialHeap<T: PartialOrd + Clone> {
    pointer: usize,
    h_type: HeapType,
    length: usize,
    trees: Vec<Option<Box<BinomialTree<T>>>>,
}

impl<T> BinomialHeap<T>
where
    T: PartialOrd + Clone,
{
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            pointer: 0,
            h_type: heap_type,
            length: 0,
            trees: vec![],
        }
    }

    fn merge_trees(
        mut first: BinomialTree<T>,
        mut second: BinomialTree<T>,
        heap_type: HeapType,
    ) -> BinomialTree<T> {
        let mut result;
        if heap_type.is_correct(&first.value, &second.value) {
            first.subtrees.push(Box::new(second));
            result = first;
        } else {
            second.subtrees.push(Box::new(first));
            result = second;
        }
        result.rank += 1;
        result
    }

    fn update_pointer(&mut self) {
        let mut best_idx: Option<usize> = None;
        for (i, t) in self.trees.iter().enumerate() {
            if let Some(tree) = t.as_ref() {
                if let Some(bi) = best_idx {
                    let best_val = &self.trees[bi].as_ref().unwrap().value;
                    if self.h_type.is_correct(&tree.value, best_val) {
                        best_idx = Some(i);
                    }
                } else {
                    best_idx = Some(i);
                }
            }
        }
        self.pointer = best_idx.unwrap_or(0);
    }

    fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len());
        for tree in &self.trees {
            if let Some(tree) = tree {
                result.extend(tree.to_vec());
            } 
        }
        result
    }
}

impl<T> HasLength for BinomialHeap<T>
where
    T: PartialOrd + Clone,
{
    fn len(&self) -> usize {
        self.length
    }
}

enum HeapTypeMatch {
    All,
    First,
    Second,
    None,
}

fn heap_type_match(
    main_type: HeapType,
    first_type: HeapType,
    second_type: HeapType,
) -> HeapTypeMatch {
    if (first_type == main_type) & (second_type == main_type) {
        HeapTypeMatch::All
    } else if first_type == main_type {
        HeapTypeMatch::First
    } else if second_type == main_type {
        HeapTypeMatch::Second
    } else {
        HeapTypeMatch::None
    }
}

struct MergeBucket<T: PartialOrd + Clone> {
    bucket: [Option<Box<BinomialTree<T>>>; 3],
    pointer: isize,
}

impl<T> MergeBucket<T>
where T: PartialOrd + Clone,
{
    fn new() -> Self {
        Self {
            bucket: [const { None }; 3],
            pointer: -1,
        }
    }

    fn len(&self) -> usize {
        (self.pointer + 1) as usize
    }

    fn push(&mut self, value: Box<BinomialTree<T>>) {
        if self.pointer <= 1 {
            self.pointer += 1;
            self.bucket[self.pointer as usize] = Some(value);
        }
    }

    fn pop(&mut self) -> Option<Box<BinomialTree<T>>> {
        if (self.pointer >= 0) && (self.pointer < 3) && self.bucket[self.pointer as usize].is_some() {
            let result = self.bucket[self.pointer as usize].clone();
            self.bucket[self.pointer as usize] = None;
            self.pointer -= 1;
            return result
        }
        None
    }
}

impl<T> Heap<T> for BinomialHeap<T>
where
    T: PartialOrd + Clone,
{
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&(self.trees[self.pointer].as_ref().unwrap().value))
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let tree = self.trees[self.pointer].take().unwrap();

        let k = tree.rank;
        let tree_size = 1usize << k;
        let value = tree.value;

        self.length = self.length.saturating_sub(tree_size);

        let mut other = BinomialHeap {
            length: (1usize << k) - 1,
            pointer: 0,
            h_type: self.h_type,
            trees: vec![None; k],
        };
        for child in tree.subtrees {
            let r = child.rank;
            other.trees[r] = Some(child);
        }

        self.meld(&mut other); 

        Some(value)
    }

    fn push(&mut self, value: T) {
        self.meld(&mut BinomialHeap {
            pointer: 0,
            h_type: self.h_type,
            length: 1,
            trees: vec![Some(Box::new(BinomialTree::from(value)))],
        });
    }

    fn meld(&mut self, other: &mut Self) {
        if self.h_type == other.h_type {
            let heap_type = self.h_type;
            let max_len = self.trees.len().max(other.trees.len());
            self.trees.resize(max_len, None);
            other.trees.resize(max_len, None);

            let mut new_trees: Vec<Option<Box<BinomialTree<T>>>> = Vec::with_capacity(max_len + 1);
            new_trees.resize(max_len + 1, None);

            let mut carry: Option<Box<BinomialTree<T>>> = None;

            for i in 0..max_len {
                let mut bucket = MergeBucket::new();
                if let Some(t) = self.trees[i].take() { 
                    bucket.push(t); 
                }
                if let Some(t) = other.trees[i].take() { 
                    bucket.push(t); 
                }
                if let Some(t) = carry.take() {
                    bucket.push(t); 
                }

                match bucket.len() {
                    1 => {
                        new_trees[i] = Some(bucket.pop().unwrap());
                    }
                    2 => {
                        let b = *bucket.pop().unwrap();
                        let a = *bucket.pop().unwrap();
                        let merged = Self::merge_trees(a, b, heap_type);
                        carry = Some(Box::new(merged));
                    }
                    3 => {
                        let keep = bucket.pop().unwrap();
                        let b = *bucket.pop().unwrap();
                        let a = *bucket.pop().unwrap();
                        new_trees[i] = Some(keep);
                        let merged = Self::merge_trees(a, b, heap_type);
                        carry = Some(Box::new(merged));
                    }
                    _ => {},
                }
            }

            if let Some(t) = carry {
                if new_trees.len() == max_len {
                    new_trees.push(None);
                }
                new_trees[max_len] = Some(t);
            }

            self.trees = new_trees;

            self.length += other.length;
            other.length = 0;
            other.trees.clear();

            
        } else {
            let vec = other.to_vec();
            for value in vec {
                self.push(value);
            }
        }
        self.update_pointer();
    }


    fn merge(mut self, mut other: Self, new_heap_type: HeapType) -> Self {
        match heap_type_match(new_heap_type, self.h_type, other.h_type) {
            HeapTypeMatch::All => {
                self.meld(&mut other);
                self
            },
            HeapTypeMatch::First => {
                let mut other = other.to_vec();
                for value in other.drain(..) {
                    self.push(value);
                }
                self
            },
            HeapTypeMatch::Second => {
                let mut result = other;
                let mut other = self.to_vec();
                for value in other.drain(..) {
                    result.push(value);
                }
                result
            },
            HeapTypeMatch::None => {
                let mut heap = Self::new(new_heap_type);
                let mut first = other.to_vec();
                let mut second = self.to_vec();
                for value in first.drain(..){
                    heap.push(value);
                }
                for value in second.drain(..) {
                    heap.push(value);
                }
                heap
            },
        }
    }
}

#[derive(Clone)]
struct BinomialTree<T: PartialOrd + Clone> {
    rank: usize,
    value: T,
    subtrees: Vec<Box<BinomialTree<T>>>,
}

impl<T> BinomialTree<T>
where
    T: PartialOrd + Clone,
{
    fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(1 << self.rank);
        self.fill_vec(&mut result);
        result
    }

    fn fill_vec(&self, result: &mut Vec<T>) {
        result.push(self.value.clone());
        for subtree in &self.subtrees {
            subtree.fill_vec(result);
        }
    }
}

impl<T> From<T> for BinomialTree<T>
where
    T: PartialOrd + Clone,
{
    fn from(value: T) -> Self {
        Self {
            rank: 0,
            value,
            subtrees: vec![],
        }
    }
}
