// fn main(){
  //immutable variable can't be assigned a new value
//     let x=5;
//     for i in 0..1000{
//         x=x+i;
//         println!("x is {}",x);
//     }
// println!("Hello, world!");
// }
//fn main(){
    //Boolean
//     let is_male =true;
//     let is_above18=false;
//     if is_male{
//         println!("you are a male";)
//     } 
//     else {
//         println!("you are not male")
//     };
//    if(is_male && is_above18){
//         println!("
//         you are a legal male")
//    }
// }
// fn main(){
//     let greet=String::from("Hello,World!");
//     println!("{}",greet);
// }
// fn main(){
//   let sentence=String::from("Hello world!");
//   let first_word=get_first_word(sentence);
//   let n=1000;
//   for i in 1..n{
//     println!("Hello, world! {}",i);
//   }println!("The first word is: {}", first_word);
// }
// fn get_first_word(sentence:String)->String{
//   let mut ans=String::from("");
//   for char in sentence.chars(){
//     ans.push_str(char.to_string().as_str());
//     if char==' '{
//       break;
//   }
// } return ans
// }
// fn main(){
//   let mut x=1;
//   x=2;
//   println!("x is {}",x);
// }
// fn main(){

//   stack_fn();
//   heap_fn();
//   update_fn();
// }
// fn stack_fn(){
//   let a=10;
//   let b=20;
//   let c=a+b;
//   println!("Stack function: a={}, b={}, c={}", a, b, c);
// }
// fn heap_fn(){
//   let s1=String::from("Hello");
//   let s2=String::from("World");
//   let s3=s1 + " " + &s2; // s1 is moved, s2 is borrowed
//   //string is stored in heap 
//   println!("Heap function: s3={}", s3);
// }

// fn update_fn(){
//   let mut s=String::from("Hello");
//   println!("Before update: {}", s);
//   println!(
//     "Capacity:{}, Length:{}, pointer:{:p}",
//     s.capacity(),
//     s.len(),
//     s.as_ptr()
//   );
//   s.push_str(", World!"); 
//   println!("After update: {}", s);
//   println!(
//     "Capacity:{}, Length:{}, pointer:{:p}",
//     s.capacity(),
//     s.len(),
//     s.as_ptr()
//   );
// }
//Anytime you put something on heap it has its pointer stored in stack or has its owner in stack
//if the stack ever get out of scope the heap will be freed
//let s1=String::from("Hello");
//let s2=s1; // s1 is moved to s2, s1 is no longer valid to solve dangling pointer issue and double free issue

//Bring back ownership
// fn main(){
//   let s1=String::from("hello");
//    s1=take_owner(s1);
//   println!("s1 is: {}", s1);
// }
// fn take_owner(some_string:String)->String{
//   println!("{}",some_string);
//   return some_string;//return ownership back to the original main
// }
//Borrowing and reference

//if the owner dies the string will also die
// but if the borrowors die it will not cause any affect on the string
// fn main(){
//   let s1=String::from("Hello,Rust");
//   let s2=&s1;
//   println("{}",s1);
//   println("{}",s2);
// }

// fn main(){
//   let mut s1=String::from("hello");
//   update_str(&mut s1);
//   println!("{}",s1);
// }
// fn update_str(s:&mut String){
//   s.push_str(" world");
// }
//if there is some immutable reference then you can't have another immutable reference either
// ex   let mut s1=String::from("hello");
//    let mut s2=&mut s1
//    but can't have let s3=&s1
struct User{
  name:String,
  age:u32,
  active:bool,
}
fn main(){
  let user1=User{
    name:String::from("Alice"),
    age:30,
    active:true,
  };
  println!("User1: Name: {}, Age: {}, Active: {}", user1.name, user1.age, user1.active);
}