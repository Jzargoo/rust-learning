mod entities;
mod repositories;

use entities::user::User;
use repositories::crud_repository::CrudRepository;
use repositories::in_memory_repo::InMemoryRepo;
use entities::entity::Entity;

use crate::repositories::repo_error::RepoError;


fn main() -> Result<(), RepoError>{
    let mut user = User::new(
        "Jack".to_string(), 
        1,
         "jack@gmail.com".to_string());

    user.set_id(2);
    
    println!("User: {:?}", user);

    let mut repo: InMemoryRepo<User> = InMemoryRepo::new();

    let mut saved_user = repo.save(user)?;
    
    println!("User that has been saved: {:?}", saved_user);
    
    let bool = repo.exists_by_id(saved_user.get_id());

    println!("User with id {} existed ? {}", saved_user.get_id(), bool);

    saved_user.name = "John".to_string();
    
    saved_user.email = "John_123@mail.ru".to_string();

    

    println!("New user is : {:?}", saved_user);

    println!("Find all before update are : {:?}", repo.find_all());

    let saved_user = repo.save(saved_user)?;

    println!("Find all after update are : {:?}", repo.find_all());

    println!("Potential user is : {:?}", repo.find_by_id(saved_user.get_id()));

    let user = repo.delete_by_id(saved_user.get_id())?;

    println!("We deleted user with id {}, its def is {:?}", saved_user.get_id(), user);

    repo.delete_by_id(4)?;

    Result::Ok(())
}
