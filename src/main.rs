use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8; // スレッドは8個！
const SEM_NUM: isize = 4; // 4つまではいける
// したがって、 wait と post の間は 必ず 4 スレッド以内に制限されるはずである。

static mut CNT: AtomicUsize = AtomicUsize::new(0);


// セマフォ用の型 1
pub struct Semaphore{
  mutex: Mutex<isize>,
  cond: Condvar,
  max: isize
}

impl Semaphore {
    pub fn new(max: isize) -> Self { // 2
        Semaphore {
          mutex: Mutex::new(0),
          cond: Condvar::new(),
          max,
        }
    }

    pub fn wait(&self) {
      // カウントが最大値以上なら待機 3
      let mut cnt = self.mutex.lock().unwrap();

      // 既にmax以上の数でプロセスが実行されている状態だったら
      while *cnt >= self.max {
          // アトミック
          cnt = self.cond.wait(cnt).unwrap();
      }

      *cnt += 1;  // 4
    }

    pub fn post(&self) {
      // カウントをデクリメント  4
      let mut cnt = self.mutex.lock().unwrap();
      *cnt -= 1;

      // どれかのプロセスが終わる（＝ notify_one される）
      if *cnt <= self.max {
        self.cond.notify_one();
      }
    }
}


fn main() {
  let  mut v = Vec::new();

  // SEM_NUM だけ同時実行可能なセマフォ
  let sem = Arc::new(Semaphore::new(SEM_NUM));

  for i in 0..NUM_THREADS {
    let s = sem.clone();

    let t = std::thread::spawn(move || {
      for l in 0..NUM_LOOP {
        s.wait();

        // アトミックにインクリメント
        unsafe {CNT.fetch_add(1, Ordering::SeqCst)};

        let n = unsafe { CNT.load(Ordering::SeqCst) };

        println!("セマフォ: スレッド = {}, ループ = {} CNT = {}", i, l, n);

        assert!((n as isize) <= SEM_NUM);

        // アトミックにデクリメント
        unsafe {CNT.fetch_sub(1, Ordering::SeqCst)};

        s.post();
      }
    });

    v.push(t);
  }

  for t in v {
    t.join().unwrap();
  }


}




