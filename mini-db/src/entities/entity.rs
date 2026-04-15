use std::hash::Hash;

pub trait Entity: Clone {
    type Id: Clone + Eq + PartialOrd + Hash;

    fn get_id(&self) -> Self::Id;
    fn set_id(&mut self, new: Self::Id);
}