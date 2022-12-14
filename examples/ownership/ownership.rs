/*
fn main() {
   println!("Before {}", s); // 1
   if true {
     println!("Before declaration {}", s); // 2
     let s = "Hello World";
     println!("s: {}", s); // 3
   }
   println!("After {}", s); // 4
}
*/

/*
// This won't compile
fn main() {
   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{}", s);
   let s2 = s;
   println!("{}", s);
}
*/

fn main() {
   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{}", s);
   let s2 = s.clone();
   println!("{} {}", s, s2);
}
