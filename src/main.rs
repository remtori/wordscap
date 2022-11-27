use std::collections::HashMap;

static RAW_WORDS: &'static str = include_str!("../processed.txt");

fn main() {
    let args: String = std::env::args()
        .skip(1)
        .map(|word| word.to_ascii_lowercase())
        .collect();

    let words: Vec<&'static str> = RAW_WORDS.split('\n').collect();

    let word_count = &mut HashMap::new();
    for c in args.chars() {
        *word_count.entry(c).or_insert(0) += 1usize;
    }

    let out = &mut Vec::new();
    'word: for word in words {
        let word_count = &mut word_count.clone();
        for c in word.chars() {
            if let Some(count) = word_count.get_mut(&c) {
                if *count == 0 {
                    continue 'word;
                }

                *count -= 1;
            } else {
                continue 'word;
            }
        }

        out.push(word);
    }

    out.sort();

    println!("{out:#?}");
}
