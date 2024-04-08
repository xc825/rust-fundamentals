use std::println;

fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn vec_sum(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}
fn surround(numbers: &mut Vec<i32>, aznum: i32) {
    numbers.insert(0, aznum);
    numbers.push(aznum);
}

fn append_vec(first: &mut Vec<i32>, second: &mut Vec<i32>) {
    first.append(second);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let _last_value = vec.last().unwrap();
    //println!("The last value in the vector is: {}", last_value);

    //Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let numbers = vec![1, 2, 3, 4, 5];
    // println!("Enter a list of numbers separated by spaces");
    // let input: &mut String = &mut String::new();
    // std::io::stdin().read_line(input).expect("Failed to read line");
    // let numbers: Vec<i32> = input
    //     .split_whitespace()
    //     .filter_map(|s| s.parse().ok())
    //     .collect();

    println!("sum of numbers {:?} is {}", numbers, vec_sum(&numbers));

    let mut v = vec![1, 2, 3];
    println!("{:?}", v); // Output: [1, 2, 3]
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    surround(&mut v, 55);
    println!("{:?}", v); // Output: [55, 0, 1, 2, 3, 4, 5, 6, 7, 8, 55]

    let mut v2 = vec![9, 10];
    append_vec(&mut v, &mut v2);
    println!("{:?}", v); // Output: [55, 0, 1, 2, 3, 4, 5, 6, 7, 8, 55, 9, 10]

}
