use trpl::Html;
fn main(){
    let html = "<html><head><title>Ashish's page</title></head></html>";
    let document = Html::parse(html);
    let title = document
        .select_first("title")
        .map(|element|element.inner_html());
    println!("{title:?}");
}