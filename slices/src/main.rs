fn main() {
    string_slices();
    zero_index();
}

fn string_slices() {
    let s = String::from("hello world");
    let _hello = &s[0..5]; // from index 0 (included) to 5 (not included)
    let _world = &s[6..11];

    // _hello is a pointer to the same address as s, but has a len of 5
    // where s has a len of 11

    // The same goes for _world: pointer to the address s references + 6
    // with a len of 5
}

fn zero_index() {
    let s = String::from("hello");
    let _slice = &s[0..2]; // same
    let _slice = &s[..2]; // same
}
