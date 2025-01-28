// dst -> dynamic size type 
// difference between smart pointer arc, rc, box and refcell 
// RAII -> Resource acquisition is intilization -> nk call func btreturn some value el value bt present resource ma f lma el value d get dropped yba'a na lost el ownership aw el access 3la el resource d  

//box -> heap allocation , singleowner, you can pass el ownership to other but can't share ownership 
// rc -> in same thread you can have multiple owner and the last one have its ownership he is the ressipnsible of dropped it and dallocated el memeory from the heap 
//rc doesn't implement deref mut yba'a na mch masmohli a mutate elly gowah bs 3ndha get_mut d mutate f halet bs lw fadel owner wahed f el strong_count == 1 w return option mut f lw feh owner wahed return some w ymutate
//arc-> same as rc but have one differenece -> shared between multiple threads
//refcell -> peiece of memory bs 3awez ta'oul ll rust please don't  apply el ownership rule at compile time but in runtime because what i need to do is dynamic w hayb'a safe bs sebni asbtlk d 
// btdini access 3la mut memory location f kol el funtions elly mhatagha 3chan a mutate hya actually protected by shared ref f el func mch mhtaga mutself mhataga self bs-> bt3ml el powercheck rules f el runtime mch el compile time 
//refcell m3ndhash defer traitt bs btdini Api btsa3dni 3chan atl3 el ref elly gowa elly hya borrow 3chan hya return struct m implement el deref  
// to combine -> interior mutability -> y3ni el interface elly f eidi mch yrequire ykun m3aya mutable access kol el layers elly mt3rf n you can mutate it f bthide d 

// struct MyVec<T>{
//     // smart pointer elly hwa box 1- d heap allocated , 
//     //2- implmeent trait deref w d elly bykhleh y3ml automatic coarsion f bynmsl ka'ano ref mn el type elly gowah f el hatet elly mhtaga d zy el check f func push
//     // 3- lw dropped el box el memory will be get deallocated 
//     //4- implement drop trait 
//     items : Box<[Option<T>]>, // 2 words -> word el pointer have the address of the memeory in heap, w word length 
//     capacity: usize, // 1 word 
//     // length : usize, // 1word
// }

// impl<T: Copy> MyVec<T> {
//     pub fn new() ->Self{
//         MyVec { 
//             items: Box::new([None; 10]),  // create array of options bhes amlaha b none w lma ykun feh data f yb'a some 
//             capacity: 10, 
//             // length: 0 
//             }
//     }
//     pub fn with_capacity(capacity: usize) ->Self{
//         MyVec { 
//             items: Box::new([None; capacity]),  // create array of options bhes amlaha b none w lma ykun feh data f yb'a some 
//             capacity, 
//             // length: 0 
//             }
//     }
//     pub fn push (&mut self, item: T){
//         if self.capacity == get_array_length(&self.items){

//         }
//     } 
// }
// fn get_array_length<T>(arr : &[T]) -> usize{
//     arr.len()
// }
use std::rc::Rc;
use std::cell::RefCell;
struct File{
    name :String,
    data: Vec<u8>,
}
impl Drop for File {
    fn drop(&mut self) {
        println!("Dropping file: {}", self.name); 
    }
}
impl File{
    pub fn new (name: &str, length: usize) ->File{
        File{
            name: name.to_string(),
            data: vec![0; length],
        }
    } 
}
struct Directory{
    name: String,
    // files: Vec<Rc<File>>, // 3chan el Rc ma'adrsh adkhul w mutate wq aghyer elly gowah hastkhdm refcell 
    files: Vec<Rc<RefCell<File>>>, // shared owner ship l mutable memory location
}

impl Directory {    
    pub fn new(name:&str) ->Self {
        Directory{
            name: name.to_string(),
            files: vec![],
        }
    }
    pub fn add_file(&mut self, file: Rc<RefCell<File>>){
        self.files.push(file);
    }
    pub fn rm_file (&mut self, filename : &str){
        self.files.retain(|file| file.borrow().name != filename); // retain b return true lw 3awez akhle el file w false lw ymsaho 
    }
    pub fn list_files(&self) {
        println!("{}", self.name);
        for file in &self.files{
            let file_ref = file.borrow();
            println!("  {} - {} bytes -- {} links", file_ref.name, file_ref.data.len(), Rc::strong_count(&file));
        }
        println!();
    }
    pub fn rename_file(&mut self, old_filename :&str , new_filename: &str){
        for file in &self.files{
            let mut file_ref = file.borrow_mut();
            if file_ref.name == old_filename{
                // let mut file_mut_ref = file.borrow_mut();
                // file_mut_ref.name = String::from(new_filename);
                file_ref.name = String::from(new_filename);
            }
        }
    }
}
fn main(){

    // rc nfs fekret el file system w hwa ni yb'a 3ndi 3 directory byshawro 3la nfs el binary f el disk bdl m akrr el binary d marten 
    let mut dir1 = Directory::new("/usr");
    let mut dir2 = Directory::new("/home");
    
    let file1 = Rc::new(RefCell::new(File::new("data1", 100)));
    let file2 = Rc::new(RefCell::new(File::new("data2", 200)))  ;
    dir1.add_file(Rc::clone(&file1));
    dir1.add_file(Rc::clone(&file2));
    dir2.add_file(file1);
    dir2.add_file(file2);

    // dir1.add_file(File::new("data1", 100));
    // dir1.add_file(File::new("data2", 200));
    // dir2.add_file(File::new("data1", 100));
    // dir2.add_file(File::new("data2", 200));

    println!("List of Files in directory:");
    dir1.list_files();
    dir2.list_files();

    dir2.rename_file("data2", "new_data2");

    println!("Removing data1 from dir2");
    dir2.rm_file("data1");

    println!("List of Files in directory:");
    dir1.list_files();
    dir2.list_files();

    println!("Removing data1 from dir1");
    dir1.rm_file("data1");

    println!("List of Files in directory:");
    dir1.list_files();
    dir2.list_files();

}