fn add_bonus(points: &mut i32){

    *points += 5;

}
fn main(){
    let mut score = 10;
    add_bonus(&mut score);
    println!("{score}");
}