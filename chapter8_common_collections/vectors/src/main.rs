fn main() {
    declaration();
    scopes();
    read();
    borrow();
    iterating();
    multiple_types();
}

fn declaration() {
    let _v: Vec<i32> = Vec::new();
    // macro
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(5);
    v.push(9);
}

fn scopes() {
    let _v = vec![1, 2];
} // <- v is out of scope, its content aswell

fn read() {
    let v = vec![1, 2, 3, 4, 5];
    // get the ref
    let _third: &i32 = &v[2];
    // play it safe
    let _third: Option<&i32> = v.get(2);
    // will panic
    // let _does_not_exist = &v[100];
}

fn borrow() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // this is an immutable
    // let _first = &v[0];
    // can't get a mutable borrow on the same reference (v. borrows v)
    // v.push(6);
}

fn iterating() {
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    println!("");

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    println!("");
}

fn multiple_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Float(val) => {
                let q = val / 7 as f64;
                println!("{}", q);
            }
            SpreadsheetCell::Text(val) => println!("{}", val),
            SpreadsheetCell::Int(val) => println!("{}", val * val)
        }
    }
}