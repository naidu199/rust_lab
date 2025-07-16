fn main() {
    let number = 5;
    'outer: for i in 1..=10 {
        println!("{} x {} = {}", number, i, number * i);
    }
}
