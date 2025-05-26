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
fn main(){
  let sentence=String::from("Hello world!");
  let first_word=get_first_word(sentence);
  let n=1000;
  for i in 1..n{
    println!("Hello, world! {}",i);
  }println!("The first word is: {}", first_word);
}
fn get_first_word(sentence:String)->String{
  let mut ans=String::from("");
  for char in sentence.chars(){
    ans.push_str(char.to_string().as_str());
    if char==' '{
      break;
  }
} return ans
}