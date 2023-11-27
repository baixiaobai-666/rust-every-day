
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

// fn main() {
//   let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//   let s = String::from("hello");

//   &s
// }

fn main() {
  let condition = true;
  if condition == true {
    println!("condition is true");
  } else {
    println!("condition is false");
  }
  let condition1 = true;
  let number = if condition1 {
    5
  } else {
    6
  };
  println!("The number is {}", number);

  let var1 = 10;
  let var2 = if var1 >= 10 {
    var1
  } else {
    var1 + 10
  };
  println!("{}", var2);
  let mut v = 0;
  for i in 1..10 {
    v = if i == 9 {
      continue;
    } else {
      i
    }
  }
  println!("{}", var1); 

  let o = Some(3);
  let v = if let Some(x) = o {
    x
  } else {
    0
  };
  println!("{}", v);
  let res: Option<i32> = check();
  println!("{:?}", Some(res)); 
  for_some();
  some_while();
  some_loop();
}

fn check() -> Option<i32> {
  let n: i32 = 6;
  if  n % 4 == 0 {
    Some(n - 4)
  } else if  n % 3 ==0 {
    Some(n - 3)
  } else if n % 2 == 0 {
    Some(n - 2)
  } else {
    Some(n)
  }
}

fn for_some() {
  for i in 1..5 {
    println!("{}", i);
  }

  let a = [1, 2, 3, 4, 5];
  // iter 方法把数组变成一个迭代器
  for (i, v) in a.iter().enumerate() {
    println!("第{}个元素是{}", i + 1, v)
  }
}

fn some_while() {
  let mut n = 0;
  while n <= 5 {
    println!("{}", n);
    n += 1;
  }
  println!("我执行完成了！！！")
}

fn some_loop() {
  let mut n = 0;
  loop {
      if n > 5 {
        break;
      } else {
        println!("{}", n);
          n += 1;
      }
  }
  println!("我执行完成了！！！")
}