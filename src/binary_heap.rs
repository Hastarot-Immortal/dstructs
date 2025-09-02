use crate::{has_length::HasLength, heap::{Heap, HeapType}};

#[derive(Clone)]
pub struct BinaryHeap<T: PartialOrd + Clone> {
    elements: Vec<T>,
    h_type: HeapType,
}

impl<T> BinaryHeap<T>
where
    T: PartialOrd + Clone,
{
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            elements: vec![],
            h_type: heap_type,
        }
    }

    pub fn build_min(elements: Vec<T>) -> Self {
        let mut result = Self {
            elements,
            h_type: HeapType::Min,
        };
        result.rebuild_heap();
        result
    } 

    pub fn build_max(elements: Vec<T>) -> Self {
        let mut result = Self {
            elements,
            h_type: HeapType::Max,
        };
        result.rebuild_heap();
        result
    }

    fn rebuild_heap(&mut self) {
        for index in (0..(self.len() >> 1)).rev() {
            self.heapify(index);
        }
    }

    fn heapify(&mut self, index: usize) {
        let mut temp = index;
        let left_index = (index << 1) + 1;
        let right_index = (index << 1) + 2;

        if (left_index < self.len()) && !(self.h_type.is_correct(&self.elements[temp], &self.elements[left_index]))
        {
            temp = left_index;
        }
        if (right_index < self.len())
            && !(self.h_type.is_correct(&self.elements[temp], &self.elements[right_index] ))
        {
            temp = right_index;
        }
        if temp != index {
            self.elements.swap(index, temp);
            self.heapify(temp);
        }
    }

    fn fix_heap(&mut self, mut index: usize) {
        let parent = index >> 1;
        if (index > 0) && !self.h_type.is_correct(&self.elements[parent], &self.elements[index]) {
            self.elements.swap(index, parent);
            index = parent;
            self.fix_heap(index);
        }
    }
}

impl<T> HasLength for BinaryHeap<T>
where
    T: PartialOrd + Clone,
{
    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Heap<T> for BinaryHeap<T>
where
    T: PartialOrd + Clone,
{
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.elements[0])
        }
    }

    fn push(&mut self, value: T) {
        self.elements.push(value);
        let index = self.elements.len() - 1;
        self.fix_heap(index);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last = self.len() - 1;
            self.elements.swap(0, last);
            let result = self.elements.pop();
            if !self.is_empty() {
                self.heapify(0);
            }
            result
        }
    }

    fn meld(&mut self, other: &mut Self) {
        self.elements.append(&mut other.elements);
        self.rebuild_heap();
    }
    
    fn merge(self, other: Self, new_heap_type: HeapType) -> Self {
        let mut result = Self {
            elements: Vec::with_capacity(self.len() + other.len()),
            h_type: new_heap_type,
        };
        result.elements.extend(self.elements.into_iter());
        result.elements.extend(other.elements.into_iter());

        result.rebuild_heap();
        result
    }
}