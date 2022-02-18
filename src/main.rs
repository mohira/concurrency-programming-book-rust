use std::sync::{Arc, Mutex}; // ①
use std::thread;

fn some_func(lock: Arc<Mutex<u64>>) {
    //②
    loop {
        // ロックしないとMutex型の中の値は参照不可
        let mut val = lock.lock().unwrap(); //❸ lock 関数を呼び出してロックして保護対象データの参照を取得。
        *val += 1;
        println!("{}", *val);
    }
}

fn main() {
    // Arcはスレッドセーフな参照カウンタ型のスマートポインタ
    let lock0 = Arc::new(Mutex::new(0)); // ④
                                         //    let lock0 = Arc::new((0)); // ④

    // 参照カウンタがインクリメントされるので
    // 中身はクローンされない
    let lock1 = lock0.clone();

    // スレッド生成
    // クロージャじゃない変数へmove
    let th0 = thread::spawn(move || {
        // ⑥
        some_func(lock0);
    });

    // スレッド生成
    // クロージャじゃない変数へmove
    let th1 = thread::spawn(move || {
        // ⑥
        some_func(lock1);
    });

    // 待ち合わせ
    th0.join().unwrap();
    th1.join().unwrap();
}
