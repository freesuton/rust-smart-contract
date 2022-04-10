


fn main() {
    let mut x = String::from("xxx");
    let mut s = String::from("hello world");
    let mut s1 = &mut s;
    // s1.push_str("df");
    // s1 = &mut x;
    s1 = String::from("xxx");
    // print_welcome(s1);
    println!("{}",s);
    println!("{}",s1);
    println!("{}",x);
}

// fn print_welcome(text: String){
//     println!("{}", text);
// }