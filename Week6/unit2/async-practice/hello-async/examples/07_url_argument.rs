use trpl::Html;
async fn page_title(url:&str)->Option<String>{
    let text = trpl::get(url).await.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|element|element.inner_html())
}
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()<2{
        println!("please provide a URL");
        return;
    }
    trpl::block_on(async{
        let url = &args[1];
        match page_title(url).await{
            Some(title)=>println!("the title for {url} is : {title}"),
            None => println!("{url} has no title"),
        }
    });
}