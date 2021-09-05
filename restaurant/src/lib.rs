fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {

    }

    # [derive(Debug)]
    pub struct Breakfast {
        pub toast: String, // 可修改
        seasonal_fruit: String, // 不可修改 meal.seasonal_fruit获取都不给获取了
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 内部都是public的
    pub enum Appetizer {
        Soup,
        Salad
    }
}

mod front_of_house;
pub use crate::front_of_house::hosting;
// 有pub可以用hosting::add_to_waitlist
// 没有的话 外部就不能够使用hosting::add_to_waitlist
// 只有内部可以使用 像eat_at_restaurant
// 写pub use的目的是 外部路径可以隐藏front_of_house

use crate::front_of_house::hosting::add_to_waitlist; // 绝对路径
// use self::front_of_house::hosting; 相对路径

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd lick {:?} toast please", meal);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::fmt;
use std::io;

use std::fmt::Result as r1;
use std::fmt::Result as r2;

// 方便地引入
use std::{
    cmp::Ordering,
    io
};

use std::io::{
    self,
    Write
};

use std::collections::*;

fn function1() -> fmt::Result {}

fn function2() -> io::Result {}
