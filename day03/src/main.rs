use anyhow::Result;
mod helpers;
use helpers::parse_file;

fn process_file() -> Result<Vec<Vec<char>>> {
    let lines = parse_file("input")?
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok(lines)
}

fn main() -> Result<()>{
    let lines = process_file()?;
    
    let mut temp_vec: Vec<char> = Vec::new();
    let columns = lines[0].len();
    let rows = lines.len();

    // naive way to transpose "lines" vector

    // iterate over each column
    for col in 0..columns {
        // iterate over each row
        for line in lines.iter() {
            temp_vec.push(line[col]);
        }
    }

    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();
    let half = rows / 2;
    
    // transpose initial vector
    for window in temp_vec.windows(rows).step_by(rows) {
        // majority element if present more than half of the elements
        let most_common = window.iter().filter(|&c| *c == '1').count() > half;
        if most_common {
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
