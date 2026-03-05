use trpl::{Either, Html};

async fn page_title(url: &str) -> Option<String> {
    // let response = trpl::get(url).await;
    // let response_txt = response.text().await;
    let response_txt = trpl::get(url).await.text().await;
    Html::parse(&response_txt)
        .select_first("title")
        .map(|title| title.inner_html())
}

// Under the hood, the rust compiler transform the previous function
// to something like this.
// use std::future::Future;
// use trpl::Html;

// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

async fn page_title2(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let url1 = "https://www.rust-lang.org";
    let url2 = "https://www.google.com/";

    // Rust doesn't provide a runtime to run async code, so we're
    // using tokio here.
    trpl::block_on(async {
        match page_title(url1).await {
            Some(title) => println!("Title: {}", title.trim()),
            None => println!("No title found"),
        }
    });

    // Now we call two urls
    trpl::block_on(async {
        // These are futures, nothing is executed here yet.
        let title_fut_1 = page_title2(url1);
        let title_fut_2 = page_title2(url2);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        // println!("{url} returned first");
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{}'", title.trim()),
            None => println!("It had no title."),
        }
    })
}
