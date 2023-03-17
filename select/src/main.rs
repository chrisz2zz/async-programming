use futures::{FutureExt, pin_mut, select, Stream, stream::FusedStream, StreamExt};

fn main() {
    println!("Hello, world!");

    futures::executor::block_on(race_tasks());
    futures::executor::block_on(count());

    println!("{}", 
        futures::executor::block_on(add_two_streams(futures::stream::empty(), futures::stream::empty())));
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
    }
}

async fn task_one() {
    let mut _sum = 0;
    for i in 1..=100 {
        _sum += i;
    }
}

async fn task_two() {
    let mut _sum = 0;
    for i in 1..=100 {
        _sum += i;
    }
}

async fn count() {
    let mut a_fut = futures::future::ready(6);
    let mut b_fut = futures::future::ready(4);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(),
        };
    }
    assert_eq!(total, 10);
}

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }
    total
}