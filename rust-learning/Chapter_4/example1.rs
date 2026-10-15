fn main(){ 
    let coins = 12; 
    {
        let label = String::from("gold"); 
        println!("{coins} {label}"); 
    } 
    println!("{coins}");
    
}

