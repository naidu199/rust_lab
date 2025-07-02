fn main() {
   
    let a: i32 = 2;
    let b: i32 = 3;

    
    let result = a.pow(3) + 3 * a.pow(2) * b + 3 * a * b.pow(2) + b.pow(3);

    println!(" ({} + {})^3 is: {}", a, b, result);
}
