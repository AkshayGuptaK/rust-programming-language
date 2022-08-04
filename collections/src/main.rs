use std::collections::HashMap;

fn main() {
    let ints = vec![5, 2, 4, 3, 1, 2, 4, 2, 3];
    let mode = mode(&ints);
    let median = median(ints);
    println!("The median is {} and the mode is {}", median, mode);

    let word = "word";
    let new_word = pig_latin(word);
    println!("the pig latin for {} is {}", word, new_word);
}

fn median(mut ints: Vec<i32>) -> i32 {
    let median_index = (ints.len() - 1) / 2;
    ints.sort_unstable();
    ints[median_index]
}

fn mode(ints: &[i32]) -> i32 {
    let mut freqs = HashMap::new();
    for int in ints {
        let count = freqs.entry(*int).or_insert(0);
        *count += 1;
    }

    let mut max_freq = 0;
    let mut mode = 0;
    for (int, freq) in freqs.iter() {
        if *freq > max_freq {
            max_freq = *freq;
            mode = *int;
        }
    }
    mode
}

fn pig_latin(word: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = word.chars();
    let first_letter = chars.next().unwrap();
    if VOWELS.contains(&first_letter) {
        return format!("{}hay", word);
    }
    format!("{}{}ay", chars.as_str(), first_letter)
}
