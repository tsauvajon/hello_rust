fn main() {
    with_variables();
    with_funcs();
    with_returns();
    references();
}

fn with_variables() {
    let s1 = String::from("hello");
    let _s2 = s1; // s1 moved to s2

    println!("{}, world!", s1);  // error: s1 is now invalid

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy

    println!("s1 = {}, s2 = {}", s1, s2); // no problem

    let x = 5;
    let y = x;

    // no problem with the `Copy` trait, which is present on scalar types
    println!("x = {}, y = {}", x, y);
}

fn with_funcs() {
    // s comes into scope
    let s = String::from("hello");
    
    // s's value moves into the function...
    // ... and so is no longer valid here
    takes_ownership(s);

    // x comes into scope
    let x = 5;

    // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    makes_copy(x);
} // Here, x goes out of scope, then s.
  // But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn with_returns() {
    // gives_ownership moves its return
    // value into s1
    let _s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
    let _s3 = takes_and_gives_back(s2);  
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

// gives_ownership will move its
// return value into the function
// that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("hello");

    // some_string is returned and
    // moves out to the calling
    // function.
    some_string
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    // a_string is returned and moves out to the calling function
    a_string
} // in the `with_returns` case, s2 is passed as parameters and the return
  // is moved to s3. Therefore, s2 is dropped here - nothing would be dropped
  // if we did `s2 = takes_and_gives_back(s2);` instead

fn references() {
    let s1 = String::from("hello");
    let _len = calculate_length(&s1);
    
    let s1 = String::from("hello");
    change_is_impossible(&s1);

    let mut s = String::from("hello");
    change_is_possible(&mut s);


    // Mutable borrows: NOK
    let mut s = String::from("hello");
    let _r1 = &mut s;
    let r2 = &mut s; // error: only 1 mutable borrow at the time

    // Mutable borrows: OK
    let mut s = String::from("hello");
    {
        let _r3 = &mut s;
    } // r3 goes out of scope here, so we can make a new reference with no problems.
    let _r4 = &mut s;

    // Mix of mutable and immutable borrows: NOK
    let mut s = String::from("hello");
    let _r5 = &s; // no problem
    let _r6 = &s; // no problem
    let _r7 = &mut s; // BIG PROBLEM
}

// using references (pointers) to the variables will not take ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_is_impossible(some_string: &String) {
    // error: cannot change the value of a variable passed by reference
    some_string.push_str(", world");
}

fn change_is_possible(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
