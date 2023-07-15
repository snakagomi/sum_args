fn main() {
    let args = std::env::args();
    let mut total = 0.0;
    for (i, v) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let num: f64 = match v.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }
    println!("sum: {}", total);
}
