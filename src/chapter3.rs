#[test]
fn test() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn test1() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

// ctrl + k = push
// ||| + GIT + CLONE

#[test]
fn test2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}


#[test]
fn test4() {
    define_x();
}
fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}

#[test]
fn test5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("{}", x); // Prints "42".
}

#[test]
fn test6() {
    let mut x: i32 = 1;
    x = 7;
    x = x + 3;


    let y: i32 = 4;
    let y: &str = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn text7() {
    let x: i32 = 1;
}

#[test]
fn test8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
fn test9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}


















