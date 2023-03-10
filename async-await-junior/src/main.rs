use std::{thread, time};

use futures::executor::block_on;

async fn hello_world() {
    println!("hello world!");
}

async fn learn_song() -> String {
    thread::sleep(time::Duration::from_secs(5));
    "learn song".to_string()
}

async fn sing_song(song: String) {
    println!("I`m singing a song: {}", song);
}

async fn dance() {
    println!("I`m dancing.");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
    let future = hello_world();
    block_on(future);

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main());
}
