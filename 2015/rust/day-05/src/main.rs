fn is_string_nice(str: &str) -> bool {
    let chars = str.chars().collect::<Vec<char>>();

    chars.windows(2).any(|c| c[0] == c[1])
        && chars
            .windows(2)
            .all(|c| !["ab", "cd", "pq", "xy"].contains(&&*c.iter().collect::<String>()))
        && chars
            .iter()
            .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
            .collect::<Vec<&char>>()
            .len()
            >= 3
}

fn is_string_nice_2(str: &str) -> bool {
    let chars = str.chars().collect::<Vec<char>>();
    // let mut pairs: HashMap<&[char], usize> = HashMap::new();
    // chars
    //     .chunks(2)
    //     .for_each(|c| *pairs.entry(c).or_default() += 1);

    // pairs.iter().any(|p| *p.1 >= 2) && chars.windows(3).any(|c| c[0] == c[2])
    chars.windows(3).any(|c| c[0] == c[2])
        && chars.windows(2).enumerate().any(|(i, pair)| {
            str.rfind(&&*pair.iter().collect::<String>())
                .map(|index| index > i + 1)
                .unwrap_or(false)
        })
}

fn main() {
    let file = std::fs::read_to_string("data.txt").unwrap();
    let lines = file.trim_end().split("\n").collect::<Vec<&str>>();

    println!(
        "Part 1: {}",
        lines.iter().filter(|l| is_string_nice(l)).count()
    );

    println!(
        "Part 2: {}",
        lines.iter().filter(|l| is_string_nice_2(l)).count()
    );
}
