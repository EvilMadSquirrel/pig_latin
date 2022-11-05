fn main() {
    let src = "fello earth";
    let pig = convert_to_pig_latin(&src);
    println!("{pig}")
}

fn convert_to_pig_latin(s: &str) -> String {
    let mut pig_latin = Vec::new();
    let vovels = ['a', 'e', 'i', 'o', 'u'];
    let words = s.split_whitespace();
    for word in words {
        let mut pig_word = String::new();
        if word.starts_with(&vovels) {
            pig_word.push_str(word);
            pig_word.push_str("-hay");
            pig_latin.push(pig_word);
        } else {
            let first_char = word.chars().nth(0).unwrap().to_string();
            let other_part = String::from(word).replace(&first_char, "");
            pig_word = format!("{}-{}ay", other_part, first_char);
            pig_latin.push(pig_word);
        }
    }
    pig_latin.join(" ")
}
