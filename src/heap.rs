#[derive(Clone, Copy, PartialEq)]
pub enum HeapType {
    Min,
    Max,
}

impl HeapType {
    pub fn is_correct<T: PartialOrd>(
        &self,
        parent: &T,
        child: &T,
    ) -> bool {
        match self {
            HeapType::Min => child > parent,
            HeapType::Max => child < parent,
        }
    }
}

pub trait Heap<T: PartialOrd + Clone> {
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn meld(&mut self, other: &mut Self);
    fn merge(self, other: Self, new_heap_type: HeapType) -> Self;
}
