fn main() {
    fn a() {
        let _x: &str = "hello";
        let _y: i32 = 22;
        b();
    }

    fn b() {
        let _x: String = String::from("world");
    }

    // ------- Ownership rules -------
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        let _s = String::from("hello"); // s is valid from this point forward
                                        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // ------- Move -------
    {
        let _s1 = String::from("hello");
        let _s2 = _s1;
        // println!("{}, world!", s1); // error[E0382]: borrow of moved value: `s1`
    }

    // ------- Clone -------
    {
        let _s1 = String::from("hello");
        let _s2 = _s1.clone();
        println!("s1 = {}, s2 = {}", _s1, _s2);
    }

    // ------- take ownership -------
    {
        let _s = String::from("hello"); // s comes into scope
        take_ownership(_s); // s's value moves into the function...
                            // ... and so is no longer valid here

        // println!("{}", s); // error[E0382]: borrow of moved value: `s`
    }

    // ------- copy -------
    {
        let _x = 5; // x comes into scope
        make_copy(_x); // x would move into the function,
                       // but i32 is Copy, so it’s okay to still
                       // use x afterward
        println!("{}", _x);
    }

    // ------- gives ownership -------
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1
        let _s2 = String::from("hello"); // s2 comes into scope
        let _s3 = takes_and_gives_back(_s2); // s2 is moved into
                                             // takes_and_gives_back, which also
                                             // moves its return value into s3
    }

    // ------ Calculate length ------
    {
        let _s1 = String::from("hello");
        let (_s2, len) = calculate_length(_s1);
        println!("The length of '{}' is {}.", _s2, len);

        // instaead of returning tuple to get the ownership back, we can use reference.
        // See calculate_length_ref() function.
    }

    // ------ Mutable reference ------
    {
        mutable_reference();
    }

    // ------ Dangling reference ------
    {
        // let reference_to_nothing = dangling_reference(); // error[E0106]: missing lifetime specifier
    }

    // ------ Slice ------
    {
        slice();
    }
}

fn take_ownership(some_string: String) {
    // After this function is called, some_string will go out of scope and drop.
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    // instead of move, some_integer will be copied.
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn mutable_reference() {
    let mut s = String::from("hello");

    // there can be as many immutable references as we want.
    let _r1 = &s;
    let _r2 = &s;
    // let _r3 = &mut s; // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{} and {}", _r1, _r2);

    // now we can have one mutable reference as _r1 and _r2 are out of scope.

    let _r3 = &mut s;
}

// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn slice() {
    let mut s = String::from("hello world");
    let _first_word = first_word(&s);
    s.clear();
    println!("{}", _first_word); // this is a problem as _first_word is 5 but s is cleared.

    // ------- String slice -------
    let mut s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    let _hello_world = &s[..];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // iter() returns each element in a collection
        if item == b' ' {
            // b' ' is byte literal
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
