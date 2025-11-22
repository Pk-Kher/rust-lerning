#![allow(unused)]
use std::{
    cmp,
    error::Error,
    future::Future,
    io,
    pin::{Pin, pin},
    thread::{self, current, sleep},
    time::{Duration, Instant},
};
use tokio::fs;
use trpl::{Either, Html, ReceiverStream, Stream, StreamExt};

// trait Hello {
//     type data;
//     fn show_data() -> Self::data;
// }
//
// struct Person;
//
// impl Hello for Person {
//     type data = String;
//     fn show_data() -> Self::data {
//         String::from("hello")
//     }
// }
//
// fn use_hello_traits() {
//     let data = Person::show_data();
//     println!("{}",data);
// }

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // trpl::run(async {
    //     let first_site_title = page_title(&args[1]);
    //     let second_site_title = page_title(&args[2]);
    //     let data = match trpl::race(first_site_title, second_site_title).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };
    //     println!("{:#?}", data);
    // if let Some(title) = second_site_title {
    //     println!("{}", title);
    // } else {
    //     println!("there is no title");
    // }
    // if let Some(title) = first_site_title {
    //     println!("{}", title);
    // } else {
    //     println!("there is no title");
    // }
    // });
    // trpl::run(async {
    //     let file_content = read_file_content(String::from("Cargo.toml"));
    //     let file_content1= read_file_content(String::from("Cargo.toml"));
    //     trpl::join(file_content,file_content1).await;
    // });
    // yilding_control();
    // stream_future();
    without_pin_error();
}

fn stream_future() {
    trpl::run(async {
        // step 1;
        // let values = vec![1, 2, 3, 4, 5, 6, 7];
        // let iter = values.iter().map(|val| val * 2);
        // let mut stream = trpl::stream_from_iter(iter);
        // while let Some(data) = stream.next().await {
        //     println!("{data:?}");
        // }
        // step2
        // let mut messages = get_messages();
        // while let Some(data) = messages.next().await {
        //     println!("{data}");
        // }
        // step3
        // let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        // while let Some(result) = messages.next().await {
        //     match result {
        //         Ok(message) => println!("{message}"),
        //         Err(reason) => eprintln!("Problem: {reason:?}"),
        //     }
        // }
        //step 4
        // let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        let mut messages = get_messages().timeout(Duration::from_millis(200));
        // let interval = get_interval();
        let interval = get_interval()
            .map(|count| format!("Interval :{count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_millis(10));
        let merged = messages.merge(interval).take(20);
        let mut stream = pin!(merged);
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
    fn get_messages() -> impl Stream<Item = String> {
        // let messages = ["a", "b", "c"];
        // let (tx, rx) = trpl::channel();
        // for message in messages {
        //     tx.send(format!("message :{message}"));
        // }
        // ReceiverStream::new(rx)
        // step 3
        let (tx, rx) = trpl::channel();
        trpl::spawn_task(async move {
            let messages = ["a", "b", "c"];
            for message in messages {
                trpl::sleep(Duration::from_millis(400)).await;
                if let Err(error) = tx.send(format!("message :{message}")) {
                    eprintln!("can not send message '{message}' : {error}");
                    break;
                };
            }
        });
        ReceiverStream::new(rx)
    }
    fn get_interval() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();
        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;
                if let Err(error) = tx.send(count) {
                    eprintln!("can not send interval count {count} : {error}");
                    break;
                }
            }
        });
        ReceiverStream::new(rx)
    }
}

fn _interval_merge() {
    let mut intervals: Vec<Vec<u8>> =
        vec![vec![7, 7], vec![1, 3], vec![1, 2], vec![5, 6], vec![6, 9]];
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    println!("initials: {:?}", intervals);
    let mut result = vec![intervals[0].clone()];
    for i in 1..intervals.len() {
        let result_len = result.len();
        let mut last = &mut result[result_len - 1];
        let current = &intervals[i];
        if current[0] <= last[1] {
            last[1] = cmp::max(last[1], current[1]);
        } else {
            result.push(current.clone());
        }
    }
    println!("after that: {:?}", result);
}
async fn read_file_content(name: String) -> Result<String, io::Error> {
    let data = fs::read_to_string(name).await?;
    println!("{}", data);
    Ok(data)
}
async fn yilding_control() {
    fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("{name} ran for {ms}");
    }
    // trpl::run(async {
    //  step one 1
    // let a = async {
    //     println!("a is started");
    //     slow("a", 30);
    //     slow("a", 20);
    //     slow("a", 10);
    //     trpl::sleep(Duration::from_millis(50)).await;
    //     println!("a is finish working");
    // };
    // let b = async {
    //     println!("b is started");
    //     slow("b", 30);
    //     slow("b", 20);
    //     slow("b", 10);
    //     trpl::sleep(Duration::from_millis(50)).await;
    //     println!("b is finish working");
    // };
    // trpl::race(a, b).await;
    //
    // step 2
    // let one_ms = Duration::from_millis(1);
    // let a = async {
    //     println!("a is started");
    //     slow("a", 30);
    //     trpl::sleep(one_ms).await;
    //     slow("a", 20);
    //     trpl::sleep(one_ms).await;
    //     slow("a", 10);
    //     trpl::sleep(one_ms).await;
    //     println!("a is finish working");
    // };
    // let b = async {
    //     println!("b is started");
    //     slow("b", 30);
    //     trpl::sleep(one_ms).await;
    //     slow("b", 20);
    //     trpl::sleep(one_ms).await;
    //     slow("b", 10);
    //     trpl::sleep(one_ms).await;
    //     println!("b is finish working");
    // };
    // trpl::race(a, b).await;
    //
    // // step 3
    // let a = async {
    //     println!("a is started");
    //     slow("a", 30);
    //     trpl::yield_now().await;
    //     slow("a", 20);
    //     trpl::yield_now().await;
    //     slow("a", 10);
    //     trpl::yield_now().await;
    //     println!("a is finish working");
    // };
    // let b = async {
    //     println!("b is started");
    //     slow("b", 30);
    //     trpl::yield_now().await;
    //     slow("b", 20);
    //     trpl::yield_now().await;
    //     slow("b", 10);
    //     trpl::yield_now().await;
    //     println!("b is finish working");
    // };
    // trpl::race(a, b).await;
    // step 4 test speed switch task using sleep and yield
    let one_nano = Duration::from_nanos(0);
    let start = Instant::now();
    async {
        for _ in 0..1000 {
            trpl::sleep(one_nano).await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!("sleep finish in the time: {}", time.as_secs_f32());

    let start = Instant::now();
    async {
        for _ in 0..1000 {
            trpl::yield_now().await;
        }
    }
    .await;

    println!(
        "yield now finish in the time : {}",
        (Instant::now() - start).as_secs_f32()
    );
    // });
}

fn _message_passing() {
    trpl::run(async {
        //step 1
        // let (tx, mut rx): (trpl::Sender<String>, trpl::Receiver<String>) = trpl::channel();
        //
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        //
        // let receive = rx.recv().await.unwrap();
        // println!("message :{}", receive);
        //
        // step2
        // let (tx, mut rx) = trpl::channel();
        // let vals = vec![String::from("hello"), String::from("hi")];
        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(5000)).await;
        // }
        //
        // while let Some(data) = rx.recv().await {
        //     println!("message : {}", data);
        // }
        //
        // step3
        // let (tx, mut rx) = trpl::channel();
        // let fut1 = async move {
        //     let vals = vec![String::from("hello"), String::from("hi")];
        //     for val in vals {
        //         tx.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(5000)).await;
        //     }
        // };
        // let fut2 = async {
        //     while let Some(data) = rx.recv().await {
        //         println!("message : {}", data);
        //     }
        // };
        // trpl::join(fut1, fut2).await;
        // step 4
        // let (tx, mut rx) = trpl::channel();
        // let tx_clone = tx.clone();
        // let fut1 = async move {
        //     let vals = vec![String::from("hello"), String::from("hi")];
        //     for val in vals {
        //         tx_clone.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(5000)).await;
        //     }
        // };
        // let fut2 = async move {
        //     let vals = vec![String::from("hello"), String::from("hi")];
        //     for val in vals {
        //         tx.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(5000)).await;
        //     }
        // };
        // let fut3 = async move {
        //     while let Some(data) = rx.recv().await {
        //         println!("message : {}", data);
        //     }
        // };
        // // trpl::join!(fut1, fut2, fut3);
        // //but now we dont know how many future we will get so we are useing vec!
        // //
        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        //     vec![Box::pin(fut1), Box::pin(fut2), Box::pin(fut3)];
        // trpl::join_all(futures).await;
        // step remove heap allocation from box
        let (tx, mut rx) = trpl::channel();
        let tx_clone = tx.clone();
        let fut1 = pin!(async move {
            let vals = vec![String::from("hello"), String::from("hi")];
            for val in vals {
                tx_clone.send(val).unwrap();
                trpl::sleep(Duration::from_millis(5000)).await;
            }
        });
        let fut2 = pin!(async move {
            let vals = vec![String::from("hello"), String::from("hi")];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(5000)).await;
            }
        });
        let fut3 = pin!(async move {
            while let Some(data) = rx.recv().await {
                println!("message : {}", data);
            }
        });
        // trpl::join!(fut1, fut2, fut3);
        //but now we dont know how many future we will get so we are useing vec!
        //
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![fut1, fut2, fut3];
        trpl::join_all(futures).await;
    });
}
fn _concurrency_async1() {
    trpl::run(async {
        let fut1 = async {
            for i in 0..10 {
                println!("thread {i} ");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 0..5 {
                println!(" out side thread {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await;
    });
}
fn _concurrency_async() {
    trpl::run(async {
        // trpl::spawn_task(async {
        //     for i in 0..10 {
        //         println!("thread {i} ");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });
        let handle = trpl::spawn_task(async {
            for i in 0..10 {
                println!("thread {i} ");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 0..5 {
            println!(" out side thread {i}");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        handle.await.unwrap();
    });
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;

    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn without_pin_error() {
    // #[derive(Debug)]
    // struct Bad<'a> {
    //     ref_val: &'a i32,
    //     value: i32,
    // }
    //
    // impl<'a> Bad<'a> {
    //     fn new(value: i32, ref_value: &'a i32) -> Self {
    //         Bad {
    //             ref_val: ref_value,
    //             value,
    //         }
    //     }
    // }
    #[derive(Debug)]
    struct Bad {
        self_ref: *mut i32,
        value: i32,
    }

    // !Unpin by default because it contains a raw pointer

    impl Bad {
        fn new(v: i32) -> Self {
            let mut tmp = Bad {
                self_ref: std::ptr::null_mut(),
                value: v,
            };
            tmp.self_ref = &mut tmp.value;
            // println!("{:p} {:p}", &tmp.value, tmp.self_ref);
            tmp
        }
    }
    let value = 12;
    let mut data = Bad::new(10);
    println!("data value addr = {:p} value = {}", &data.value, data.value);
    println!("data ptr   addr = {:p} value = {}", data.self_ref, unsafe {
        *data.self_ref
    });
    unsafe {
        *data.self_ref = 9999;
    };
    println!("data after change value using address");
    println!("data value addr = {:p} value = {}", &data.value, data.value);
    println!("data ptr   addr = {:p} value = {}", data.self_ref, unsafe {
        *data.self_ref
    });
    let data2 = data;
    println!("value addr = {:p}", &data2.value);
    println!("ptr   addr = {:p}", data2.self_ref);
    println!("{:#?}", unsafe { *data2.self_ref });
}
