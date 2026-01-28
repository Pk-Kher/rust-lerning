fn main() {
    let mut buf = vec![9; 500999999];
    // let padding = vec![0; (8 - (buf.len() % 8)) % 8];
    // buf.extend(padding);
    let padding = (8 - (buf.len() % 8)) % 8;
    for _ in 0..padding {
        buf.push(0);
    }
}
