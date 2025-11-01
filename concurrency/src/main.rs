use std::{
    rc::Rc,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    // thread_ex();
    // channel_shared_data();
    shared_data_using_mutex();
}

fn _thread_ex() {
    let data = vec![1, 2, 3, 4, 5];
    let called_thread = thread::spawn(move || {
        println!("{data:?}");
        // for i in &data {
        //     println!("sub thread {}", i);
        //     thread::sleep(Duration::from_millis(1));
        // }
    });
    // for i in 1..5 {
    //     println!("out thread {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    called_thread.join().unwrap();
    // let mut n = 1;
    // let t = thread::spawn(move || {
    //     n = n + 1;
    //     thread::spawn(move || {
    //         n = n + 1;
    //     })
    // });
    // n = n + 1;
    // t.join().unwrap().join().unwrap();
    // println!("{n}");
}

fn shared_data_using_mutex() {
    let data = Mutex::new(23);
    {
        let mut num = data.lock().unwrap();
        *num = 6;
    }
    let d = data.lock().unwrap();
    println!("{:?}", d);
    drop(d); // if you remove this then it will be the deadlock
    let mut p = data.lock().unwrap();
    *p = 10;
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut data = counter.lock().unwrap();
            *data += 1;
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    let data = thread::spawn(|| String::from("hello"));
    let a = data.join().unwrap();
    let s = String::from("Hello world");

    let a = Arc::new(&s);
    println!("counter {}", *counter.lock().unwrap());
}

fn _channel_shared_data() {
    let (tx, rc) = mpsc::channel();
    thread::spawn(move || {
        let string = String::from("hell");
        thread::sleep(Duration::from_millis(2000));
        tx.send(string.clone()).unwrap();
        let data = tx.send("34".to_string());
        match data {
            Ok(_) => println!("send"),
            Err(_) => println!("error"),
        }
        thread::sleep(Duration::from_millis(2000));
    });

    let received = rc.recv().unwrap();
    let received1 = rc.recv().unwrap();
    println!("{} {}", received, received1);

    let (tx2, rc2) = mpsc::channel();
    let tx_clone1 = tx2.clone();
    thread::spawn(move || {
        println!("thread called");
        loop {
            tx_clone1.send("hello".to_string()).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });
    let tx_clone2 = tx2.clone();
    thread::spawn(move || {
        println!("thread called");
        loop {
            tx_clone2.send("pk".to_string()).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });
    for rec in rc2 {
        println!("received : {}", rec);
    }
    // loop{
    //     match rc2.try_recv(){
    //         Err(_)=>{},
    //         Ok(data)=>println!("received : {}",data),
    //         // Err(mpsc::TryRecvError::Empty)=>println!("no message yet"),
    //         // Err(mpsc::TryRecvError::Disconnected)=>println!("Disconnected"),
    //
    //     }
    // }
}
