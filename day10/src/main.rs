use anyhow::Result;
use std::fs::read_to_string;
use std::collections::HashMap;

fn is_corrupted(line: &str, mappings: &HashMap<char, char>) -> (char, bool) {

    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if let Some(&mapped) = mappings.get(&c) {
            stack.push(mapped);
        } else if mappings.values().any(|&x| x == c) && stack.pop() != Some(c) {
            return (c, true);
        }
    }
    ('X', false)
}

fn main() -> Result<()> {
    let mappings: HashMap<char, char> = [('(' ,')'), ('[',']'), ('{','}'), ('<','>')].into_iter().collect::<HashMap<char, char>>();
    let scores: HashMap<char, i32> = [('X', 0),(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect::<HashMap<char, i32>>();
    
    let input = read_to_string("input")?.parse::<String>()?;

    
    let score = input.split('\n')
                        .map(|ln| is_corrupted(ln, &mappings))
                        .filter(|(_, y)| *y )
                        .fold(0, |acc, (x, _)| acc + scores.get(&x).or(Some(&0)).unwrap());

    println!("Score: {}", score);

    Ok(())
}
