struct OrderItem{
    price:u32,
    quality:u32,
}
impl OrderItem{
    fn build(price:u32,quality:u32)->Option<OrderItem>{
        if price > 0 && quality > 0{
            Some(OrderItem{price,quality})
        }
        else{
            None
        }
    }
    fn total_cost(&self)->u32{
        let cost = self.price*self.quality;
        cost
    }
    fn is_bulk(&self)->bool{
        if self.quality >= 10{
            true
        }
        else{
            false
        }
    }
}   
fn main(){
    let item = OrderItem::build (50,0);
    
    match item{
        Some(item)=>{
            let total = item.total_cost();
            println!("cost:{total}");
            let bul = item.is_bulk();
            println!("bulk:{bul}");
        }
        None => {
            println!("Invalid item");
        }
    }
}
