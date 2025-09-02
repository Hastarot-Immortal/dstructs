pub mod heap;
pub mod binary_heap;
pub mod binomial_heap;
pub mod has_length;

#[cfg(test)]
mod bin_heap {
    use super::*;
    use binary_heap::BinaryHeap;
    use heap::{HeapType, Heap};
    
    #[test]
    fn pop() {
        let mut heap: BinaryHeap<u8> = BinaryHeap::new(HeapType::Min);
        heap.push(3);
        heap.push(5);
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn meld() {
        let mut heap1: BinaryHeap<u8> = BinaryHeap::new(HeapType::Max);
        let mut heap2: BinaryHeap<u8> = BinaryHeap::new(HeapType::Min);

        heap1.push(4);
        heap1.push(7);
        heap2.push(5);
        heap2.push(2);
        heap1.meld(&mut heap2);
        assert_eq!(heap1.pop(), Some(7));
        assert_eq!(heap1.pop(), Some(5));
        assert_eq!(heap1.pop(), Some(4));
        assert_eq!(heap1.pop(), Some(2));
        assert_eq!(heap2.peek(), None);
    }

    #[test]
    fn merge() {
        let mut heap1: BinaryHeap<u8> = BinaryHeap::new(HeapType::Max);
        let mut heap2: BinaryHeap<u8> = BinaryHeap::new(HeapType::Min);

        heap1.push(4);
        heap1.push(7);
        heap2.push(5);
        heap2.push(2);
        let heap3 = heap1.clone().merge(heap2.clone(), HeapType::Max);
        assert_eq!(heap1.peek(), Some(&7));
        assert_eq!(heap2.peek(), Some(&2));
        assert_eq!(heap3.peek(), Some(&7));
    }
}

#[cfg(test)]
mod binom_heap {
    use crate::{binomial_heap::BinomialHeap, heap::Heap};

    #[test]
    fn pop() {
        let mut heap = BinomialHeap::new(crate::heap::HeapType::Min);
        heap.push(2);
        heap.push(5);
        heap.push(1);
        heap.push(6);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn meld() {
        let mut heap1 = BinomialHeap::new(crate::heap::HeapType::Min);
        let mut heap2 = BinomialHeap::new(crate::heap::HeapType::Min);
        heap1.push(2);
        heap1.push(5);
        heap1.push(4);

        heap2.push(1);
        heap2.push(3);
        heap2.push(6);
        heap2.push(9);
        heap2.push(8);
        heap2.push(11);

        heap1.meld(&mut heap2);

        assert_eq!(heap1.peek(), Some(&1));
        assert_eq!(heap2.peek(), None);
    }

    #[test]
    fn merge() {
        let mut heap1 = BinomialHeap::new(crate::heap::HeapType::Min);
        let mut heap2 = BinomialHeap::new(crate::heap::HeapType::Min);
        heap1.push(2);
        heap1.push(5);
        heap1.push(4);

        heap2.push(1);
        heap2.push(3);
        heap2.push(6);
        heap2.push(9);
        heap2.push(8);
        heap2.push(11);

        let heap3 = heap1.clone().merge(heap2.clone(), crate::heap::HeapType::Max);

        assert_eq!(heap1.peek(), Some(&2));
        assert_eq!(heap2.peek(), Some(&1));
        assert_eq!(heap3.peek(), Some(&11));
    }
}