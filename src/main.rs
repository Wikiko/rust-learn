use std::collections::HashMap;

fn average(numbers: &Vec<u32>) -> f32 {
    numbers.iter().sum::<u32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &Vec<u32>) -> u32 {
    let mut frequencie = HashMap::new();
    for number in numbers {
        let count = frequencie.entry(number).or_insert(0);
        *count += 1;
    }
    *frequencie
        .values()
        .max()
        .expect("Cannot compute the mode of zero numbers")
}

fn is_vogal(string: &str) -> bool {
    let vogals = ["a", "e", "i", "o", "u"];
    vogals.iter().any(|&x| x == string)
}

fn pig_latin(string: &str) -> String {
    match string.get(0..1).map(|first_letter| first_letter.to_lowercase()) {
        None => String::from("string invalida"),
        Some(first_letter) => {
                if is_vogal(&first_letter) {
                    format!("{}-{}", string, "hay")
                } else {
                    format!("{}{}-{}", &string[1..], first_letter, "ay")
                }
        }
    }
}

fn to_uppercase(text: &str) -> String {
    text.to_string()
}

fn main() {
    let mut integers = vec![1,2,3,4,5,6,7,8,9,10];
    let mut map_integers = HashMap::new();

    map_integers.insert("media", average(&integers));
    map_integers.insert("median", median(&mut integers) as f32);
    map_integers.insert("mode", mode(&integers) as f32);

    println!("{:#?}", map_integers);

    println!("{}", pig_latin(""));
    println!("{}", pig_latin("apple"));

    let teste = String::from("teste");
    println!("{}", to_uppercase(&teste));

}
