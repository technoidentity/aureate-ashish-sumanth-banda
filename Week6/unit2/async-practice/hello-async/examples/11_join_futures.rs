use std::time::{Duration, Instant};

fn main() {
    trpl::block_on(async {
        let start = Instant::now();

        let food = async {
            println!("Food started");
            trpl::sleep(Duration::from_secs(2)).await;
            println!("Food ready");
            "Pizza"
        };

        let drink = async {
            println!("Drink started");
            trpl::sleep(Duration::from_secs(1)).await;
            println!("Drink ready");
            "Juice"
        };

        let (food_result, drink_result) =
            trpl::join(food, drink).await;

        println!("Serving: {food_result} and {drink_result}");
        println!("Elapsed: {:?}", start.elapsed());
    });
}