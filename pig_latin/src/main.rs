const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

fn main() {
    let s = "What a beautiful day";
    let mut result = String::new();
    for word in s.to_lowercase().split_whitespace() {
        let mut i = 0;
        let mut ending_consonant = 'h';
        for c in word.chars() {
            if i == 0 {
                if CONSONANTS.contains(c) {
                    ending_consonant = c;
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
            i += 1;
        }
        result.push_str(&*format!("-{}ay ",ending_consonant));
    }
    println!("{result}");
}
