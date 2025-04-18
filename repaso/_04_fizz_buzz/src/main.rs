// FizzBuzz
// divisible por 3 "Fizz"
// divisible por 5 "Buzz"
// divisible por 3 y 5 "FizzBuzz"
// otro "n"
//

fn fizz_buzz() {
    let start = 1;
    let end = 100;
    let mut msg = String::new();
    for i in start..=end {
        if i % 3 == 0 && i % 5 == 0 {
            msg.push_str(&format!(" {}:FizzBuzz", i));
        } else if i % 3 == 0 {
            msg.push_str(&format!(" {}:Fizz", i));
        } else if i % 5 == 0 {
            msg.push_str(&format!(" {}:Buzz", i));
        } else {
            msg.push_str(&format!(" {}", i));
        }
    }
    println!("{}", msg);
}

fn main() {
    fizz_buzz();
}
