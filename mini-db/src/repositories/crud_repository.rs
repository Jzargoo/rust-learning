use std::vec;

pub use crate::entities::entity::Entity;
use crate::repositories::repo_error::RepoError;

pub trait CrudRepository<T: Entity> {
    fn find_all(&self) -> Vec<T> {
        return vec![];
    }
    
    fn save(&mut self, entity: T) -> Result<T, RepoError>;
    fn exists_by_id(&self, id: T::Id) -> bool;
    fn find_by_id(&self, id: T::Id) -> Option<T>;
    fn delete_by_id(&mut self, id: T::Id) -> Result<T,RepoError>;
} 