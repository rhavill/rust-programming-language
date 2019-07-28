use std::collections::HashMap;
use std::io;

fn main() {
    let v = vec![10, 9, 8, 7, 6, 5, 4, 4, 4, 4, 3, 3, 3, 3, 2, 1];
    println!("In summary, v is: {:?}", v);
    println!("In summary, mean is: {}", mean(&v));
    println!("In summary, median is: {}", median(&v));
    println!("In summary, mode is: {:?}", mode(&v));
    let text = String::from("Hello there, Alberta.");
    println!("In summary, pig latin for '{}' is '{}'", text, pig_latin(&text));
    employee_departments();
}

fn mean(xs: &Vec<i32>) -> f32 {
    let mut total: f32 = 0.0;
    let mut count = 0;
    let mut mean = 0.0;
    for x in xs {
        total = total + *x as f32;
        count = count + 1;
    }
    if count > 0 {
        mean = total / count as f32;
    }
    mean
}

fn median(xs: &Vec<i32>) -> f32 {
    let mut median = 0.0;
    let length = xs.len();
    if  length > 0 {
        let mut ys = xs.clone();
        ys.sort();
        let middle = length / 2;
        let has_even_length = is_even(ys.len() as i32);
        if has_even_length {
            let a: f32 = ys[middle] as f32;
            let b: f32 = ys[middle - 1] as f32;
            median = (a + b) / 2.0
        }
        else {
            median = ys[middle - 1] as f32;
        }
    }
    median
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn mode(xs: &Vec<i32>) -> Vec<i32> {
    let length = xs.len();
    let mut modes: Vec<i32> = Vec::new();
    let mut counts = HashMap::new();
    let mut max_count = 0;
    // If every element occurs the same number of times, there is no mode.
    let mut mode_exists = false;
    
    if length > 0 { 
        for x in xs {
            let count = counts.entry(x).or_insert(0);
            *count += 1;
            if *count > max_count {
                max_count = *count;
            }
        }
        for (element, count) in &counts {
            if *count < max_count {
                mode_exists = true;
            }
            else if *count == max_count {
                modes.push(**element);
            }
        }
        if !mode_exists {
            modes = [].to_vec();
        }
    }
    modes
}

fn pig_latin(s: &String) -> String {
    let mut pig_latin = String::new();
    let words = s.split_whitespace();
    let mut word_count = 0;
    let total_words = s.split_whitespace().count();
    for word in words {
        word_count += 1;
        let mut count = 0;
        let length = word.len();
        let characters = word.chars();
        let mut starts_with_vowel = false;
        let mut first_letter = ' ';
        for c in  characters {
            count += 1;
            let is_second_letter_in_sentence = count == 2 && word_count == 1;
            if count == 1 {
                first_letter = c;
                if is_vowel(&c) {
                    starts_with_vowel = true;
                    pig_latin.push(c);
                }
                else {
                    first_letter.make_ascii_lowercase();
                }
            }
            else if count == length {
                let is_letter = c.is_alphabetic();
                if is_letter {
                    pig_latin.push(c);
                }
                pig_latin.push('-');
                if starts_with_vowel {
                    pig_latin += &String::from("hay");
                }
                else {
                    pig_latin.push(first_letter);
                    pig_latin += &String::from("ay");
                }
                if !is_letter {
                    pig_latin.push(c);
                }
            }
            else {
                if is_second_letter_in_sentence && !starts_with_vowel {
                    pig_latin.push(c.to_ascii_uppercase());
                }
                else {
                    pig_latin.push(c);
                }
            }
        }
        if word_count < total_words as i32{
            pig_latin.push(' ');
        }
    }
    pig_latin
}

fn is_vowel(c: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut is_vowel = false;
    for vowel in &vowels {
        if c == vowel {
            is_vowel = true;
            break;
        }
    }
    is_vowel
}

fn employee_departments() {
    println!("Employee Departments");
    println!("To show employees, type \"show\".");
    println!("To add an employee, type \"add {{employee}} to {{department}}\".");
    let mut employees = HashMap::new();
    employees.insert(String::from("HR"), vec![String::from("Alice"), String::from("Bob")]);
    loop {

        let text = get_input();

        if text.is_empty() {
            break;
        }

        let mut text = text.split_whitespace();

        let first_word = text.next();
        let second_word = text.next();
        text.next();
        let fourth_word = text.next();

        if first_word.is_none() || second_word.is_none() {
            break;   
        }
        
        match first_word.unwrap() {
            "show" => {
                match second_word.unwrap() {
                    "all" => show_all(&employees),
                    _ => show(&employees, second_word.unwrap()),
                }
            },
            "add" => {
                if fourth_word.is_none() {
                    break;   
                }
                add_employee(&mut employees, second_word.unwrap(), fourth_word.unwrap());
            },
            _ => break,
        }
    }
}

fn get_input() -> String {
    println!("Enter a command or hit <Enter> to exit:");
    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Failed to read line");
    text.trim().to_string()
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>, emp: &str, dep: &str) {
    let department_key = dep.to_string();
    let employee = emp.to_string();
    employees.entry(department_key).or_insert(Vec::new()).push(employee);
}

fn show_all(employees: &HashMap<String, Vec<String>>) {
    for (dep, emps) in employees {
        println!("Department: {}", dep);
        for emp in emps {
            println!("Employee: {}", emp);
        }
        println!("");
    }
}

fn show(employees: &HashMap<String, Vec<String>>, dep: &str) {
    let emps = employees.get(dep);
    println!("Department: {}", dep);
    if emps.is_some() {
        for emp in emps.unwrap() {
            println!("Employee: {}", emp);
        }
    }
    println!("");
}