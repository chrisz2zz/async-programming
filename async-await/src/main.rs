use std::future::{Future, self};

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

fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move {*x}
}

fn main() {
    let f = foo();
    
    println!("{}, {}", block_on(bar()), block_on(f));
}
