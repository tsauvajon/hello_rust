fn main() {
    variables();
    data_types();
    operations();
    statements();
    functions();
    control_flow();
    loops();
}

fn variables() {
    // mutable vs immutable
    let mut mutable = 5;
    println!("mutable: {}", mutable);
    mutable = 6;
    println!("mutable: {}", mutable);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // shadowing
    let shadowed = 5;
    let shadowed = shadowed + 1;
    let shadowed = shadowed * 2;
    println!("shadowed: {}", shadowed);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}

fn data_types() {
    // must annotate parsed types
    let _not_inferred: u32 = "42".parse().expect("Not a number!");
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    
    // char
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_, second_member, _) = tuple;
    println!("second_member: {}", second_member);

    let _five_hundred = tuple.0;
    let _six_point_four = tuple.1;
    let _one = tuple.2;

    // arrays
    let nbs = [1, 2, 3, 4, 5];
    let _first = nbs[0];
    let _second = nbs[1];
}

fn operations() {
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;
}

fn statements() {
    let foo = {
        let bar = 3;
        bar + 1
    };

    println!("foo: {}", foo);
}

fn functions() {
    // parameters
    with_params(32);

    // return values
    let five_returns = five();
    println!("five_returns: {}", five_returns);

    // return values
    let five_plus_one = plus_one(five());
    println!("five_plus_one: {}", five_plus_one);
}

fn with_params(limit: i32) {
    println!("limit: {}", limit);
}

fn five() -> i32 {
    // no semicolon to return values
    5
}

fn plus_one(nb: i32) -> i32 {
    nb + 1
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("the number is less than 5");
    } else {
        println!("the number is greater or equal to 5");
    }

    if number != 1 {
        println!("the number was not 1");
    }

    if number % 4 == 0 {
        println!("the number is a multiple of 4");
    } else if number % 3 == 0 {
        println!("the number is a multiple of 3");
    } else if number % 2 == 0 {
        println!("the number is a multiple of 2");
    }

    let maximum = 5;
    let input = 17;
    let value = if input > maximum {
        0
    } else {
        input
    };
    println!("value: {}", value);
}

fn loops() {
    // loop == while true
    let mut cpt = 0;
    loop {
        println!("loop: again!");
        if cpt > 2 {
            break
        }
        cpt = cpt + 1;
    }
    
    let scores = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("while: score: {}", scores[index]);
        index = index + 1;
    }

    for element in scores.iter() {
        println!("for: score: {}", element);
    }

    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
