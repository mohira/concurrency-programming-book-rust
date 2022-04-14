// 最適化抑制読み書き用
use std::ptr::{read_volatile, write_volatile}; // ①

// メモリバリア用
use std::sync::atomic::{fence, Ordering}; // ②
use std::thread;

const NUM_THREADS: usize = 4; // スレッド数
const NUM_LOOP: usize = 1000000; // 各スレッドでのループ数

// volatile用のマクロ ③
macro_rules! read_mem {
  ($addr: expr) => { unsafe {read_volatile($addr)}};
}

macro_rules! writer_mem {
  ($addr: expr, $val: expr) => {
      unsafe{write_volatile($addr, $val)}
  };
}

// パン屋のアルゴリズム用の型 ④
struct BakeryLock {
  entering: [bool; NUM_THREADS],
  tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
  // ロック関数。idxはスレッド番号

  //スレッド idx がチケット取得中状態であることを示すために、
  //entering[idx] を true に設定。また、その前後では、メモリバリアを行いアウトオブオーダ
  // でのメモリ読み書きが行われることを防いでいる。
  fn lock(&mut self, idx: usize) -> LockGuard {
    // ここからチケット取得処理 ⑤
    fence(Ordering::SeqCst); // ← これなんだっけ？
    writer_mem!(&mut self.entering[idx], true);
    fence(Ordering::SeqCst);

    // 現在配布されているチケットの最大値を取得 ⑥
    let mut max = 0;
    for i in 0..NUM_THREADS {
      if let Some(t) = read_mem!(&self.tickets[i]) {
        max = max.max(t);
      }
    }

    // 最大値+1 を自分のチケット番号とする ⑦
    let ticket = max + 1;
    writer_mem!(&mut self.tickets[idx], Some(ticket));

    // もうチケットGETしたぜ！！！
    fence(Ordering::SeqCst);
    writer_mem!(&mut self.entering[idx], false); // ⑧
    fence(Ordering::SeqCst);

    // ここから待機処理 ⑨
    for i in 0..NUM_THREADS {
      // iはスレッド番号
      if i == idx {
        continue;
      }

      // スレッド i が、チケット取得中なら待機 ⑩
      while read_mem!(&self.entering[i]) {}

      loop {
        // スレッドi と 自分の優先順位 を比較して
        // 「自分の方が優先順位が高い」または「スレッドiが処理中でない」場合に待機を終了
        // ⑪← 怪しい？

        match read_mem!(&self.tickets[i]) {
          // スレッドiのチケット番号を見る
          Some(t) => {
            // スレッドiのチケット番号より
            // 「自分の番号が若い」
            // または
            // 「チケット番号が同じ かつ 自分のほうがスレッド番号が若い」
            // 場合に待機終了

            if ticket < t || (ticket == t && idx < i) {
                break;
              }
          }
          None => {
            // スレッドiが処理中でない場合は待機終了
            // 「チケット持ってない 」
            // 「チケットを持っている」:=レジでの処理を待っている
            break;
          }
        }
      }
    }
    fence(Ordering::SeqCst);
    LockGuard {idx}
  }
}

// ロック管理用の型 ⑫
struct LockGuard {
  idx: usize
}


// Drop for isなに？
impl Drop for LockGuard {
  // ロック解放処理 ⑬
  fn drop(&mut self) {
    // スコープを外れたときにする挙動を定義
    fence(Ordering::SeqCst);
    writer_mem!(&mut LOCK.tickets[self.idx], None);
  }
}

// グローバル変数⑭
static mut LOCK: BakeryLock = BakeryLock {
  entering: [false; NUM_THREADS],
  tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;


fn main() {
  // NUM_THREADSだけスレッドを生成
  let mut v = Vec::new();
  for i in 0..NUM_THREADS {
    let th = thread::spawn(move || {
      // NUM_LOOPだけループし、COUNTをインクリメント
      for _ in 0..NUM_LOOP {
        // ロック獲得     
        let _lock = unsafe {LOCK.lock(i)};

        unsafe {
          let c = read_volatile(&COUNT);
          write_volatile(&mut COUNT, c + 1);
        }
      }
    });

    v.push(th);
  }

  for th in v {
    th.join().unwrap();


    println!(
      "COUNT={} (expected={})",
      unsafe { COUNT },
      NUM_LOOP * NUM_THREADS
    )
  }
}

