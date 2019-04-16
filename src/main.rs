fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn clousure() {
        let f = |x| x + 1;
        assert_eq!(2, f(1));
    }

}
