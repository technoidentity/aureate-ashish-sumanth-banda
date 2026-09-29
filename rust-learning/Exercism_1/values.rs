enum Action{
    Add(u32),
    Multiply(u32),
    Reset,

}
fn apply(current:u32,action:Action)->u32{
    match action{
        Action::Add(amount)=>current+amount,
        Action::Multiply(multi)=>current*multi,
        Action::Reset=>0,
    }
}
fn main(){
    //let action = Action::Add(3);
    let action = Action::Multiply(3);
    let resultv = apply(10,action);

    println!("{resultv}");
}