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

fn main() {
    let mut integers = vec![1,2,3,4,5,6,7,8,9,10];
    let mut map_integers = HashMap::new();

    map_integers.insert("media", average(&integers));
    map_integers.insert("median", median(&mut integers) as f32);
    map_integers.insert("mode", mode(&integers) as f32);

    println!("{:#?}", map_integers);

}
