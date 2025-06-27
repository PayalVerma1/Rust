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
// struct User{
//   name:String,
//   age:u32,
//   active:bool,
// }
// fn main(){
//   let user1=User{
//     name:String::from("Alice"),
//     age:30,
//     active:true,
//   };
//   println!("User1: Name: {}, Age: {}, Active: {}", user1.name, user1.age, user1.active);
// }
// struct Rect{
//     width:u32,
//     height:u32,
// }
// impl Rect{
//   fn area(&self)->u32{
//     self.width * self.height
//   }
// }
// fn main(){
//   let r1=Rect{
//     width:30,
//     height:50,
//   };
//   println!("The area of the rectangle is: {}", r1.area());
// }
// enum Shape {
//     Circle(f64), // radius
//     Rectangle(f64, f64), // width, height
// }
// fn calculate_area(shape:Shape)->f64{
// let ans=match shape{
//    Shape::Circle(radius)=>3.14 * radius * radius,
//    Shape::Rectangle(width,height)=>width * height,
// };
// return ans;
// }
// fn main(){
//   let Circle=Shape::Circle(5.0);
//   let Rectangle=Shape::Rectangle(4.0, 6.0);
//   println!("Area of Circle: {}", calculate_area(Circle));
// }
// use std::fs;
// enum Result<T,E>{
//   Ok(T),
//   Err(E),
// } 
// fn main(){
//   let res=fs::read_to_string("hello.txt");
//   match res{
//     Ok(content)=>{
//       println!("File content: {}", content);
//     }
//     Err(err)=>{
//       println!("Error reading file: {}", err);
//     }
//   }
// }

// struct Point<T>{
//     x: T,
//     y: T,
// }
// fn main(){
//   let p1=Point{x:5,y:10};
//   println!("Point p1: x={}, y={}", p1.x, p1.y);
// }
// fn main(){
//    println!("{}", find_even(10));
// }
// fn find_even(num:i32)->bool{
//   if num%2==0{
//     return true;
//   }
//   else{
//     return false;
//   }
// }





//vectors

// fn main(){
//   let mut vec=Vec::new();
//   vec.push(1);
//   vec.push(2);
//   vec.push(3);
//    let ans= even_filter(vec); 
//   println!("Vector elements:{:?}", ans);
// }
// fn even_filter(vec:Vec<i32>)->Vec<i32>{
//   let mut new_vec=Vec::new();
//   for val in vec{
//     if val%2==0{
//       new_vec.push(val);
//     }
//   }
//   return new_vec;
// }
// fn main(){
//   let mut vec=Vec::new();
//   vec.push(1);
//   vec.push(2);
//   vec.push(3);
//   vec.push(4);
//   even_filter(&mut vec);
//   println!("{:?}",vec);
// }
// fn even_filter(vec:&mut Vec<i32>){
//   let mut i=0;
//  while i<vec.len(){
//   if vec[i]%2!=0
// {
//    vec.remove(i);
// }
// else{
//   i+=1;
// } }

// }



//HashMap
// use std::collections::HashMap;
// fn main(){
//   let mut users=HashMap::new();
//   users.insert(String::from("Payal"),  25);
//    users.insert(String::from("verma"), 22);
//    let first_user_age=users.get("Payal");
//    match first_user_age{
//     Some(age) => println! ("{:?}" ,age),
//     None => println!("User not found"),
//    }
// }
// use std::collections::HashMap;
// fn main(){
//   let input_vec=vec![(String::from("Payal"),21), (String::from("Verma"), 22), (String::from("Alice"), 23)];
//   let hm=vec_to_hashmap(input_vec);
//   println!("HashMap: {:?}", hm);
// }
// fn vec_to_hashmap(vec:Vec<(String,i32)>)->HashMap<String,i32>{
//      let mut hm=HashMap::new();
//      for(key,value) in vec{
//       hm.insert(key, value);
//      }
//      return hm;
// }




//iterators

// fn main(){
//   let mut v1=vec![1,2,3,4,5];
//   let iter=v1.iter_mut();
//   for val in iter{
//     *val=*val + 1; 
//   }
//   println!("Updated vector: {:?}", v1);
// }

// fn main(){
//   let mut v1=vec![1,2,3,4,5];
//   let mut iter=v1.iter_mut();
//  while let Some(val)=iter.next(){
//   println!("{}",val);
//  }
//   println!("Updated vector: {:?}", v1);
// }


//into_iter() is a method that converts a collection into an iterator that takes ownership of the collection's items.
// fn main(){
//   let nums=vec![2,3,4,5];
//   let iter=nums.into_iter();
//   for val in iter{
//     println!("{}",val);
//   }
// }

//consuming adapters
//take ownership of the iterator and produce a final result — they consume the iterator.
// fn main(){
//   let vec=vec![2,3,4,5];
//   let iter=vec.iter();
//   let sum:i32=iter.sum();
//   println!("{}",sum);
// }
//iterator adapter
// fn main(){
//    let vec=vec![2,3,4,5];
//   let iter=vec.iter();
//   let iter2=iter.map(|x|x+1);
//   for x in iter2{
//     println!("{}",x);
//   }
// }
fn main(){
   let vec=vec![2,3,4,5];
  let iter=vec.iter();
  let iter2=iter.filter(|x| **x%2==0).map(|x| x+1);
  for x in iter2{
    println!("{}",x);
  }
}