fn main(){
    let name = String::from("Ashish");
    let greet = || println!("Hello,{name}");
    greet();
    greet();

    let mut count = 0;
    let mut increment = ||{
        count += 1;
        count
    };
    assert_eq!(increment(),1);
    assert_eq!(increment(),2);

    let message = String::from("Finished");
    let take_message = move || message;
    let result = take_message();
    assert_eq!(result,"Finished");

    let numbers = vec![10,20];
    let mut iterator = numbers.iter();
    assert_eq!(iterator.next(), Some(&10));
    assert_eq!(iterator.next(),Some(&20));
    assert_eq!(iterator.next(),None);

    let mut scores = vec![10,20];
    for score in scores.iter_mut(){
        *score += 5;
    }
    assert_eq!(scores,vec![15,25]);

    let total: i32 = scores.into_iter().sum();
    assert_eq!(total,40);
    println!("all checks passed");
}