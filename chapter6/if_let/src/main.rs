fn main() {
    compare_approaches();
}

fn compare_approaches() {
    compare_with_match(Some(3));
    compare_with_if_let(Some(3));
    
    compare_with_match(Some(2));
    compare_with_if_let(Some(2));

    compare_with_match(None);
    compare_with_if_let(None);

    let mut count = 0;
    count = count_with_match(Coin::Dime, count);
    count = count_with_match(Coin::Dime, count);
    count = count_with_match(Coin::Dime, count);
    count = count_with_match(Coin::Quarter(UsState::Alabama), count);
    count = count_with_match(Coin::Quarter(UsState::Alaska), count);
    println!("{}", count);
    count = count_with_if_let(Coin::Dime, count);
    count = count_with_if_let(Coin::Dime, count);
    count = count_with_if_let(Coin::Dime, count);
    count = count_with_if_let(Coin::Quarter(UsState::Alabama), count);
    count = count_with_if_let(Coin::Quarter(UsState::Alaska), count);
    println!("{}", count);
}

fn compare_with_match(some_u8_value: Option<u8>) {
    // exhaustive checking
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };
}

fn compare_with_if_let(some_u8_value: Option<u8>) {
    // more concise
    if let Some(3) = some_u8_value {
        println!("three");
    };
}

enum Coin {
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn count_with_match(coin: Coin, count: u32) -> u32 {
    match coin {
        Coin::Quarter(state) => {
            println!("Quarter from {:?}!", state);
            count
        },
        _ => count + 1,
    }
}

fn count_with_if_let(coin: Coin, count: u32) -> u32 {
    if let Coin::Quarter(state) = coin {
        println!("Quarter from {:?}", state);
        count
    } else {
        count+1
    }
}
