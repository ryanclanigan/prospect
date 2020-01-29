pub trait BaseOperation {
    type Primitive;

    fn apply(&mut self) -> Result<Self::Primitive, &'static str>;
}
