pub trait BaseOperation<T> {
    fn get_args(&self) -> &Box<[&T]>;
    fn get_op(&self) -> Box<dyn Fn(&Box<[&T]>) -> Result<T, &'static str>>;

    fn apply(&self) -> Result<T, &'static str> {
        self.get_op()(self.get_args())
    }
}
