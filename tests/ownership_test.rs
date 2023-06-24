#[cfg(test)]
mod tests {
    #[test]
    fn str_ownership() {
        let s = String::from("hello, world");
        let s1 = s;
        // print!("s: {}",s);   ownership error
        assert_eq!(s1, "hello, world");
    }
}
