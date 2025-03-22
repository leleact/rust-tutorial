pub fn str_echo(str: &str) -> &str {
    println!("The string of fn parameter is {}", str);
    return str;
}

#[cfg(test)]
mod tests {
    #[test]
    fn use_str_echo() {
        use crate::math::sin::str_echo;
        let str = "hello";
        let s = str_echo(str);
        assert_eq!("hello", s);
    }
}