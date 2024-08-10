use std::env;
use std::process;
use comma_seq::FirstDigitCalculator;
/*
use std::fs::File;
use std::io::{BufWriter, Write};
*/


fn last_digit(n: u64) -> u64 {
    n % 10
}

/*
fn write_vec_to_file(vec: &Vec<u64>, filename: &str) -> std::io::Result<()> {
    // Open the file in write mode
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Iterate over the vector and write each element to the file
    for &num in vec.iter() {
        writeln!(writer, "{}", num)?; // Write each number followed by a newline
    }

    Ok(())
}
*/

fn generate_sequence(first_term: u64, print_flg:bool) -> u64 {
    //let mut sequence = Vec::new();
    let mut terms: u64 = 1;
    let mut m: u64 = first_term;
    let mut i: u64 = first_term+1;    
    let mut first_digit_calc = FirstDigitCalculator::new();
    
    //sequence.push(m);
    if print_flg{
        println!("{m}");
    }
    let mut left = last_digit(m) * 10;
    let mut diff:u64 = 0;
    while diff < 100 {
        i += 1;
        diff = i - m;
        if diff == left + first_digit_calc.first_digit(i) {
            //sequence.push(i);
            terms += 1;
            m = i;
            if print_flg{
                println!("{m}");
            }
            left = last_digit(m) * 10;
            i += left;
        }
    }

    terms
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure that at least one argument (the integer) is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <integer>", args[0]);
        process::exit(1);
    }

    let first_term: u64;

    // Parse the argument as an integer
    let input = &args[1];
    match input.parse::<u64>() {
        Ok(num) => {
            //println!("Starting the Comma Sequence with {}", num);
            first_term = num;
        }
        Err(_) => {
            eprintln!("Error: '{}' is not a valid integer", input);
            process::exit(1);
        }
    }

    let print = args.contains(&"--print".to_string());

    let terms = generate_sequence(first_term, print);
    if !print{
        println!("The Comma sequence starting with {} has {} terms.", first_term, terms)
    }

}