struct Product{
    price: u32,
    quantity: u32,
}
impl Product{
    fn total_cost(&self)->u32{
        let mut cost = 0;
        cost = self.price*self.quantity;
        cost
    }
}
fn main(){
    let product1 = Product{
        price: 50,
        quantity: 3,
    };
    let product2 = Product{
        price: 20,
        quantity: 4,
    };
    let expected_cost1 = product1.total_cost();
    let expected_cost2 = product2.total_cost();
    println!("{}",expected_cost1);
    println!("{}",expected_cost2);
}