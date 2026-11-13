let numbers = vec![10,20,30];
let total = numbers.iter().copied()
    .fold(0,|total,number| total + number);
assert_eq!(total,60);