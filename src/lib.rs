pub mod heaps;
pub mod container;
pub mod probalistic;

#[cfg(test)]
mod binom_heap {
    use crate::{heaps::binomial_heap::BinomialHeap, heaps::heap::{Heap,HeapType}};

    #[test]
    fn pop() {
        let mut heap = BinomialHeap::new(HeapType::Min);
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
        let mut heap1 = BinomialHeap::new(HeapType::Min);
        let mut heap2 = BinomialHeap::new(HeapType::Min);
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
        let mut heap1 = BinomialHeap::new(HeapType::Min);
        let mut heap2 = BinomialHeap::new(HeapType::Min);
        heap1.push(2);
        heap1.push(5);
        heap1.push(4);

        heap2.push(1);
        heap2.push(3);
        heap2.push(6);
        heap2.push(9);
        heap2.push(8);
        heap2.push(11);

        let heap3 = heap1.clone().merge(heap2.clone(), HeapType::Max);

        assert_eq!(heap1.peek(), Some(&2));
        assert_eq!(heap2.peek(), Some(&1));
        assert_eq!(heap3.peek(), Some(&11));
    }
}

#[cfg(test)]
mod b_filter {
    use super::probalistic::bloom_filter::BloomFilter;

    #[test]
    fn insert_and_contains() {
        let mut bf = BloomFilter::with_capacity(5);
        bf.insert(10u8);
        bf.insert(12);
        bf.insert(13);
        bf.insert(14);
        bf.insert(15);
        bf.insert(16);
        assert!(bf.contains(&10));
        assert!(!bf.contains(&7));
        assert!(!bf.contains(&16));
    }
}