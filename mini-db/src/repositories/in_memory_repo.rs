use std::collections::HashMap;

use crate::entities::entity::Entity;
use crate::repositories::{
    repo_error::RepoError,
    crud_repository::CrudRepository
};

pub struct InMemoryRepo<T:Entity> {
    data: HashMap<T::Id, T>
}

impl<T:Entity> InMemoryRepo<T> {
    pub fn new() -> Self {
        InMemoryRepo { 
            data: HashMap::new()
        }
    } 

}

impl<T:Entity> CrudRepository<T> for InMemoryRepo<T>{
    fn save(&mut self, entity: T) -> Result<T, RepoError> {
        
    let id = entity.get_id();

    self.data.insert(id, entity.clone());

    Ok(entity)
  
    }

    fn find_all(&self) -> Vec<T> {
        self.data.values().cloned().collect()
    }

    fn find_by_id(&self, id: T::Id) -> Option<T> {
        self.data.get(&id).cloned()
    }

    fn delete_by_id(&mut self, id: T::Id) -> Result<T, RepoError> {
        let removed = self.data.remove(&id);

        match removed {
            None => Result::Err(RepoError::ElHasNotExisted),
            Some(v) => Result::Ok(v)
        }
    }
    

    
    fn exists_by_id(&self, id: <T as Entity>::Id) -> bool {
        self.data.contains_key(&id)
    }
}