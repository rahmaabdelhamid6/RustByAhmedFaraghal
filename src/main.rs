// use day1::{Amount, Currency};

use day1::{Amount, Currency};


#[derive(Debug, Clone, Copy)]
pub struct USD; //  d bykhleni mhtgsh l state machine code w switch case 
impl Currency for USD{
    const CODE: &'static str = "USD";
    const SYMBOL: &'static str = "$";
    const RATIO: u8 = 100;
}
#[derive(Debug, Clone, Copy)]
pub struct EGP;
impl Currency for EGP{
    const CODE: &'static str = "EGP";
    const SYMBOL: &'static str = "LE ";
    const RATIO: u8 = 100;
}
fn main()
{
    // let a = Amount::new(55, Currency::USD);
    // let b = Amount::new(66, Currency::USD);
    // println!("Amount is {}", a + b);

    let a = Amount::new(55, USD);
    let z = Amount::new(55, USD);
    println!("Amount is amount of a {:#}", a );
    // println!("Amount is amount of a + z = {}", a +z);
    let b = Amount::new(77, EGP);
    println!("Amount is amount of b {}", b );
    // println!("Amount is amount of a + b {}", a +b);
}