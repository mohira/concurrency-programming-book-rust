use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn child(id: u64, p: Arc<(Mutex<u64>, Condvar)>) {
  let &(ref lock, ref cvar) = &*p;

  let mut started = lock.lock().unwrap();

  while *started < 5 {
    println!("child {} standby, started = {}", id, started);
    started = cvar.wait(started).unwrap();
    println!("child {} notififed, started = {}", id, started);
  }

  println!("child {}", id);
}

fn parent(p: Arc<(Mutex<u64>, Condvar)>) {
  let &(ref lock, ref cvar) = &*p;

  let mut started = lock.lock().unwrap();
  *started += 1;
  cvar.notify_all();

  println!("parent");
}

fn main() {
  let pair0 = Arc::new((Mutex::new(0), Condvar::new()));
  let pair1 = pair0.clone();
  let pair2 = pair0.clone();
  let pair3 = pair0.clone();
  let pair4 = pair0.clone();
  let pair5 = pair0.clone();
  let pair6 = pair0.clone();

  let c0 = thread::spawn(move || { child(0, pair0) });
  let c1 = thread::spawn(move || { child(1, pair1) });

  let p = thread::spawn(move || { parent(pair2) });
  let p1 = thread::spawn(move || { parent(pair3) });
  let p2 = thread::spawn(move || { parent(pair4) });
  let p3 = thread::spawn(move || { parent(pair5) });
  let p4 = thread::spawn(move || { parent(pair6) });

  c0.join().unwrap();
  c1.join().unwrap();

  p.join().unwrap();
  p1.join().unwrap();
  p2.join().unwrap();
  p3.join().unwrap();
  p4.join().unwrap();
}