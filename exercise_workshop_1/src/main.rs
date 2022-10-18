use std::io;

pub fn is_subarray(org_arr: &[i8],sub_arr: &[i8]) -> bool {
    for i in 0..org_arr.len() {
        if org_arr[i] == sub_arr[0] {
            let mut is_subarray = true; 
            for j in 0..sub_arr.len(){
                if org_arr[i+ j] != sub_arr[j] {
                    is_subarray = false;
                    break;
                }
            }
            if is_subarray { return true}
        }
    }
    
    false
}

fn get_nth_char(n: usize, input: &str) -> char {
    input.chars().nth(n).unwrap()
}

fn main() {
    let is_subarray = is_subarray(&[1,2,3], &[2,3]);

    println!("{}", is_subarray);

    let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
    let input_string = input.trim();

    let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
    let input = input.trim();
    assert_eq!(input.len(), 1, "Must be a character");

    let char =get_nth_char(0, input);
    let mut count = 0;
    for i in 0..input_string.len() {
        if  get_nth_char(i, input_string)== char {
            count += 1;
        }
    }
    let mut input_string = input_string.to_string();
    input_string.retain(|c| c!= char);
    println!("{} {}", count, input_string );
}
