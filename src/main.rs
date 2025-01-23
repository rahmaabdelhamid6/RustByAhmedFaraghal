// feh website esmu godbolt -> d disassembler btl3li el assembly code 
//explicit better than implicit
// fn pick (value:u8){
//     if value% 2 == 0{
//         true
//     }else {
//         false
//     }
// }

// hna ma'drsh aktbo kda 3chan el lifteime bta3o at3mlo unwind mn el stack b3d el func scope m khls lw kan ay type ghir string 
// lakn el &str bytktb f data seg elly maktub hardcoded b eidi f el code zy even aw odd  f el lifetime bta3o static 3omro mn 3omr el program kollo  f bhtolo lifetime annotation esmo static , d ghir el string elly byigi f el runtime f d dynamically allocated 

// fn pick (value:u32) -> &'static str {
//     if value% 2 == 0{
//         "even"
//     }else {
//         "odd"
//     }
// }

// fn pick<T> (value:u32, even :T, odd: T) -> T {
//     if value% 2 == 0{
//         even
//         // f el C++ lw grb aktb even+1 w get f el main b3t 100 , 200 bs mshtkhdmtsh ay type tni mch int hay3deha 3chan wa'at el call byruh yshel T w yhut bdalha 100 f hayba'a 3adi y +1 3leha f lw el code compiled tmm hat3di lw la haydini  error 
//         // lakn f el rust mcha hay3diha 3chan d generic templatef btb'a 3awza thut validation rule 3la el template code abl m na a3mlo call f hya lazem tb'a correct abl m had ynadeha f fekeret ni ab3tlha str f mch valid +1 3leh f hayidini el error mn badri 
//         // f lw habet ahut +1 f el rust lazem akun mdeh info 3n T takfi nu y3amlha k int
//     }else {
//         odd
//     }
// }

//create trait generic over el return type like rand::random
trait Random {
    fn generate () ->Self;
}
impl Random for u8{
    fn generate () -> Self{
       55
    }
}
impl Random for &str{
    fn generate () -> Self{
       "hola"
    }
}
//set boundaries to describe T -> fn random <T: >() or fn random() where T : 
fn random <T> () ->T 
where T : Random, 
{
    T::generate()
}
fn pick_u8 (value:u8){
    println!("you picked {}", value);
}
fn pick_str (value:&str){
    println!("you picked {}", value);
}
//create generic on lifetime
// lifetime annotation
#[derive(Debug)]
enum  Either<'a,'b> // el enum shayel ref mch l values bs 2 different lifetimes
{
    This(&'a str),
    That(&'b str),
}
#[derive(Clone)]
struct Employee{
    name: String, 
    age: u32,
}

fn get_name<'a> (employee_1: &'a Employee) -> &'a str{
    &employee_1.name
}
fn who_is_older<'a, 'b>(employee_1: &'a Employee , employee_2: &'b Employee) -> Either<'a, 'b>{
    if employee_1.age > employee_2.age{
       Either::This((&employee_1.name)) 
    }else {
        Either::That((&employee_2.name))
    }
}

// fn who_is_older<'a>(employee_1: &'a Employee , employee_2: &'a Employee) -> &'a str{
//     if employee_1.age > employee_2.age{
//         &employee_1.name
//     }else {
//         &employee_2.name
//     }
// }
fn main (){
    // let v = rand::random(); // random d generic 3la el return type bta3ha f el type inferance my3rfsh yhl hwa hayrg3 eh mn el line d bs f mhtag source tani y3rf el type eh f lma bnb3taha l pick btakhud nfs el type 
    // f lw kan pick 3nd awl arg btakhud brdu generics kan haydini error 3chan mch 3aref ydeha type eh???
    // println!("you picked {} , it's {}", v, pick(v));
    // println!("you picked {} , it's {}", v, if pick(v){"even"}else {"odd"});

    // momkn a force d 3la type na 3amli b eni ahut ablo #[drive(Copy,Clone)] 3la el type elly 3mlto
    let v = random(); //primitive types (built in types like u8,u32,etc) are cheaper to do this that send it as ref, they make copy instead of move 
    pick_u8(v);
    
    // println!("you picked it's {}", pick(v));

    // println!("you picked {} , it's {}", v, pick(v, "even", "odd"));
    // println!("you picked {} , it's {}", v, pick(v, true , false ));
    // println!("you picked {} , it's {}", v, pick(v, 100, 200));

    let mahmoud = Employee{
        name: "Mahmoud".to_string(),
        age: 22
    };
    let name = get_name(&mahmoud);
    println!("my name is {}", name);

    let ahmed = Employee{
        name: "ahmed".to_string(),
        age: 20
    };
    // println!("the older is {}", who_is_older(&mahmoud, &ahmed));
    println!("the older is {:?}", who_is_older(&mahmoud, &ahmed));
}