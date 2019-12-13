use std::thread::current;

#[derive(Copy, Clone)]
pub struct Instruction {
    direction : char,
    distance : u32,
}

impl Instruction {
    pub fn default() -> Instruction {
        Instruction {
            direction : '0',
            distance : 0
        }
    }
    pub fn new(s : String) -> Instruction {
        let (dir, dist) = s.split_at(1);
        Instruction {
            direction : dir.chars().next().unwrap(),
            distance : dist.parse::<u32>().unwrap()
        }
    }
}

#[derive(Copy, Clone)]
pub struct FieldPoint {
    has_wire1 : bool,
    has_wire2 : bool,
}

impl FieldPoint {
    pub fn default() -> FieldPoint {
        FieldPoint {
            has_wire1: false,
            has_wire2: false
        }
    }
    pub fn set(&mut self, wire : bool) {
        if wire {
            self.has_wire1 = true;
        }
        else {
            self.has_wire2 = true;
        }
    }
    pub fn reset(&mut self, wire: bool) {
        if wire {
            self.has_wire1 = false;
        }
        else {
            self.has_wire2 = false;
        }
    }
}

// Manually re-adjusting the indicies to simulate a 10000 x 10000 2d array.
// x is base, y is offset.
// For ex: [1][2] = 1*10,000 + 2 = [10,002]
//         [123][1937] = 123*10,000 + 1937 = [1,231,937]
pub struct ManrattyField {
    array : [FieldPoint; 100000000],
}

impl ManrattyField {
    pub fn default() -> ManrattyField {
        ManrattyField {
            array: [FieldPoint::default(); 100000000],
        }
    }

    pub fn set(&mut self, pt_x : u32, pt_y : u32, wire: bool) {
        self.array[((pt_x * 10000) + (pt_y)) as usize].set(wire);
    }
    pub fn reset(&mut self, pt_x : u32, pt_y : u32) {
        self.array[((pt_x * 10000) + (pt_y)) as usize].set(false);
    }
    pub fn get(&self, pt_x : u32, pt_y : u32) -> (bool, bool) {
        // temp
        (false, false)
    }
}

pub struct Manratty {
    // Field blob
    field: ManrattyField,
    origin: Vec<u32>,
    location: Vec<u32>,
    // {x,y}
    wire1_instructions: Vec<Instruction>,
    wire2_instructions: Vec<Instruction>,
}

impl Manratty {
    const WIRE1 : bool = true;
    const WIRE2 : bool = false;

    pub fn store_instructions(&mut self, w1 : &Vec<Instruction>, w2 : &Vec<Instruction>) {
        self.wire1_instructions = w1.to_vec();
        self.wire2_instructions = w2.to_vec();
    }

    pub fn plot_wire(&mut self) {
        /*          +y
         *          U
         *  -x  L       R  +x
         *          D
         *          -y
        */
        for i in self.wire1_instructions {
            match self.direction {
                'L' => {
                    for j in 0..self.distance {
                        self.field.set(self.location[0] - j, self.location[0], self.WIRE1);
                    }
                    self.location[0] = self.origin[0] - self.distance;
                }
                'R' => {
                    for j in 0..self.distance {
                        self.field.set(self.location[0] + j, self.location[0], self.WIRE1);
                    }
                    self.location[0] = self.location[0] + self.distance;
                }
                'U' => {
                    for j in 0..self.distance {
                        self.field.set(self.location[1] + j, self.location[1], self.WIRE1);
                    }
                    self.location[1] = self.location[1] + self.distance;
                }
                'D' => {
                    for j in 0..self.distance {
                        self.field.set(self.location[1] - j, self.location[1], self.WIRE1);
                    }
                    self.location[1] = self.location[1] - self.distance;
                }
                _ => {
                    println!("Now you don' fucked up.");
                }
            }
        }
    }

    // The idea here is to move out in a spiral (the kind that proved
    // countable infinity as well as dope-ass Ulam-spiral) in an attempt
    // to find the closest cross-over early. I'm going to try to do the
    // lazy thing and assume there exists only one correct answer and
    // go back to accumulate results in the end if required.
    pub fn find_origin_closest(&self) {
        // copy so we do not fuck over our origin by taking ownership
        let mut current = self.origin.to_vec();
        for i in (self.origin[0]+1)..10000000 {

        }
    }

    pub fn default() -> Manratty {
        Manratty {
            field : ManrattyField::default(),
            origin : vec![5000, 5000],
            location: vec![5000, 5000],
            wire1_instructions: vec![],
            wire2_instructions: vec![],
        }
    }
    pub fn new() -> Manratty {
        Manratty {
            field : ManrattyField::default(),
            origin : vec![0,0],
            location: vec![0,0],
            wire1_instructions: vec![],
            wire2_instructions: vec![],
        }
    }
}
