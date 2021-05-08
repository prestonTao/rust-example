// 实现一个简单的自旋锁（spinlock）
   
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};

pub fn run() {
   let spinlock = Arc::new(AtomicUsize::new(1));

   let spinlock_clone = Arc::clone(&spinlock);
   let thread = thread::spawn(move|| {
       // lock
       spinlock_clone.store(1, Ordering::SeqCst);
       // do something
       let t = time::Duration::from_secs(300);
       std::thread::sleep(t);
       // unlock
       spinlock_clone.store(0, Ordering::SeqCst);
   });

   // Wait for the other thread to release the lock
   println!("等待其他线程释放锁");
   while spinlock.load(Ordering::SeqCst) != 0 {}

   if let Err(panic) = thread.join() {
       println!("Thread had an error: {:?}", panic);
   }
}