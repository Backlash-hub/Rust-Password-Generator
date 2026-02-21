use passwords::PasswordGenerator;

fn gen_password(length: usize) -> String {
    let password = PasswordGenerator {
        length,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true
    };
    password.generate_one().expect("Failed to generate password")
}

fn get_input() -> usize {
    let mut input = String::new();
    println!("Please enter desired password length: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let length: usize = input.trim().parse().expect("Failed to parse input");
    length
}

fn main() {
    let length: usize = get_input();
    let password = gen_password(length);
    println!("Password: {}", password);


}
