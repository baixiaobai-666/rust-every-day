
// fn main() {
//   let s1 = String::from("hello");
//   let s2 = s1;
//   println!("{}, world!", s1);
// }


// fn main() {
//   let s1 = String::from("hello");
//   let s2 = s1.clone();

//   println!("s1 = {}, s2 = {}", s1, s2);

//   let x = 5;
//   let y = &x;

//   assert_eq!(x, 5);
//   assert_eq!(*y, 5)

// }

// fn main() {
//   let x = String::from("hello world");
//   let len = calculate_length(&x);
//   println!("The length of '{}' is {}.", x, len);
// }
// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// fn main() {
//   let mut x = String::from("hello world");
//   calculate_length(&mut x);
// }
// fn calculate_length(s: &mut String) {
//   s.push_str(" ！");
//   println!("{}", s);
// }

// fn main() {
//   let mut s = String::from("hello");

//   let r1 = &s;
//   let r2 = &s;
//   println!("{} and {}", r1, r2);
//    // 新编译器中，r1,r2作用域在这里结束

//   let r3 = &mut s;
//   println!("{}", r3);
// } 
// // 老编译器中，r1、r2、r3作用域在这里结束
 // 新编译器中，r3作用域在这里结束

fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");

  &s
}