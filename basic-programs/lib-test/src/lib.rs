use crate::front_house::customer_experience::customer_service;

pub mod front_house;

pub fn customer_added_order () {
    customer_service::make_order();
    let mut vec = Vec::new();
    vec.push(1);

}