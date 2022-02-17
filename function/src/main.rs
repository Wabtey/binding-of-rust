fn five() -> i32 {
    5
}

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    let mut x = five();
    x =plus_one(x);

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    //this is a expression so there isn't semi-colon ;
    x + 1
}