mod p34;

fn main() {
    let mut prod = 1;
    for index in 1..10 {
        prod *= index;
        println!("The factorial of {} is {}", index, prod);
    }
}
