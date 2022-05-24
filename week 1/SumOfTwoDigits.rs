fn main() {

    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff);
    let mut input = buff.split_whitespace();

    let a: i64 = input
    .next()
    .unwrap()
    .parse()
    .unwrap();
    
    let b: i64 = input
    .next()
    .unwrap()
    .parse()
    .unwrap();

    let result = a + b;
    println!("{}", result);
}
