use std::collections::HashMap;
fn main() {
    // let s1 = String::from("hello");
    // // let s2=String::from("wold");
    // let s = &s1[0..2];
    // println!("{}", s);
    // for i in s1.chars() {
    //     println!("{:?}", i);
    // }
    find_mode();
}
fn find_mode() {
    let data: Vec<i32> = vec![
        7, 1, 2, 2, 4, 7, 7, 7, 77, 7, 1, 2, 23, 435, 36, 7, 576, 8, 34, 21, 2,8,1,1,1,1,1,1,1,1
    ];
    let mut mode_data: HashMap<i32, i32> = HashMap::new();
    let mut mode: i32 = 0;
    for v in data {
        let count = mode_data.entry(v).or_insert(0);
        *count += 1;
        if mode_data.get(&v).copied().unwrap_or(0) > mode {
            mode = v;
        }
    }
    // for (key, v) in &mode_data {
    //     if mode_data.get(&mode).copied().unwrap_or(0) < *v {
    //         mode = *key;
    //     }
    // }
    println!("mode is :{}", mode);
}
fn _hash_map() {
    // let mut score=HashMap::new();
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // let team = String::from("blue");
    // let blue_score = scores.get(&team).copied().unwrap_or(0);
    // println!("{:#?}-{}", scores, blue_score);
    // let data=scores.entry(String::from("blue1")).or_insert(40);
    // println!("insert if not exists:{}",data);
    // *data=90;
    // println!("value change:{}",data);
    // for (key, value) in scores {
    //     println!("key:{} value:{}", key, value);
    // }
    let text = "hello world wonderful world";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1
    }
    println!("word count:{word_count:?}");
}
