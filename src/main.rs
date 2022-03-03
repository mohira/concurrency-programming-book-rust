use std::sync::{Arc, Mutex, Condvar}; // ①
use std::thread;

// Condvar型の変数が条件変数であり
// Mutex と Condvar を含むタプルがArcに包んで渡される
fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) { // ②
    let &(ref lock, ref cvar) = &*p;

    // まず、ミューテックスロックを行う
    let mut started = lock.lock().unwrap(); // ③

    while !*started { // Mutex中の共有変数が false の間ループ
        // wait で 待機
        started = cvar.wait(started).unwrap(); // ④
    }

    // 以下のように、 wait_while を使うことも可能
    // cvar.wait_while(started, |started| ~*started).unwrap();

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, 
        ref cvar) = &*p;

    // まず、ミューテックスロックをおこなう ⑥
    let mut started = lock.lock().unwrap();
    *started = true; // 共有変数を更新


    // pthred_cond_broadcast(cvar) 
    cvar.notify_all(); // 通知

    println!("parent");
}

fn main() {
    // ミューテックスと条件変数を作成
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || {child(0, pair0)});
    let c1 = thread::spawn(move || {child(1, pair1)});

    let p  = thread::spawn(move || {parent(pair2)});

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}