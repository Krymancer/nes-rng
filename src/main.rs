fn random(mut seed: u16) -> u16 {
    let lsb = (seed & 1) == 1;
    seed = seed >> 1;
    if lsb {
        seed = seed ^ 0xB400;
    }
    seed
}

fn d20(mut seed: u16) -> (u16, u16) {
    let mut number;
    loop {
        seed = random(seed);
        number = seed & 0b00011111;
        if number < 20 {
            break;
        };
    }
    (number + 1, seed)
}

fn main() {
    let mut seed: u16 = std::process::id()
        .try_into()
        .expect("Get seed from process id");
    let mut number;
    for _ in 1..=20 {
        (number, seed) = d20(seed);
        println!("You rolled a {}, and the new seed is: {}", number, seed);
    }
}
