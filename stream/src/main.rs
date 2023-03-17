use std::pin::Pin;

use futures::{channel::mpsc, Stream, io, FutureExt};

fn main() {
    println!("Hello, world!");

    futures::executor::block_on(send_recv());
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    
    tx.try_send(1).unwrap();
    tx.try_send(2).unwrap();
    drop(tx);

    assert_eq!(Some(1), rx.try_next().unwrap());
    assert_eq!(Some(2), rx.try_next().unwrap());
    assert_eq!(None, rx.try_next().unwrap());
    println!("finish");
}


async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, ()>>>
) -> Result<i32, ()> {
    use futures::stream::TryStreamExt;
    let mut sum = 0;
    while let Some(item) = stream.try_next().await.unwrap() {
        sum += item;
    }
    Ok(sum)
}