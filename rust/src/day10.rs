use itertools::Itertools;


pub fn main() {
    let mut i = 1;
    let mut next = "3113322113".to_owned();
    let mut part1 = 0;
    let mut part2 = 0;
    loop {
        next = next.chars()
                   .group_by(|&x| x)
                   .map(|(n, l)| {
                       let mut s = l.len().to_string();
                       s.push(n);
                       s
                   })
                   .collect::<String>();
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
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
