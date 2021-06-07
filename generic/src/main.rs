mod points;
mod news;
mod pairs;

use crate::points::{Point, largest};
use crate::news::{Summary, NewsArticle, Tweet, notify};
use crate::pairs::Pair;

fn main() {
    use_generic();
    use_trait();
    use_trait_condition_separation();
}

fn use_generic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result_num = largest(&number_list);
    let result_char = largest(&char_list);
    println!("Max num = {}, Max char = {}", result_num, result_char);

    // No generics means that the caller has to change the function according to the type.
    /*
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);    

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    */

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("x: {}, y: {}", p3.x, p3.y);
}

fn use_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
}

fn use_trait_condition_separation() {
    let pair = Pair { x: 10, y: 20 };
    let pair2 = Pair::new(111, 222);
    pair.cmp_display();
    pair2.cmp_display();
}