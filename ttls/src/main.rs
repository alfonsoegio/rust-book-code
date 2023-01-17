use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct SomethingElse {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn best_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn string_notify<T: Summary>(item: &T) -> String {
    format!("Breaking news! {}", item.summarize())
}

pub fn more_notify<T: Summary + fmt::Display>(item: &T) -> String {
    string_notify(item)
}

impl Summary for SomethingElse {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}




fn main() {

    println!("Hello, world!");
    let something_else = SomethingElse {};
    println!("{}", something_else.summarize());

    let article = NewsArticle {
        headline: String::from("Hello World"),
        author: String::from("Federico Jimenez Losantos"),
        location: String::from("Teruel"),
        content: String::from("El peloton del pasmo")
    };
    // println!("-----------------");
    // more_notify(&article);
    // println!("-----------------");

    let tweet = Tweet {
        content: String::from("Goodbye World"),
        username: String::from("El Xokas"),
        reply: false,
        retweet: true
    };

    notify(&tweet);
    notify(&article);
    best_notify(&tweet);
    best_notify(&article);
    println!("{}", article.summarize());
    println!("{}", tweet.summarize());

    let number_list = vec![12, 23, 243, 34, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("{}", largest);
    let found = find_largest(&number_list);
    println!("{}", found);
    // let generic_found = generic_find_largest(&number_list);
    // println!("{}", generic_found);
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// not working ... T is cmp? no one ensures
// fn generic_find_largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
