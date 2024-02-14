fn main() {
    println!("Hello, world!");
    println!("{}", fibonacci(100000));

    let name = "myname".to_string();
    say_howdy(&name);
    say_goodby(&name);

}

// create a function that return fibonacci sequence
fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn say_howdy(name: &str){
    println!("Howdy, {}", name);
}

fn say_goodby(name: &str){
    println!("Goodby, {}", name);
}
