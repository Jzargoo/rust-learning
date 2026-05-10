use crate::repositories::crud_repository::CrudRepository;

use super::super::InMemoryRepo;
use super::super::User;

#[test]
pub fn save_success_test(){
    let mut repo_test = InMemoryRepo::new();
    let user_test = User::new(
        "Test".to_string(),
        1, 
        "test@gmail.com".to_string());
    
    let res = repo_test.save(user_test.clone());
    
    match res {
        Err(_) => panic!("Cannot save user!"),
        Ok(saved_user) => {
            assert_eq!(&saved_user, &user_test, "!THEY ARE THE SAME FOOLISH PROGRAMMER!")
        }
    }
}

#[test]
#[should_panic(expected="CHILL")]
pub fn check_panic(){
    panic!("I AM RED");
}

