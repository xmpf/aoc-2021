use std::fmt;

pub struct Submarine {
    x: i64,
    z: i64,
    pub aim: i64,
}

impl Submarine {
    pub fn new(initial_x: i64, initial_z: i64, initial_aim: Option<i64>) -> Submarine {
        Submarine {
            x: initial_x, 
            z: initial_z, 
            aim: initial_aim.unwrap_or(0),
        }
    }
    pub fn change_location(&self, dx: i64, dz: i64, daim: Option<i64>) -> Submarine {
        Submarine {
            x: self.x + dx,
            z: self.z + dz,
            aim: self.aim + daim.unwrap_or(0),
        }
    }

    pub fn mul_coords(&self) -> i64 {
        self.x * self.z
    }
}

impl fmt::Display for Submarine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}