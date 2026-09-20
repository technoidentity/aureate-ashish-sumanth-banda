fn grain(square: u32 ) -> u64{
    let mut grains = 1;
    
    for _ in 1..square{
        grains = grains * 2;
    }

    grains
}

fn main(){
    println!("{}", grain(10));
}