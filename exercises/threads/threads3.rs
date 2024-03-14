// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 通过使用 Arc<Mutex<mpsc::Sender<u32>>> ，你可以确保在多个线程之间安全地共享 Sender ，避免了所有权问题，并保证了线程安全。
fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // 创建了一个mpsc通道
    let (tx, rx) = mpsc::channel();
    // 实例化了Queue结构体
    let queue = Queue::new();
    let queue_length = queue.length;
    let tx_arc = Arc::new(Mutex::new(tx));

    // 调用send_tx函数将队列中的值发送到通道中
    send_tx(queue, tx_arc);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    // 通过接收通道中的值并计算总接收数来验证接收的值数量与队列长度是否相等
    assert_eq!(total_received, queue_length)
}

/*
Mutex是一种在多线程编程中用于保护共享数据访问的同步原语。在Rust中，Mutex是一个提供了内部可变性的类型，它允许在多个线程之间安全地共享数据。Mutex的全称是Mutual Exclusion（互斥），它确保在任意时刻只有一个线程能够访问被Mutex保护的数据，从而避免竞争条件和数据竞争。

在Rust中，Mutex通常与Arc（原子引用计数）结合使用，以在多个线程间共享Mutex。Arc允许多个线程拥有对共享数据的所有权，而Mutex确保在任意时刻只有一个线程能够访问数据。

Mutex的基本用法包括使用 `Mutex::new` 创建一个新的Mutex实例，然后通过调用 `lock` 方法来获取Mutex的锁，并在锁定的作用域中访问被保护的数据。当作用域结束时，锁会自动释放。

示例：
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    } // Mutex在这里自动释放锁

    println!("Final counter value: {:?}", counter.lock().unwrap());
}
在上面的示例中，我们创建了一个Mutex来保护一个计数器变量。在作用域中，我们通过调用 `lock` 方法获取Mutex的锁，然后对计数器进行操作。一旦作用域结束，Mutex会自动释放锁。
*/
