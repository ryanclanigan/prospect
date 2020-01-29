use anyhow::Error;

pub trait BaseOperation {
    type Primitive;

    fn apply(&mut self) -> Result<Self::Primitive, Error>;
}
