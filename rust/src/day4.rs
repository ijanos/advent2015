use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "bgvyzdsv";

fn first_n_zeroes(zeroes: usize) -> usize {
    let mut md5hasher = Md5::new();
    (0..)
        .find(|&i| {
            md5hasher.reset();
            md5hasher.input_str(&format!("{}{}", INPUT, i));
            md5hasher.result_str().chars().take(zeroes).all(|c| c == '0')
        })
        .unwrap()
}

pub fn main() {
    println!("Part 1: {}", first_n_zeroes(5));
    println!("Part 2: {}", first_n_zeroes(6));
}

#[test]
fn test() {
    assert_eq!(first_n_zeroes(5), 254575);
    assert_eq!(first_n_zeroes(6), 1038736);
}
