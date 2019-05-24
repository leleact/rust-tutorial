/// use trait
///
/// if struct impl a trait
/// when someone want to use of this struct trait function
/// the trait mod must import explicitly.
///
pub mod tr {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
}

pub mod st {
    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Article {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl super::tr::Summary for Article {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn use_tr() {
        use crate::traits::use_of_traits::{st, tr::Summary};
        let article = st::Article {
            headline: String::from("use of traits"),
            location: String::from("2"),
            author: String::from("leleact"),
            content: String::from("use summarize trait function must import mod explicitly"),
        };
        println!("{:?}", article);
        assert_eq!("use of traits, by leleact (2)", article.summarize());
    }
}
