use anyhow::Result;
mod helpers;
mod submarine;
use helpers::parse_file;
use submarine::Submarine;


fn process_file() -> Result<impl Iterator<Item = Vec<String>>> {
    let lines = parse_file("input")?
        .map(|line| { 
            line.split(' ')
                .map(|e| e.to_string())
                .collect()
        });
    Ok(lines)
}

fn part_a() -> Result<Submarine> {
    let lines = process_file()?;
    let mut subm = Submarine::new(0, 0, None);
    for line in lines {
        let command = line[0].as_str();
        let displacement = line[1].parse::<i64>()?;
        subm = match command {
            "up" => { subm.change_location(0, -displacement, None) },
            "down" => { subm.change_location(0, displacement, None) },
            "forward" => { subm.change_location(displacement, 0, None) },
            _ => panic!("Erroneous command")
        };
    }
    Ok(subm)
}

fn part_b() -> Result<Submarine> {
    let lines = process_file()?;
    let mut subm = Submarine::new(0, 0, None);
    for line in lines {
        let command = line[0].as_str();
        let displacement = line[1].parse::<i64>()?;
        subm = match command {
            "up" => { subm.change_location(0, 0, Some(-displacement)) },
            "down" => { subm.change_location(0, 0, Some(displacement)) },
            "forward" => { subm.change_location(displacement, subm.aim * displacement, Some(0)) },
            _ => panic!("Erroneous command")
        };
    }
    Ok(subm)
}

fn main() -> Result<()> {
    let sub_a = part_a()?;
    let sub_b = part_b()?;
    
    println!("[A]: {}", sub_a.mul_coords());
    println!("[B]: {}", sub_b.mul_coords());
    Ok(())
}
