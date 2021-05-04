fn main() {
    let number = 40;

    println!("FIBO: {}", fibo(number))
}

fn fibo(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    return fibo(n - 1) + fibo(n - 2);
}