use itertools::Itertools;

fn day10(input: &str) -> (usize, usize) {
    let mut i = 1;
    let mut next = String::from(input);
    let mut part1 = 0;
    let mut part2 = 0;
    loop {
        // worst case size is double the digits
        let mut buf = String::with_capacity(next.len() * 2);
        for (number, list) in next.chars().group_by(|&x| x) {
            buf.push_str(&list.len().to_string());
            buf.push(number);
        }
        next = buf;
        if i == 40 {
            part1 = next.len();
        }
        if i == 50 {
            part2 = next.len();
        }
        i += 1;
        if i == 51 {
            break;
        }
    }
    return (part1, part2);
}

pub fn main() {
    let (part1, part2) = day10("3113322113");
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}

#[test]
fn test() {
    let (part1, part2) = day10("3113322113");
    assert_eq!(part1, 329356);
    assert_eq!(part2, 4666278);
}
