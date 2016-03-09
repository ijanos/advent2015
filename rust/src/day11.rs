use itertools::Zip;
use itertools::Itertools;

fn inc_char(c: char) -> char {
    if c as u8 == 122 {
        'a'
    } else {
        (c as u8 + 1) as char
    }
}

fn increment(pw: &str) -> String {
    let mut buf = String::with_capacity(pw.len());
    let mut carry = true;
    for c in pw.chars().rev() {
        let mut c = c;
        if carry {
            c = inc_char(c);
            carry = c == 'a';
        }
        buf.push(c);
    }
    buf.chars().rev().collect()
}

fn valid(pw: &str) -> bool {
    if ['i', 'o', 'l'].iter().any(|&c| pw.contains(c)) {
        return false;
    };
    if !Zip::new((pw.chars(), pw[1..].chars(), pw[2..].chars()))
            .any(|(a, b, c)| a as u8 + 1 == b as u8 && b as u8 + 1 == c as u8) {
        return false;
    };
    let doubles = pw.chars().group_by(|&x| x).fold(0, |acc, (_, l)| {
        acc +
        match l.len() {
            0 | 1 => 0,
            _ => 1,
        }
    });
    if doubles < 2 {
        return false;
    };
    true
}

pub fn main() {
    let mut current = String::from("vzbxkghb");
    let mut i = 0;
    loop {
        current = increment(&current);
        if valid(&current) {
            i += 1;
            println!("Part {}: {}", i, current);
        }
        if i == 2 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{increment, inc_char, valid};

    #[test]
    fn increment_chars() {
        assert_eq!('b', inc_char('a'));
        assert_eq!('a', inc_char('z'));
    }

    #[test]
    fn increment_string() {
        assert_eq!("abd", &increment("abc"));
        assert_eq!("xza", &increment("xyz"));
        assert_eq!("aaa", &increment("zzz"));
    }

    #[test]
    fn valid_passwords() {
        assert_eq!(false, valid("hijklmmn"));
        assert_eq!(false, valid("abbceffg"));
        assert_eq!(false, valid("abbcegjk"));
        assert_eq!(false, valid("abcffff"));
        assert_eq!(true, valid("abcdffaa"));
        assert_eq!(true, valid("ghjaabcc"));
    }
}
