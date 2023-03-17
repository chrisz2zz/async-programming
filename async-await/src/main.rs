use std::future::Future;

use futures::executor::block_on;

async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

async fn foo1(x: &u8) -> u8 {
    *x
}

#[allow(unused)]
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move {*x}
}

// 'x' doesn`t live enough
// fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     foo1(&x)
// }

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        foo1(&x).await
    }
}

async fn blocks() {
    let my_string = "foo".to_string();
    let future_one = async {
        println!("{my_string}");
    };

    let future_two = async {
        println!("{my_string}");
    };

    let ((), ()) = futures::join!(future_one, future_two);
}

fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    let ret = async move {
        println!("{my_string}");
    };

    // println!("{}", my_string); //has been moved

    ret
}

fn main() {
    let f = foo();
    
    block_on(blocks());
    block_on(move_block());
    println!("{}, {}, {}", block_on(bar()), block_on(f), block_on(good()));
}
