use anyhow::Result;
mod helpers;
use helpers::parse_file;
fn main() -> Result<()>{
    let lines = parse_file("input")?
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut temp_vec: Vec<char> = Vec::new();
    let columns = lines[0].len();
    let rows = lines.len();

    // naive way to transpose "lines" vector

    // iterate over each column
    for col in 0..columns {
        // iterate over each row
        for row in 0..rows {
            temp_vec.push(lines[row][col]);
        }
    }

    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();

    // transpose initial vector
    for window in temp_vec.windows(rows).step_by(rows) {
        let count_ones = window.iter().filter(|&c| *c == '1').count();
        let count_zeros = rows - count_ones;
        if count_ones > count_zeros {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("Gamma => {:?}", gamma);
    println!("Epsilon => {:?}", epsilon);

    // parse vector as a str and then as a binary value
    let gamma_value = isize::from_str_radix(gamma.iter().collect::<String>().as_str(), 2)?;
    let epsilon_value = isize::from_str_radix(epsilon.iter().collect::<String>().as_str(), 2)?;
    let mul = gamma_value * epsilon_value; 
    println!("{}", mul);

    Ok(())
}
