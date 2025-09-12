pub trait HasLength {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Insertable<T> {
    fn new() -> Self;
    fn insert(&mut self, value: T);
    fn contains(&self, value: &T) -> bool; 
}

pub trait Container<T>: HasLength + Insertable<T> {}