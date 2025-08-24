pub enum HeapType {
    Min,
    Max,
}

impl HeapType {
    pub fn is_correct<T: PartialOrd>(
        &self,
        array: &Vec<T>,
        parent_index: usize,
        child_index: usize,
    ) -> bool {
        match self {
            HeapType::Min => array[child_index] > array[parent_index],
            HeapType::Max => array[child_index] < array[parent_index],
        }
    }
}

pub trait Heap<T: PartialOrd> {
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn meld(&mut self, other: &mut Self);
}
