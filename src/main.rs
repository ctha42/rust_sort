use std::fs;
use std::path::Path;
use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file location
    #[arg(short, long)]
    file: String,
    /// Output file location
    #[arg(short, long)]
    output: String,
    /// Sort in decending order
    #[arg(short, long)]
    decending: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args = Args::parse(); 

    if !Path::new(&args.file).exists(){
        return Err(format!("File '{}' does not exist", &args.file).into());
    }

    let input = fs::read_to_string(args.file)?;
    let length = input.len() as isize;
    let mut unsorted_nums = parse_input(input, length);

    let nums = quick_sort(&mut unsorted_nums, args.decending);
    
    let mut file = File::create(args.output)?;
    
    for i in &nums{                                                                                                                                                                  
        write!(file, "{}, ", i)?;                                                                                                                            
    }                                                                                                                                                                                

    Ok(())
}

fn parse_input(input: String, len: isize) -> Vec<isize>{
    let mut vec = Vec::new();
    let mut iter = input.split(|c| c == ',' || c == ' ');

    for _ in 0..len {
        if let Some(num_str) = iter.next() {
            if let Ok(num) = num_str.trim().parse::<isize>(){
                vec.push(num);
            }
        } 
        else {
            break
        }
    }

    for num_str in iter {
        if let Ok(num) = num_str.trim().parse::<isize>(){
            vec.push(num);
        }
    }
    vec
}

fn quick_sort(vec: &mut Vec<isize>, dec: bool) -> Vec<isize> {
    
    if vec.len() <= 1 {
        return vec.to_vec();
    }
    
    let pivot = vec.pop().unwrap();
    let mut left = vec![];
    let mut right = vec![];

    for x in vec {
        if *x <= pivot {
            left.push(*x);
        } else {
            right.push(*x);
        }
    }

    let mut sorted = vec![];
    
    if dec{
        sorted.extend(quick_sort(&mut right, dec));
        sorted.push(pivot);
        sorted.extend(quick_sort(&mut left, dec));
    }
    else{
        sorted.extend(quick_sort(&mut left, dec));
        sorted.push(pivot);
        sorted.extend(quick_sort(&mut right, dec));
    }
    sorted
}