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

    {
        for i in 1..5 {
            println!("The i is {i} in the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };
    fut_1.await;
    // trpl::join(fut_1, fut_2).await;
}

async fn message_passing_between_futures() {
    let (tx, mut rx) = trpl::channel();

    let values = vec!["value 1", "value 2", "value 3"];

    let fut_1 = async {
        for val in values {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut_2 = async {
        while let Some(v) = rx.recv().await {
            println!("Value comming: {v}");
        }
    };

    trpl::join(fut_1, fut_2).await;
}

async fn using_join_macro() {
    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();

    let fut_1 = async move {
        let data = vec!["Hi", "from", "Loi"];

        for val in data {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_secs(1)).await;
        }
    };

    let fut_2 = async {
        while let Some(val) = rx.recv().await {
            println!("Received message: {val}");
        }
    };

    let fut_3 = async move {
        let data = vec!["and", "have", "a good day"];

        for val in data {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_secs(1)).await;
        }
    };

    trpl::join!(fut_1, fut_3, fut_2);
}

fn main() {
    trpl::block_on(async {
        using_join_macro().await;
    })
}
