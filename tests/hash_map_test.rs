#[cfg(test)]
mod tests {
    #[test]
    fn hash_map_new() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 20);
        assert_eq!(2, scores.len());
    }

    #[test]
    fn hash_map_access() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 20);
        assert_eq!(10, *scores.get("Blue").unwrap());
        assert_eq!(20, *scores.get(&String::from("Yellow")).unwrap());
        for (key, value) in &scores {
            println!("{} : {}", key, value)
        }
    }
}
