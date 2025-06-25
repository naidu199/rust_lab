fn main() {
    for i in 1..=5 {
        // Create a string with the number i repeated i times
        let pattern = i.to_string().repeat(i);
        
        // Print the pattern using a placeholder
        println!("{}", pattern);
    }
}
