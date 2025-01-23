// to run optimize cargo run --profile=release or cargo run --release/
// rust mfihash null pointer w exceptions
// when i need to use crate it must be added in cargo.toml you can put it manually or by cargo add crate name 
// kol crates leha el features bta3tha elly btnzl beha feh enabeled defualt features w another feature you can enable elly gnbha - lma bagi a rurn cargo add cratename d btkun elly mch enabled
// momkn akhtar enable full aw ahdd elly na 3awzaha bs cargo add -F full or featurename 

//b intall b cargo install cargo-nameofwhatinedd (cargo install cargo-expand) , b search b cargo search searchname 

// ka'ani b3ml el result enum
#[derive(Debug)] 
pub enum Status <S,E> //generics types bhdd el type elly 3awez ab3to lma bagi astkhdmo  
{
    Tamam(S),
    MchTamam (E),   
}
pub trait Serialize{
    fn serialize(&self);
    fn length(&self) -> i32{
        0
    }
}

impl <S, E> Serialize for Status <S, E> // kda b genralize el trait l ay type -> pattern matching : value it will be checked on runtime 
// ma3lomat el typecheck d compile time mch run time 
// impl Serialize for Status <(), String> // hna bhddlha types mo3yana tshtghl m3aha 
where  // ba'dar a3rf el type wl traits elly motaha leha w el implementors mn rust  std libaray website,  w feh goz' feha esmu blanket implemetation w d el safat elly trait byakhudha l ay T aw S b shroot aw mn ghir shroot mch ll trait nfsha 
    S: std::fmt::Debug,  // kol el ma3loma elly mhtag a3rfha 3n S,E nha ay type w b y w thier implementation of debug trait 3lehum 

    E: std::fmt::Debug,
{
    fn serialize(&self) {
        // todo!(); // it do panic 
        match self {
            Status::Tamam(x) => println!("{:#?}", x),
            Status::MchTamam(y) => println!("{:#?}", y),
        }
    }
    
}
//rust have 2 types of macro declared macro look like function and proc macro #[]
#[derive(Debug)] // #[internal macro] -> internal mace give ability to drive a group of builtin traits like Debug, Clone, drop ..etc
pub struct  MemoryBudget{
    remaining_bytes: usize, //usize change based on word size in the architecture -> usize::MAX lw grbt atb3o haydini akbr num

}
// self -> owned value
// &self -> shared reference (borrow)
// &mut self -> exclusive reference 
//inmplement block on a type i made 
impl MemoryBudget {
    pub fn new(budget: usize) -> Self // momkn a return MemoryBudget== Self aw Result enum
    {
        Self{remaining_bytes: budget}
    }
    #[must_use]
    pub fn decrement(&mut self, mem:usize) -> Status<(), String> {
        if self.remaining_bytes < mem{
            Status::MchTamam( "no memory left ".to_owned())
        }else{
            self.remaining_bytes -= mem;
            Status::Tamam(())
        } 
    }
    //consuming self -> take ownership 
    // pub fn allocate(mut self , mem :usize){
    //     self.remaining_bytes -= mem;
    // }
    pub fn remaining(&self) ->usize{
        self.remaining_bytes
    }
    pub fn bye(self){
        // drop fn get automatically awl m el object y get outof scope had yakhud el ownership bta3to masln y get consumed 

    }
}
//orphan rule -> hwa ni implement public trait on public type lakn public train on my own type it's valid or make my own trait on my own types or my own trait on public type
//implement trait fot type , lma bagi asmi trait afkr b it can aw could be debug masln aw display , etc w lma el trait bykun byakhud self d bydi power lli ha3mlo implementation w momkn el trait ykun leha body aw la 
impl  Drop for MemoryBudget {
    // bruh ll rust standard library w adwr 3la el trait elly 3awzaa implementaha w a3mlha 
    //https://doc.rust-lang.org/std/ops/trait.Drop.html
    fn drop(&mut self) {
        println!("Dropping Memory Budget -{}", self.remaining_bytes)
    }
}
// &mut -> exclusive reference
// mut -> mutable binding el efferct bta3ha 3la el value elly f eidi 
fn main() {
    // let a: i32 = 5;
    // println!("Hello, world! {}",  usize::MAX);
    let  mut budget = MemoryBudget::new(1024);
    // drop(budget); // consume el value f b3d el line d mhdsh hay3rf ustkhdm budget 3chan ka,nha btfree el memory aw destructor
    // MemoryBudget::decrement(&mut budget, 512);
    // MemoryBudget::allocate( budget, 512);
    println!("A");
    let status = budget.decrement(2000);
    status.serialize();
    println!("Status :{:#?}", status);
    // println!("remaining memory:{:#?}", budget);
    println!("B");
    budget.bye();
    // drop(budget);
    println!("C");

}
