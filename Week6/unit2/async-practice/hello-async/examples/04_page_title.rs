use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|element| element.inner_html())
}

fn main() {
    trpl::block_on(async {
        let title = page_title("https://example.com").await;

        match title {
            Some(text) => println!("Page title: {text}"),
            None => println!("This page has no title"),
        }
    });
}