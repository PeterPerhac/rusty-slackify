use std::env;

fn slackify(c: char) -> String {
    match c {
        letter if letter.is_alphabetic() => format!(":alphabet-white-{}:", letter),
        number if number.is_whitespace() => ":blank:".to_string(),
        other => other.to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        arg.chars().for_each(|c| print!("{}", slackify(c.to_ascii_lowercase())));
        if iter.peek().is_some() { print!(":blank:"); }
    }
    println!();
}
