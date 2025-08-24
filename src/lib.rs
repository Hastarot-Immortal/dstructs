pub mod heap;
pub mod bin_heap;

#[cfg(test)]
mod binary_heap {
    use super::*;
    use bin_heap::BinaryHeap;
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
        let heap3 = heap1.merge(&mut heap2, HeapType::Max);
        assert_eq!(heap1.peek(), Some(&7));
        assert_eq!(heap2.peek(), Some(&2));
        assert_eq!(heap3.peek(), Some(&7));
    }
}
