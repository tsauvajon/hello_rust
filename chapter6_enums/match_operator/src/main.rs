enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    coins();
    option();
    placeholder();
}

fn coins() {
    assert!(value_in_cents(Coin::Penny) == 1);
    assert!(value_in_cents(Coin::Nickel) == 5);
    assert!(value_in_cents(Coin::Dime) == 10);
    assert!(value_in_cents(Coin::Quarter(UsState::Alabama)) == 25);
    assert!(value_in_cents(Coin::Quarter(UsState::Alaska)) == 25);
}

fn option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    assert!(six == Some(6));
    assert!(none == None);
}

fn placeholder() {
    let some_u8_value = 0u8;
    let result = match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };
    println!("result: {:?}", result);
}
