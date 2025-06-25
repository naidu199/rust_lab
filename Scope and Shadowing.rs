fn main() {
    let x = 10;
    
    println!("Before shadowing: x = {}", x);
    
    {
        let x = 20;
        println!("Inside inner scope: x = {}", x);
    }

    println!("After inner scope: x = {}", x);

    let x = 30;
    println!("After shadowing in main scope: x = {}", x);
}
