use std::fmt;
use crate::TransitionError::{InsufficientBalance, InsufficientReserves};


#[derive(PartialEq, PartialOrd, Eq, Clone)]
struct User {
    name: String
}

impl User {
    fn new(name: &str) -> Self {
        User {
            name: String::from(name)
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
enum Token {
    Atomic(String),
    Minted(String,String)
}

impl Token {
    fn mint(token0: &Token, token1: &Token) -> Token {
        if let Token::Atomic(t0) = token0 {
            if let Token::Atomic(t1) = token1 {
                return Token::Minted(t0.clone(), t1.clone());
            }
        }
        panic!("invalid token pair mint");
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Atomic(d) => write!(f, "{}", d),
            Token::Minted(d0, d1) => write!(f, "{}+{}", d0, d1)
        }
    }
}

#[derive(Clone)]
struct Balance {
    token: Token,
    value: f64
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}:{}", self.value, self.token)
    }
}

#[derive(Clone)]
struct Wallet {
    user: User,
    balances: Vec<Balance>
}

impl Wallet {
    fn new(user: &User) -> Self {
        Wallet {
            user: user.clone(),
            balances: Vec::new()
        }
    }
}

impl Wallet {
    fn get_balance(&self, token: &Token) -> f64 {
        self.balances.iter().find(|b| &b.token == token)
            .map_or(0.0, |b| b.value)
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.user, self.balances.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

#[derive(Clone)]
struct AMM {
    r0: f64,
    t0: Token,
    r1: f64,
    t1: Token
}


impl AMM {
    fn new(r0: f64, t0: &Token, r1: f64, t1: &Token) -> Self {
        assert!(t0 < t1);
        AMM {
            r0,
            t0: t0.clone(), r1, t1: t1.clone(),
        }
    }

    fn get_reserves(&self, t: &Token) -> f64 {
        if &self.t0 == t {
            self.r0
        } else if &self.t1 == t {
            self.r1
        } else {
            0.0
        }
    }
}

impl fmt::Display for AMM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:.1}:{} {:.1}:{}}}", self.r0, self.t0, self.r1, self.t1)
    }
}

#[derive(Clone)]
struct State {
    wallets: Vec<Wallet>,
    amms:  Vec<AMM>
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elms =
            [self.wallets.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            self.amms.iter().map(|x| x.to_string()).collect::<Vec<_>>()].concat();

        write!(f, "{}", elms.join(" | "))
    }
}

fn order_tokens<'a>( t0: &'a Token, t1: &'a Token) -> (&'a Token, &'a Token) {
    assert!(t0 != t1);
    if t0 < t1 {
        return (t0, t1);
    } else {
        return (t1, t0);
    }
}

fn order_token_reserves<'a>( t0: &'a Token, r0: f64, t1: &'a Token, r1: f64) -> (&'a Token, f64, &'a Token, f64) {
    assert!(t0 != t1);
    if t0 < t1 {
        return (t0, r0, t1, r1);
    } else {
        return (t1, r1, t0, r0);
    }
}

impl State {

    fn new() -> Self {
        State {
            wallets: Vec::new(),
            amms: Vec::new(),
        }
    }

    //Token supply. We define the supply of a token type τ in a state Γ as the sum of the 
    //reserves of τ in all the wallets and the AMMs occurring in Γ. 
    fn token_supply(&self, token: &Token) -> f64 {
        //UNIMPLEMENTED
        let mut total:f64 = 0.0;
        for amm in &self.amms{
            total = total + amm.get_reserves(&token);
        }
        for wallet in &self.wallets{
            total = total + wallet.get_balance(&token);
        }
        return total
    }


    fn get_amm(&self, t0: &Token, t1: &Token) -> Option<&AMM> {
        //UNIMPLEMENTED
        for amm in &self.amms {
            if amm.t0 == *t0 && amm.t1 == *t1 || amm.t0 == *t1 && amm.t1 == *t0{
                return Some(amm)
            }
        }
        return None
    }

    fn get_reserves(&self, t: &Token, tother: &Token) -> f64 {

        let reserve = match self.get_amm(t,tother) {
            Some(amm) => {
                if amm.t0 == *t{
                return amm.r0
                }else{
                    return amm.r1
                }
            },

            None => return 0.0
        };
        // return reserve
    }

    fn set_reserve(&mut self, t0: &Token, r0: f64, t1: &Token, r1: f64) {
        //UNIMPLEMENTED
        let mut i = 0;
        for amm in &self.amms {
            if amm.t0 == *t0 && amm.t1 == *t1 || amm.t0 == *t1 && amm.t1 == *t0{
                break
            }
            i = i + 1;
        }

    
        if i >= self.amms.len(){
            let new_amm = AMM {
                r0: r0,
                t0: t0.clone(),
                r1: r1,
                t1: t1.clone()
            };
            self.amms.push(new_amm);
        }else if self.amms[i].t0 == *t0 {
            self.amms[i].r0 = r0;
            self.amms[i].r1 = r1;
        }else{
            self.amms[i].r0 = r1;
            self.amms[i].r1 = r0;
        }
        

    }

  
    fn get_balance(&self, user: &User, token:  &Token) -> f64 {
        //UNIMPLEMENTED
        let mut i = 0;
        for wallet in &self.wallets {
            if wallet.user == *user {
                break
            }
            i = i + 1;
        }
        let mut j = 0;
        for balance in &self.wallets[i].balances {
            if balance.token == *token{
                break
            }
            j = j + 1;
            if j == self.wallets[i].balances.len(){
                return 0.0
            }
        }
        
        return self.wallets[i].balances[j].value;
        // 0
    }

    fn set_balance(&mut self, user: &User, token:  &Token, new_value: f64) {
        //UNIMPLEMENTED
        let mut i = 0;
        for wallet in &self.wallets {
            if wallet.user == *user {
                break
            }
            i = i + 1;
        }

        if i >= self.wallets.len() {

            let new_wallet = Wallet {
                user: user.clone(),
                balances: Vec::new()
            };
            self.wallets.push(new_wallet);
        }

        let mut j = 0;
        for balance in &self.wallets[i].balances {
            if balance.token == *token {
                break
            }
            j = j + 1;
        }

        if j >= self.wallets[i].balances.len(){
            let new_balance = Balance {
                token: token.clone(),
                value: new_value
            };
            self.wallets[i].balances.push(new_balance);
        }else{
            self.wallets[i].balances[j].value = new_value;
        }

    }

    fn net_wealth_user(&self,user: &User, f: &dyn Fn(&State, &Token)-> f64) -> f64{
        
        let t0 = Token::Atomic(String::from("t0"));
        let t1 = Token::Atomic(String::from("t1"));
        let minted_token = Token::Minted(String::from("t0"),String::from("t1"));
        let t0_balance = self.get_balance(user,&t0);
        let t1_balance = self.get_balance(user,&t1);
        let minted_balance = self.get_balance(user, &minted_token);
        f(&self,&t0)*t0_balance + f(&self,&t1)*t1_balance +f(&self,&minted_token)*minted_balance
    }

    fn net_wealth(&self, f: &dyn Fn(&State,&Token) -> f64  ) -> f64{
        let t0 = Token::Atomic(String::from("t0"));
        let t1 = Token::Atomic(String::from("t1"));
        let minted_token = Token::Minted(String::from("t0"),String::from("t1"));

        let mut sum:f64 = 0.0;
        let mut i:u32 = 0;
        for wallet in &self.wallets {
            for balance in &wallet.balances {
                let t_value = f(&self,&balance.token)*balance.value;
                sum += t_value;
            }
        }
        sum
    }



}

#[derive(Debug)]
enum TransitionError {
    InsufficientBalance,
    InvalidDepositRatio,
    InsufficientReserves,
    Unimplemented
}


trait Transition {
    fn apply(&self, s0: &State) -> Result<State, TransitionError>;
}

struct Deposit {
    sender: User,
    v0: f64,
    t0: Token,
    v1: f64,
    t1: Token
}

impl Deposit  {
    fn new(sender: &User, r0: f64, t0: &Token, r1: f64, t1: &Token) -> Self {
        assert!(r0 > 0.0 && r1 > 0.0);
        Deposit {
            sender: sender.clone(),
            v0: r0,
            t0: t0.clone(),
            v1: r1,
            t1: t1.clone(),
        }
    }
}

impl Transition for Deposit {
    fn apply(&self, pre: &State) -> Result<State, TransitionError> {
        //UNIMPLEMENTED
        let mut post = pre.clone();

        let t0_balance:f64 = post.get_balance(&self.sender, &self.t0);
        let t1_balance:f64 = post.get_balance(&self.sender, &self.t1);
        let t0_reserve:f64 = post.get_reserves(&self.t0,&self.t1);
        let t1_reserve:f64 = post.get_reserves(&self.t1,&self.t0);

        post.set_balance(&self.sender,&self.t0, t0_balance - &self.v0);
        post.set_balance(&self.sender,&self.t1, t1_balance - &self.v1);
        post.set_reserve(&self.t0,t0_reserve+&self.v0,&self.t1,t1_reserve+&self.v1);

        //add LP Token
        let lp_token = Token::mint(&self.t0, &self.t1);
        post.set_balance(&self.sender,&lp_token,&self.v0+&self.v1);
        
        Result::Ok(post)
        // Result::Err(TransitionError::Unimplemented)
    }
}

struct Redeem {
    sender: User,
    t0: Token,
    t1: Token,
    v: f64,
}

impl Redeem  {
    fn new(sender: &User, t0: &Token, t1: &Token, v: f64) -> Self {
        assert!(v > 0.0);
        Redeem {
            sender: sender.clone(),
            t0: t0.clone(),
            t1: t1.clone(),
            v,
        }
    }
}

impl Transition for Redeem {
    fn apply(&self, pre: &State) -> Result<State, TransitionError> {
        //UNIMPLEMENTED
        let mut post = pre.clone();
        let lp_token = Token::mint(&self.t0, &self.t1);
        let lp_supply = post.token_supply(&lp_token);
        let ratio_redeem = self.v/lp_supply;
        let t0_reserve = post.get_reserves(&self.t0,&self.t1);
        let t1_reserve = post.get_reserves(&self.t1,&self.t0);
        let t0_balance = post.get_balance(&self.sender,&self.t0);
        let t1_balance = post.get_balance(&self.sender,&self.t1);
        let lp_balance = post.get_balance(&self.sender,&lp_token);

        post.set_reserve(&self.t0,t0_reserve - t0_reserve*self.v/lp_supply,&self.t1,t1_reserve- t1_reserve*self.v/lp_supply);
        post.set_balance(&self.sender,&self.t0, t0_balance + t0_reserve*self.v/lp_supply);
        post.set_balance(&self.sender,&self.t1, t1_balance + t1_reserve*self.v/lp_supply);
        post.set_balance(&self.sender,&lp_token, lp_balance - self.v);

        Result::Ok(post)
        // Result::Err(TransitionError::Unimplemented)
    }
}


struct Swap {
    sender: User,
    tin: Token,
    tout: Token,
    x: f64,
}

impl Swap {
    // tin means token to be in the AMM
    fn new(sender: &User, tin: &Token, tout: &Token, x: f64) -> Self {
        Swap {
            sender: sender.clone(),
            tin: tin.clone(),
            tout: tout.clone(),
            x,
        }
    }
}

impl Transition for Swap {
    fn apply(&self, pre: &State) -> Result<State, TransitionError> {
        //UNIMPLEMENTED
        let mut post = pre.clone();
        let pre_in_balance = post.get_balance(&self.sender,&self.tin);
        let pre_out_balance = post.get_balance(&self.sender, &self.tout);
        //get reserves before swap
        let pre_out_reserve = post.get_reserves(&self.tout,&self.tin);
        let pre_in_reserve = post.get_reserves(&self.tin,&self.tout);
        //calculate constant K
        let constant_num = pre_out_reserve * pre_in_reserve;
        //calculate reserves after swap
        let post_in_reserve = pre_in_reserve + self.x ;
        let post_out_reserve = constant_num/post_in_reserve ;
        //set in token balance
        post.set_balance(&self.sender,&self.tin,pre_in_balance - self.x);
        //set out token balance
        post.set_balance(&self.sender,&self.tout,pre_out_balance + pre_out_reserve-post_out_reserve);
        //set post in and out reserve
        post.set_reserve(&self.tin,post_in_reserve,&self.tout,post_out_reserve);

        Result::Ok(post)
        // Result::Err(TransitionError::Unimplemented)
    }
}

fn price_oracle(s: &State, t: &Token) -> f64 {
    match t{
        Token::Atomic(token0) => {
            if token0 == "t0" {
                return 1000.0
            }else if token0 == "t1"{
                return 1000.0
            }
            0.0
        }
        Token::Minted(t0, t1) => {
            // let minted_balance :f64 = s.token_supply(t);
            let token0 = Token::Atomic(String::from(t0));
            let token1 = Token::Atomic(String::from(t1));
            // let amm = Some(s.get_amm(&token0, &token1)) ;
            let r0 = s.get_reserves(&token0,&token1);
            let r1 = s.get_reserves(&token1,&token0);

            let mut sum = 0.0;
            for wallet in &s.wallets {
                for balance in &wallet.balances {
                    if balance.token == *t {
                        sum += balance.value;
                    }
                }
            }

            (1000.0*r0 + 1000.0*r1)/sum
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

fn price_mini_transaction(p0:f64,p1:f64,r0:f64,r1:f64) -> f64 {
    let p = p1/p0 * r0 * r1;
    let w0 = p.sqrt() - r0;
    w0
}



fn mev0(){
    let t0 = Token::Atomic(String::from("t0"));
    let t1 = Token::Atomic(String::from("t1"));
    let o: User = User::new("O");
    let a: User = User::new("A");
    let m: User = User::new("M");

    let mut s0: State = State::new();

    s0.set_balance(&o, &t0, 100.0);
    s0.set_balance(&o, &t1, 100.0);
    s0.set_balance(&a, &t0, 20.0);
    s0.set_balance(&a, &t1, 0.0);
    s0.set_balance(&m, &t0, 5.9);
    s0.set_balance(&m, &t1, 20.6);


    let mut v: Vec<Box<dyn Transition>> = Vec::new();
    v.push(Box::new(Deposit::new(&o,100.0,&t0,100.0,&t1)));
    
    //inner layer
    let sfr0 = SFr0(20.0, 15.0, 100.0,100.0);
    let v0 = sfr0 - 100.0;
    let sfr1 = SFr1(s0.get_reserves(&t0, &t1), s0.get_reserves(&t1, &t0), sfr0);
    v.push(Box::new(Swap::new(&m,&t0,&t1,v0)));

    v.push(Box::new(Swap::new(&a,&t0,&t1,20.0)));
    
    //price minimization
    let v0 = price_mini_transaction(1000.0,1000.0,79.4, 125.9);
    v.push(Box::new(Swap::new(&m,&t1,&t0,v0)));
    //

    println!("Initial: {:.1}", s0);
    for t in v {
        s0 = t.apply(&s0).unwrap();
        // println!("reserve: {:.1}", s0.get_reserves(&t0, &t1));
        println!("{:.1}", s0);
        println!("\ttotal net_wealth: {:.1}", s0.net_wealth(&price_oracle));
        println!("\tO's net_wealth: {:.1}", s0.net_wealth_user(&o, &price_oracle));
        println!("\tA's net_wealth: {:.1}", s0.net_wealth_user(&a, &price_oracle));
        println!("\tM's net_wealth: {:.1}", s0.net_wealth_user(&m, &price_oracle));
    }
}

fn mev1(){
    let t0 = Token::Atomic(String::from("t0"));
    let t1 = Token::Atomic(String::from("t1"));
    let o: User = User::new("O");
    let a: User = User::new("A");
    let m: User = User::new("M");

    let mut s0: State = State::new();

    s0.set_balance(&o, &t0, 100.0);
    s0.set_balance(&o, &t1, 100.0);
    s0.set_balance(&a, &t0, 100.0);
    s0.set_balance(&a, &t1, 100.0);
    s0.set_balance(&m, &t0, 100.0);
    s0.set_balance(&m, &t1, 100.0);


    let mut v: Vec<Box<dyn Transition>> = Vec::new();
    v.push(Box::new(Deposit::new(&o,100.0,&t0,100.0,&t1)));
    
    //inner layer
    let sfr0 = SFr0(40.0, 35.0, 100.0,100.0);
    let mut v0 = 100.0 - sfr0;
    let sfr1 = SFr1(s0.get_reserves(&t0, &t1), s0.get_reserves(&t1, &t0), sfr0);
    v.push(Box::new(Swap::new(&m,&t1,&t0,v0)));

    v.push(Box::new(Swap::new(&a,&t0,&t1,40.0)));
    v.push(Box::new(Swap::new(&m,&t1,&t0,38.3)));
    v.push(Box::new(Deposit::new(&a,30.0,&t0,40.0,&t1)));
    
    //price minimization
    v0 = price_mini_transaction(1000.0,1000.0,117.0, 155.0);
    v.push(Box::new(Swap::new(&m,&t0,&t1,v0)));
    v.push(Box::new(Redeem::new(&a,&t0,&t1,10.0)));

    
    //price minimization
    let v0 = price_mini_transaction(1000.0,1000.0,78.0, 129.0);
    // v.push(Box::new(Swap::new(&m,&t1,&t0,v0)));
    //

    println!("Initial: {:.1}", s0);
    for t in v {
        s0 = t.apply(&s0).unwrap();
        // println!("reserve: {:.1}", s0.get_reserves(&t0, &t1));
        println!("{:.1}", s0);
        println!("\ttotal net_wealth: {:.1}", s0.net_wealth(&price_oracle));
        println!("\tO's net_wealth: {:.1}", s0.net_wealth_user(&o, &price_oracle));
        println!("\tA's net_wealth: {:.1}", s0.net_wealth_user(&a, &price_oracle));
        println!("\tM's net_wealth: {:.1}", s0.net_wealth_user(&m, &price_oracle));
    }
}

fn mev3(){
let t0 = Token::Atomic(String::from("t0"));
    let t1 = Token::Atomic(String::from("t1"));
    let a: User = User::new("A");
    let b =  User::new("B");

    let mut s0 = State::new();

    s0.set_balance(&a, &t0, 100.0);
    s0.set_balance(&a, &t1, 100.0);
    s0.set_balance(&b, &t0, 100.0);
    s0.set_balance(&b, &t1, 100.0);

    let mut v: Vec<Box<dyn Transition>> = Vec::new();

    v.push( Box::new(Deposit::new(&a,100.0, &t0, 100.0, &t1)));

    v.push( Box::new(Swap::new(&b,&t1, &t0, 13.0)));
    v.push( Box::new(Swap::new(&b,&t0, &t1, 40.0)));
    v.push( Box::new(Swap::new(&b,&t0, &t1, 30.0)));
    // v.push( Box::new(Redeem::new(&a,&t0, &t1, 30.0)));
    // v.push( Box::new(Swap::new(&b,&t0, &t1, 30.0)));
    // v.push( Box::new(Redeem::new(&a,&t0, &t1, 30.0)));

    println!("Initial: {}", s0);
    for t in v {
        s0 = t.apply(&s0).unwrap();
        println!("{}", s0);
    }
}

fn main() {
    // mev3();
    // mev0();
    mev1();
    ///////////////////////////////////AMM
    // let t0 = Token::Atomic(String::from("dai"));
    // let t1 = Token::Atomic(String::from("eth"));
    // let a: User = User::new("A");
    // let b =  User::new("B");

    // let mut s0 = State::new();

    // s0.set_balance(&a, &t0, 70.0);
    // s0.set_balance(&a, &t1, 70.0);
    // s0.set_balance(&b, &t0, 30.0);
    // s0.set_balance(&b, &t1, 10.0);

    // let mut v: Vec<Box<dyn Transition>> = Vec::new();

    // v.push( Box::new(Deposit::new(&a,70.0, &t0, 70.0, &t1)));
    // v.push( Box::new(Swap::new(&b,&t0, &t1, 30.0)));
    // v.push( Box::new(Swap::new(&b,&t1, &t0, 21.0)));
    // v.push( Box::new(Redeem::new(&a,&t0, &t1, 30.0)));
    // v.push( Box::new(Swap::new(&b,&t0, &t1, 30.0)));
    // v.push( Box::new(Redeem::new(&a,&t0, &t1, 30.0)));

    // println!("Initial: {}", s0);
    // for t in v {
    //     s0 = t.apply(&s0).unwrap();
    //     println!("{}", s0);
    // }
}