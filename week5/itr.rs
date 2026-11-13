fn main(){
    let score = vec![50,60,70];
    let updated: Vec<i32> = scores.iter().copied()
        .map(|score|score+10)
        .collect();
    assert_eq!(updated, vec![60,70,80]);
    println!("{?}",updated);
}