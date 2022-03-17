use std::sync::{Arc, Barrier, RwLock};
use std::thread;

fn increment(sum: &Arc<RwLock<u32>>) {
  let mut s = sum.write().unwrap();
  println!("increment {}", s);
  *s += 1;
}

fn main() {
  let mut v = Vec::new();

  let barrier = Arc::new(Barrier::new(10));

  let sum = Arc::new(RwLock::new(0));
  for _ in 0..10 {
    let b = barrier.clone();
    let s = sum.clone();

    let th = thread::spawn(move || {
      increment(&s);

      //b.wait(); // ←ここをコメントアウトすると :zany_face:

      println!("sum is {}", s.read().unwrap());
    });

    v.push(th);
  }
  for th in v {
    th.join().unwrap();
  }
}