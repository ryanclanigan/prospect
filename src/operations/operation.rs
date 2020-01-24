pub trait BaseOperation<T> {
    fn apply(&mut self) -> Result<T, &'static str>;
}
