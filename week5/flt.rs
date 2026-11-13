lets scores = vec![40,60,75,50];
let passed: Vec<i32> = scores.iter().copied()
    .filter(|score|*score>=60)
    .collect();
assert_eq!(passed,vec![60,75]);