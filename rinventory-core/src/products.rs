pub struct Product {
    pub id: u64,
    pub name: String,
    pub is_active: bool,
    pub uses_lots: bool
}

pub fn log_product(msg: &str) {
    println!("Product log: {}", msg);
}