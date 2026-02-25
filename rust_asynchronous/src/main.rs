use trpl::Either;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text_page = trpl::get(url).await.text().await;
    let title = trpl::Html::parse(&text_page)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    trpl::block_on(async {
        let title_fut_1 = page_title("https://www.rust-lang.org");
        let title_fut_2 = page_title("https://www.calebleak.com/");

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(val) => val,
            Either::Right(val) => val,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
