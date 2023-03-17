use futures::{join, try_join, TryFutureExt};

fn main() {
    println!("Hello, world!");
    let (a, b) = futures::executor::block_on(get_book_and_music());
    println!("{}, {}", a, b);
    let (a, b) = futures::executor::block_on(get_book2_and_music2()).unwrap();
    println!("{}, {}", a, b);
}

async fn get_book_and_music() -> (String, String) {
    let book = get_book();
    let music = get_music();
    join!(book, music)
}

async fn get_book() -> String {
    "book".to_string()
}

async fn get_music() -> String {
    "music".to_string()
}

async fn get_book2_and_music2() -> Result<(String, String), String> {
    use futures::FutureExt;
    let book2 = get_book2();
    let music2 = get_music2().map_err( |_| "".to_string() );

    try_join!(book2, music2)
}

async fn get_book2() -> Result<String, String> {
    Ok("book".to_string())
}

async fn get_music2() -> Result<String, ()> {
    Ok("music".to_string())
}