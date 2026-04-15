
use crate::entities::entity::Entity;

#[derive(Clone, Debug)]
pub struct User {
    id: u64,
    pub name: String,
    pub email: String     
}

impl Entity for User {
    type Id = u64;

    fn get_id(&self) -> Self::Id {
        self.id
    }

    fn set_id(&mut self, new: Self::Id) {
        self.id = new;
    }
}

impl User {
    pub fn new(name: String, id:u64, email: String) -> Self {
        User {
            id,
            name,
            email
        }
    } 
}
