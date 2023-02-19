use rand::seq::SliceRandom;

pub fn get_word(path: &str) -> Option<String> {
    let words = std::fs::read_to_string(path).ok()?;
    let words = words.lines().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    let word = *words.choose(&mut rng)?;
    Some(word.trim().to_uppercase().into())
}

pub fn get_user_letter(prompt: &str) -> Option<char> {
    println!("{prompt}");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok()?;
    Some(buf.trim().to_ascii_uppercase().chars().nth(0)?)
}
