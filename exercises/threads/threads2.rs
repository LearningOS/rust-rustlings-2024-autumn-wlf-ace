// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 和 Mutex 来保护共享状态
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 在更新之前获取锁
            let mut status = status_shared.lock().unwrap(); // 错误出现在这里
            status.jobs_completed += 1; // 更新共享值
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // 等待所有线程完成
    }

    // 在所有线程完成后，打印 jobs_completed 的值
    let final_status = status.lock().unwrap(); // 获取锁以读取值
    println!("jobs completed {}", final_status.jobs_completed);
}
