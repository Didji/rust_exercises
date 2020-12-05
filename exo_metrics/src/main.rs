use std::collections::HashMap;

fn main() {
    // Given lists of integers, use a vector
    let int_list_1 = vec![9,24,3,6,10,22,19,7,14,14];
    let int_list_2 = vec![4,9,4,10,3];
    let int_list_3 = vec![2,15,22,12,19,15,23,16,26,25,26,11,21];
    let int_list_4 = vec![27,49,68,86,42,63,41,76,50,49,95,6,74,98,15,88,80,15];
    let int_list_5 = vec![44,38,89,69,54,85,54,26,91,55,33,72,82,43,7,29,20,10,47,82,80,65,40,71,52];    
    let int_list_6 = vec![1,1,2,4,5,6,7,7,7,8,8,10,14,17,19,20,20,22,28,28,29,29,31,32,32,33,38,40,41,43,44,44,48,48,49,50,50,51,51,57,59];

    calculate_metrics(int_list_1);
    calculate_metrics(int_list_2);
    calculate_metrics(int_list_3);
    calculate_metrics(int_list_4);
    calculate_metrics(int_list_5);
    calculate_metrics(int_list_6);
}

fn calculate_metrics(mut numbers: Vec<i32>) {

    numbers.sort_unstable();
    //println!("{:?}", numbers);

    // return the mean (average value)
    let mean = calculate_mean(&numbers);
    println!("{}", format!("The mean is {:.2}", mean));

    // return the median (when sorted, the value in the middle position)
    let median = calculate_median(&numbers);
    if let Some(median) = median {
        println!("The median is {}", median);
    }

    // return the mode (the value that occurs the most often)
    let mode = calculate_mode(&numbers);
    if let Some(mode) = mode {
        println!("The mode is {} with {} occurrences", mode.0, mode.1);
    }
    println!("_________________________________________________");
}
fn calculate_mean(numbers: &Vec<i32>) -> f64 {    
    let size = numbers.len() as f64;
    let sum = numbers.iter().sum::<i32>() as f64;
    
    sum / size
}

fn calculate_median(numbers: &Vec<i32>) -> Option<i32> {
    let middle_index = (numbers.len() as f64 / 2_f64).round() as usize - 1;

    numbers.get(middle_index).cloned()
}

fn calculate_mode(numbers: &Vec<i32>) -> Option<(i32, i32)> {
    let mut int_count = HashMap::new();
    for int in numbers {
        let counter = int_count.entry(*int).or_insert(0);
        *counter += 1;
    }

    int_count.into_iter().max_by(|a, b| a.1.cmp(&b.1))
}