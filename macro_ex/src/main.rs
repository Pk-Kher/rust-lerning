macro_rules! three {
    () => {4};
    () => {{
        let local = 0;
        3
    }};
}

fn main() {
    let data = three!();
    println!("{}", data);
}
