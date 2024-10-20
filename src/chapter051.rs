#[test]
fn test1(){
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}

#[test]
fn test2(){
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

#[test]
fn test3(){
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    s.clone()
}

#[test]
fn test4(){
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}
fn print_str(s: &String) {
    println!("{}", s)
}

#[test]
fn test5(){
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

#[test]
fn test6(){
    let s = String::from("Hello ");
    let mut s1 = s;

    s1.push_str("World!");
    println!("Success!");
}

#[test]
fn test7(){
    let x: Box<i32> = Box::new(5);
    let mut y: Box<i32> = Box::new(1);
    *y = 4;

    assert_eq!(*x, 5);
    println!("Success!");
}

#[test]
fn test8(){
    let t: (String, String) = (String::from("hello"), String::from("world"));
    let _s = t.0;

    println!("{:?}", t.1);
}

#[test]
fn test9(){
    let t: (String, String) = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t);
}