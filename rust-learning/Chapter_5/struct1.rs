struct user { 
    active: bool,
    username: string,
    email: string,
    sign_in_count: u64,
}

let mut user1 =user{
    active: true,
    username: string::from("ashish"),
    email: string::from("ashish@example.com"),
    sign_in_count: 1,

};

println!("{}",user1.username);
user1.email =string::from("new@ecample.com");
user1.sign_in_count +=1;
