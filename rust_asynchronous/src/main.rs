use std::time::Duration;

use trpl::Either;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text_page = trpl::get(url).await.text().await;
    let title = trpl::Html::parse(&text_page)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

async fn get_page_titles() {
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
}

async fn thread_like_using_async() {
    let handler = trpl::spawn_task(async {
        for i in 1..10 {
            println!("The i is {i} in the first task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("The i is {i} in the second task");
        trpl::sleep(Duration::from_millis(500)).await;
    }

    handler.await.unwrap();
}

async fn join_futures() {
    let fut_1 = async {
        for i in 1..10 {
            println!("The i is {i} in the first task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut_2 = async {
        for i in 1..5 {
            println!("The i is {i} in the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut_1, fut_2).await;
}

fn main() {
    trpl::block_on(async {
        join_futures().await;
    })
}
