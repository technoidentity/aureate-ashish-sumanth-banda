fn passing_scores(scores: &[i32])-> Vec<i32>{
    scores
        .iter()
        .copied()
        .map(|score|score+5)
        .filter(|score|*score >= 60)
        .collect()
}
fn main(){
    let scores = vec![55,60,75];
    println!("{:?}",passing_scores(&scores));

}