use uuid::Uuid;

pub trait Item {
    fn get_id(&self) -> Uuid;
}
