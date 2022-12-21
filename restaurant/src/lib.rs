mod front_of_house;

//use std::cmp::Ordering;
//use std::io;
//is the same as
use std::{cmp::Ordering, io};

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting as HostingBoy;

//mod customer {
    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_wishlist();
        hosting::add_to_wishlist();
        HostingBoy::add_to_wishlist();

        front_of_house::hosting::add_to_wishlist();
    }
//} . if we put it here the above use statement is now out of scope
