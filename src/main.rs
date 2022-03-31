use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

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


// 送信側のための型 1
#[derive(Clone)]
pub struct Sender<T> {
  sem: Arc<Semaphore>, // 有限性を実現するセマフォ
  buf: Arc<Mutex<LinkedList<T>>, // キュー
  cond: Arc<Condvar>, // 読み込み側の条件変数
}


impl<T: Send> Sender<T> { // 2
    // 送信関数
    pub fn send(&self, data: T) {
      self.sem.wait(); // キューの最大値に到達したら待機 3

      let mut buf = self.buf.lock().unwrap();

      buf.push_back(data); // エンキュー

      self.cond.notify_one(); // 読み込み

    }
}


pub struct Receiver<T> { // ①
  sem: Arc<Semaphore>, // 有限性を実現するセマフォ
  buf: Arc<Mutex<LinkedList<T>>>, // キュー
  cond: Arc<Condvar>, // 読み込み側の条件変数
}

impl<T> Receiver<T> {
  pub fn recv(&self) -> T {
    let mut buf = self.buf.lock().unwrap();

    loop {
      // キューから取り出し ②
      if let Some(data) = buf.pop_front() {
        self.sem.post(); // ③
        return data;
      }

      // 空の場合待機 ④
      buf = self.cond.wait(buf).unwrap();
    }
  }
    
}











fn main() {

}




