fn main(){
    let mut buckets = vec![Vec::new();5];
    let entries = vec![
        (12_usize,"ashish"),
        (7_usize, "Ravi"),
        (23_usize,"sara"),
    ];
    for (key,value) in entries{
        let index = key % buckets.len();
        buckets[index].push((key,value));
    }
    let target = 7;
    let index = target % buckets.len();
    let mut found = None;
    for (stored_key,stored_value) in &buckets[index]{
        println!("Checking ID:{}", stored_key);
        if *stored_key == target{
            found = Some(*stored_value);
            break;
        }
    }
    println!("Result: {:?}",found);
}