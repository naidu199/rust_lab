fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("2nd and 3rd elements: {:?}", &arr[1..3]);

    println!("Omit start index (upto 5th): {:?}", &arr[..5]);

    println!("Omit end index (from 5th): {:?}", &arr[5..]);

    println!("Whole array using slice: {:?}", &arr[..]);
}
