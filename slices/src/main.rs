fn main() {
    string_slices();
    zero_index();
    len_index();
    full_size();
    substring();
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
    let slice = &s[..2]; // same
    println!("{}", slice); // he
}

fn len_index() {
    let s = String::from("hello");

    let len = s.len();
    let _slice = &s[2..len]; // same
    let slice = &s[2..]; // same
    println!("{}", slice); // llo
}

fn full_size() {
    let s = String::from("hello");

    let len = s.len();
    let _slice = &s[0..len]; // same
    let slice = &s[..]; // same
    println!("{}", slice); // hello
}

fn substring() {
    let hw = String::from("hello world");
    let fw = first_word(&hw);
    let sw = second_word(&hw);
    println!("first word: {}", fw);
    println!("second word: {}", sw);

    let string = String::from("bonjour monde");
    let literal = "bonjour monde";
    let fw_string = first_word_slices(&string[..]);
    let fw_literal_slice = first_word_slices(&literal[..]);
    let fw_literal = first_word_slices(literal); // already a slice !
    println!("first word: {} = {} = {}", fw_string, fw_literal_slice, fw_literal);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            return &s[i+1..] // second word onwards, not just the 2nd word
        }
    }

    &s[..]
}

fn first_word_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
