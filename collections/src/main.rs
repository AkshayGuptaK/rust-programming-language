use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    let ints = vec![5, 2, 4, 3, 1, 2, 4, 2, 3];
    let mode = mode(&ints);
    let median = median(ints);
    println!("The median is {} and the mode is {}", median, mode);

    let word = "word";
    let new_word = pig_latin(word);
    println!("the pig latin for {} is {}", word, new_word);

    let mut dpt_employees: HashMap<String, Vec<String>> = HashMap::new();
    let input_re = Regex::new(r"(Add (\S+) to (\S+))|(Get All)|(Get (\S+))").unwrap();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input_re.captures(&input) {
            Some(cap) => match cap.get(2) {
                Some(_) => add_employee_to_dept(
                    cap.get(2).unwrap().as_str(),
                    cap.get(3).unwrap().as_str(),
                    &mut dpt_employees,
                ),
                None => match cap.get(4) {
                    Some(_) => get_all_employees(&dpt_employees),
                    None => get_employees_in_dept(cap.get(6).unwrap().as_str(), &dpt_employees),
                },
            },
            None => println!("Please enter a valid command!"),
        }
    }
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

fn add_employee_to_dept(employee: &str, dept: &str, directory: &mut HashMap<String, Vec<String>>) {
    let dept = (*dept).to_string();
    let employee = (*employee).to_string();
    let entry = directory.entry(dept).or_insert_with(Vec::new);
    entry.push(employee);
}

fn get_employees_in_dept(dept: &str, directory: &HashMap<String, Vec<String>>) {
    match directory.get(dept) {
        Some(employees) => println!("{:?}", employees),
        None => println!("No such department."),
    }
}

fn get_all_employees(directory: &HashMap<String, Vec<String>>) {
    println!("{:?}", directory);
}
