use anyhow::Result;

mod helpers;
use helpers::parse_file;

pub fn part_a() -> Result<u32> {
    let total_a = parse_file("input")?
        .filter_map(|ln| {
            ln.parse::<u32>().ok()
        })
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |acc, w| acc + (w[1] > w[0]) as u32);
    Ok(total_a)
}

pub fn part_b() -> Result<u32> {
    let total_b = parse_file("input")?
        .filter_map(|ln| {
            ln.parse::<u32>().ok()
        })
        .collect::<Vec<u32>>()
        .windows(4)
        .fold(0, |acc, w| acc + (w[0..3].iter().sum::<u32>() < w[1..4].iter().sum::<u32>()) as u32);
    Ok(total_b)
}

fn main() -> Result<()> {
    // count the number of times a depth measurement increases from the previous measurement.
    println!("[A] Found: {}", part_a()?);
    println!("[B] Found: {}", part_b()?);
    Ok(())
}
