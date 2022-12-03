use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .collect::<Vec<char>>();

    let letter_weights = (1..=52).into_iter().collect::<Vec<u32>>();

    let letters_zip: Vec<(&char, u32)> = letters.iter().zip(letter_weights).collect();
    let letters_map: HashMap<&char, u32> = letters_zip.into_iter().collect();

    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let rucksacks: Vec<&str> = file.trim().split("\n").collect();

    println!(
        "Sum of priorities: {:?}",
        rucksacks.iter().fold(0, |mut acc, sack| {
            let (first_c, second_c) = sack.split_at(sack.len() / 2);
            let first_c_items: Vec<char> = first_c.chars().collect();
            let second_c_items: Vec<char> = second_c.chars().collect();
            let mut shared: Vec<char> = vec![];

            for item in second_c_items {
                if first_c_items.contains(&item) && !shared.contains(&item) {
                    shared.push(item);

                    let item_res = match letters_map.get(&item) {
                        Some(&item) => item,
                        None => 0,
                    };
                    acc += item_res;
                }
            }
            acc
        })
    );

    let mut sum = 0;

    for group in rucksacks.chunks_exact(3) {
        let mut all_inters: Vec<&&char> = vec![];
        let group_hash_1: HashSet<char> =
            HashSet::from_iter(group[0].chars().collect::<Vec<char>>());

        let group_hash_2: HashSet<char> =
            HashSet::from_iter(group[1].chars().collect::<Vec<char>>());

        let group_hash_3: HashSet<char> =
            HashSet::from_iter(group[2].chars().collect::<Vec<char>>());

        let intersection_1_2: HashSet<_> = group_hash_1.intersection(&group_hash_2).collect();
        let intersection_1_3: HashSet<_> = group_hash_1.intersection(&group_hash_3).collect();
        let intersection_2_3: HashSet<_> = group_hash_2.intersection(&group_hash_3).collect();

        let mut intersection_1_2: Vec<&&char> = intersection_1_2.iter().collect();
        let mut intersection_1_3: Vec<&&char> = intersection_1_3.iter().collect();
        let mut intersection_2_3: Vec<&&char> = intersection_2_3.iter().collect();

        all_inters.append(&mut intersection_1_2);
        all_inters.append(&mut intersection_1_3);
        all_inters.append(&mut intersection_2_3);

        let mut occur: HashMap<&&char, usize> = HashMap::new();
        for ch in all_inters {
            if occur.get(ch) == None {
                occur.insert(ch, 1);
            } else {
                *occur.get_mut(ch).unwrap() += 1;
            }
        }

        for occ in occur {
            if occ.1 == 3 {
                let chh = occ.0;
                let letter = match letters_map.get(chh) {
                    None => 0,
                    Some(chh) => *chh,
                };

                sum += letter;

                break;
            }
        }
    }
    println!("Sum of badge priorities: {}", sum);
}
