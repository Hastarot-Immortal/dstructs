use crate::heap::{Heap, HeapType};

pub struct BinaryHeap<T: PartialOrd> {
    elements: Vec<T>,
    h_type: HeapType,
}

impl<T> BinaryHeap<T>
where
    T: PartialOrd,
{
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            elements: vec![],
            h_type: heap_type,
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn rebuild_heap(&mut self, elements: &mut Vec<T>) {
        self.elements.append(elements);
        for index in (0..(self.len() >> 1)).rev() {
            self.heapify(index);
        }
    }

    fn heapify(&mut self, index: usize) {
        let mut temp = index;
        let left_index = (index << 1) + 1;
        let right_index = (index << 1) + 2;

        if (left_index < self.len()) && !(self.h_type.is_correct(&self.elements, temp, left_index))
        {
            temp = left_index;
        }
        if (right_index < self.len())
            && !(self.h_type.is_correct(&self.elements, temp, right_index))
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
        if (index > 0) && !self.h_type.is_correct(&self.elements, parent, index) {
            self.elements.swap(index, parent);
            index = parent;
            self.fix_heap(index);
        }
    }
}

impl<T> Heap<T> for BinaryHeap<T>
where
    T: PartialOrd,
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
        self.rebuild_heap(&mut other.elements);
    }
}