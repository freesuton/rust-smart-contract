use std::f64;



enum Token {
    Atomic(String),
    Minted(String,String)
}

fn price_oracle(t: &Token) -> f64 {
    match t{
        Token::Atomic(_) => 1.0,
        Token::Minted(_, _) => {
            2.0
        }
    }
}

fn SFr0(v0:f64,v1:f64,r0:f64,r1:f64) -> f64{
    let variable0:f64 = v0*v0*v1*v1 + 4.0*v0*v1*r0*r1;
    let res:f64 = (variable0.sqrt() - v0*v1)/(2.0*v1);
    res
}

fn SFr1(r0:f64,r1:f64,sfr0: f64) -> f64 {
    r0*r1/sfr0
}




fn main() {
    let token0 = Token::Atomic(String::from("sf"));
    let token1 = Token::Minted(String::from("sf"),String::from("xx"));
    let p:f64 = price_oracle(&token1);
    let p2:f64 = 8.0;
    // let p3:u64 = Sqrt(8);
    let p3:f64 = p2.sqrt();
    let sf0:f64 = SFr0(20.0, 15.0, 100.0, 100.0);
    let sf1:f64 = SFr1(100.0, 100.0, sf0);
    println!("dfgdfgfggdfgdfg- {},{} : ", sf0,sf1);
}

