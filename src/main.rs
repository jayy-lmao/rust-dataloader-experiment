use dataloader::{BatchFn, BatchFuture, Loader};
use futures::{executor, future, FutureExt as _, TryFutureExt as _};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Batcher;

impl BatchFn<i32, i32> for Batcher {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<i32, Self::Error> {
        println!("load batch {:?}", keys);
        future::ready(keys.into_iter().map(|v| v * 10).collect())
            .unit_error()
            .boxed()
    }
}
// struct Loaders {
//     author_loader: Loader<i32, i32, (), Batcher>,
// }

fn main() {
    let (tx, rx) = mpsc::channel();
    let loader = Loader::new(Batcher);
    let author_loader = Arc::new(loader.cached());
    // let v1 = author_loader.load(3);
    // let v2 = author_loader.load(3);
    let vec = vec![1, 2, 3, 4];
    for val in vec {
        let clone_loader = Arc::clone(&author_loader);
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let res = clone_loader.load(val);
            let res_resolved = executor::block_on(res);
            tx1.send(res_resolved.unwrap());
        });
    }
    for received in rx {
        println!("{}", received);
    }
}
