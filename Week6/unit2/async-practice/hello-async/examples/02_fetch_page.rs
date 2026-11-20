fn main() {
    trpl::block_on(async {
        let url = "https://example.com";

        println!("1. Requesting the webpage");
        let response = trpl::get(url).await;

        println!("2. Reading the response body");
        let response_text = response.text().await;

        println!("3. HTML received:");
        println!("{response_text}");
    });
}