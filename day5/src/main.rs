fn main() {
    //let random_string = String::from("abcdofjamejgampu");
    /*let mut vector: Vec<char> = random_string.chars().collect();

    vector.sort();

    vector.dedup();

    for char in vector {
        println!("{}", char);
    }*/

    /*let bytes_str = random_string.as_bytes();
    for i in bytes_str {
        println!("{}", i - 96); // gets alphabet order
    }*/

    /*let result = no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string());
    println!("{}", result);*/

    // combining strings
    /*let st1 = String::from("We Have");
    let st2 = String::from(" No Money");
    
    let st3 = st1 + &st2;

    println!("{}", st3);*/

    enum Days {
        Monday,
        Tuesday,
        Wednsday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }

    let today: Days = Days::Sunday;
    let todays_state = Days::is_weekend(&Days::Saturday);

    println!("{}", todays_state);

}

/*fn no_space(x: String) -> String {
    let splitted_string: Vec<char> = x.chars().collect();
    let mut new_string = String::new();
    for i in splitted_string {
        if i != ' ' {
            new_string.push(i);
        }
    }

    return new_string;
}*/