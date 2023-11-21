use rand::Rng;

fn main() {
    let random_num = rand::thread_rng().gen_range(0..101);
    println!("age: {}", random_num);
    match random_num {
        1..=17 => println!("Minor"),
        18..=25 => println!("Adult"),
        26..=60 => println!("old"),
        _ => println!("prolly dead")
    }
}
