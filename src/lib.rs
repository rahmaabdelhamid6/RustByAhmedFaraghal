use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Add;
/*type modeling -> how to represent problem domain in time domain (compile cost not run time cost)  */
#[derive(Debug)]
pub struct OverFlowError;
//new type pattern to make the validations easier
// #[derive(Debug)]
// na kda khalet currency check byhsl f el run time , tayep lw 3awzo yba'a compoile time info bhes mfdlsh mohtafez beh f el memory tol m na masgy?  
// pub struct Amount // make u64 private so no one have an access on it except me 
// {
//     amount :i64,
//     currency: Currency, // }

// if i want to represnt currency 1- use enum but it have a problems -> 1- if some one want to exapnd or add a new currency it should go to the owner of the crates to add it
// 2- at every time you will use enum you must use match statement and say what we will do in each case m 3- el enum lw hatet ablo pub f kda kol el members elly feh pub by default 3aks el struct el members bta3to ba'adar athakem men pub w men la 
// pub enum Amount {
//     USD (i64),
//     GBP (i64),
//     EGP (i64),
// }
// #[derive(Debug)]
// pub enum Currency {
//     USD,
//     GBP,
//     EGP,
// }

// impl Amount{
//     pub fn new (amount: i64, currency: Currency) ->Self{
//         Amount{amount,currency}
//     }
// }

// Add trait assumes if i didn't send a value it will right hand side (rhs) = left hand side (lhs) -> Generic type 
// you could find Add trait in rust std 
// impl Add<Self> for Amount
// {
//     // generics in traits two types -> 1- parameter <>, 2- associated type = 
//     type Output = Self;  // ka'ani ba'oul d el natiga , one to one mapping control el output type 
//     fn add(self, rhs: Self) -> Self::Output {
//         Amount{
//            amount:  self.amount + rhs.amount,
//            currency: 
//         }
//     }
// }

// impl Add<i32> for Amount
// {
//     // generics in traits two types -> 1- parameter <>, 2- associated type = 
//     type Output = Result<Self, OverFlowError>;  // ka'ani ba'oul d el natiga , one to one mapping control el output type 
//     fn add(self, rhs: i32) -> Self::Output {
//         Ok(Amount(self.amount.checked_add(rhs as i64).ok_or(OverFlowError)?))
//     }
// }

// impl Display for Amount{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "${}.{}", self.0 /100 , self.0 %100)
//     }
// }
// el traits mfihash variabels , function and const only 
pub trait Currency // kda d haykhleni a3rf el currency f el main w ay had 3awez y3ml currency gdida ya'ader 
{
    const CODE: &'static str;
    const SYMBOL: &'static str;
    const RATIO: u8;
}
// #[derive(Debug, Clone, Copy)]
// pub struct USD;
// pub struct EUR;
// #[derive(Debug, Clone, Copy)]
// pub struct EGP;
#[derive(Debug, Clone, Copy)]
pub struct Amount<C> {
    amount:i64,
    _phantom :PhantomData<C>, // Phantom d type f el rust b store feh type bs size 0 lw mch hast3mlo 3chan a satisfy el type system 
}
impl<C:Currency> Amount<C> {
    pub fn new (amount: i64, _currency:C ) ->Self{
        Amount{amount, _phantom:PhantomData::default()}
    }
}
// 3awez a3ml trait a'der atl3 mno el info elly mhtagha zy $ , usd , /100 , etc 
impl<C> Display for Amount<C>
where 
    C:Currency, 
{ 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let major = self.amount / C::RATIO as i64;
        let minor =self.amount % C::RATIO as i64;
        if f.alternate(){
            return write!( f, "{}{}.{}", C::CODE, major, minor);
        }else{
             write!( f, "{}{}.{}", C::SYMBOL, major, minor)
        }
    }
}
//generic imp l ay type 
// impl<C> Display for Amount<C>{ 
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "${}.{}", self.amount /100 , self.amount %100)
//     }
// }

// or be sepecified for each type 
// impl Display for Amount<USD>{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "${}.{}", self.amount /100 , self.amount %100)
//     }
// }

impl <C:Currency> Add<Amount<C>> for Amount<C> // self here refer to that it must both sides have the same currency 

{
    // generics in traits two types -> 1- parameter <>, 2- associated type = 
    type Output = Amount<C>;  // ka'ani ba'oul d el natiga , one to one mapping control el output type 
    
    fn add(self, rhs: Amount<C>) -> Amount<C> {
        Amount{
           amount:  self.amount + rhs.amount,
           _phantom: PhantomData 
        }
    }
}

// impl <C, T> Add<Amount<T>> for Amount<C> // to add two different currencies 
// {
//     type Output = Amount<USD>;  
//     fn add(self, rhs: Amount<T>) -> Amount<USD> {
//         Amount{
//            amount:  self.amount + rhs.amount,
//            _phantom: PhantomData 
//         }
//     }
// }
