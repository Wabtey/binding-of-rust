fn main() {
    counting_up_ex();

    println!("------------------------");

    return_value_from_loop();

    println!("------------------------");

    while_exemple();
}

fn counting_up_ex(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                //'counting_up will count up from 0 to 2
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn return_value_from_loop(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_exemple(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!();
    println!("Looping through a Collection with while");
    println!();

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!();
    println!("Looping through a Collection with for");
    println!();

    for element in a {
        println!("the value is: {}", element);
    }

    println!();
    println!("a another method (range and .rev()");
    println!();

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}