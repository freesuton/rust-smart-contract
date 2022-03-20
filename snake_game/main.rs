


fn main() {
    let message = "Hello world";
    print_welcome(message);
}

fn print_welcome(text: &str) -> &str{
    println!("{}", text);
    "Hi there"
}