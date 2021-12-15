use anyhow::Result;
use std::fs::read_to_string;
use std::collections::HashMap;

/* TODO:
    Fish internal counter goes from 6 - 0.
    When 0, a new fish is born with timer 8 and that fish's timer goes to 6
*/

fn tick(fam: &mut HashMap<i32, i64>) {
    for (k, v) in fam.iter() {
        println!("Pre: {} -> {}", k, v);
    }

    let new_fish: i64 = *fam.get(&0).unwrap();
    println!("New fish: {}", new_fish);

    for i in 1..=8 {
        let next_value: i64 = *fam.get(&i).unwrap();
        println!("next value: {}", next_value);
        fam.insert(i - 1, next_value);
    }

    fam.insert(8, new_fish);
    let entry = fam.entry(6).or_insert(0);
    *entry += new_fish;
}

fn main() -> Result<()> {
    let input = read_to_string("input")?.parse::<String>()?;
    
    let mut fam: HashMap<i32, i64> = HashMap::new();

    // init
    for i in 0..=8 {
        fam.insert(i, 0);
    }

    for token in input.split(',') {
        let timer: i32 = token.parse()?;
        let count = fam.entry(timer).or_insert(0);
        *count += 1;
    }

    for _ in 0..256 {
        tick(&mut fam);
    }

    let total_fish: i64 = fam.values().fold(0, |x, acc| acc + x );
    println!("Total fish: {}", total_fish);

    Ok(())
}
