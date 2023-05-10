use std::io;
fn main() {
    println!("Enter an integer to generate the nth Fibonacci number: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("failed to read input");
    let number : f32 = number.trim().parse().expect("enter a number");

    let fib = phi().powf(number) - neg_phi().powf(number) ;

    println!("The {number}th of Fibonacci's series is {}", fib/root_five());


}

fn phi() -> f32 {
    1.6180339
}
fn root_five() -> f32 {
    2.236067977
}
fn neg_phi() -> f32 {
    -0.6180339
}