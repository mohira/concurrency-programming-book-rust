use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    // スレッドハンドラを保存するベクタ
    let mut v = Vec::new(); // 2

    // 10スレッド分のバリア同期をArcで包む
    let barrier = Arc::new(Barrier::new(999));

    // 10スレッド起動
    for i in 0..10 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            println!("はい、バリアーーー {}", i );
            b.wait(); // バリア同期
            println!("finished barrier {}", i );
        });

        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }

}
