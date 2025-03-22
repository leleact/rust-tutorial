mod math;
mod traits;

use traits::use_of_traits::tr::Summary as _;
use crate::traits::use_of_traits::st::Article;

fn main() {
    let article = Article {
        headline: String::from("use of traits"),
        location: String::from("2"),
        author: String::from("leleact"),
        content: String::from("use summarize trait function must import mod explicitly"),
    };
    let s = article.summarize();
    println!("{:?}", s);
}

#[cfg(test)]
mod tests {

    #[test]
    fn clousure() {
        let f = |x| x + 1;
        assert_eq!(2, f(1));
    }
}
