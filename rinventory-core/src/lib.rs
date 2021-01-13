pub mod products;

pub fn log(msg: &str) {
    println!("{}", msg);
    products::log_product(msg);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
