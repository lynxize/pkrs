pub fn get_input(prompt: &str) -> String {
    let mut inp = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
    inp
}
