use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|element| element.inner_html());

    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Please provide two URLs");
        return;
    }

    trpl::block_on(async {
        let first = page_title(&args[1]);
        let second = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(first, second).await {
                Either::Left(result) => result,
                Either::Right(result) => result,
            };

        println!("{url} finished first");

        match maybe_title {
            Some(title) => println!("Page title: {title}"),
            None => println!("The page had no title"),
        }
    });
}