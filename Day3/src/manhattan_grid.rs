#[derive(Copy, Clone)]
pub struct Instruction {
    direction : char,
    distance : i32,
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
            distance : dist.parse::<i32>().unwrap()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Wire {
    direction : char,
    length : i32,
    start : [i32; 2],
    end : [i32; 2],      // Might as well.
}

impl Wire {
    pub fn default() -> Wire {
        Wire {
            direction : '0',
            length : 0,
            start : [0; 2],
            end: [0; 2],
        }
    }

    pub fn new(dir : char, len : i32, st : [i32; 2], en : [i32; 2]) -> Wire {
        Wire {
            direction : dir,
            length : len,
            start : st,
            end : en,
        }
    }
}

pub struct Manratty {
    location1: [i32; 2],
    location2: [i32; 2],
    // {x,y}
    wire1_instructions: Vec<Instruction>,
    wire2_instructions: Vec<Instruction>,

    wires1: Vec<Wire>,
    wires2: Vec<Wire>,
}

impl Manratty {

    pub fn store_instructions(&mut self, w1 : &Vec<Instruction>, w2 : &Vec<Instruction>) {
        self.wire1_instructions = w1.to_vec();
        self.wire2_instructions = w2.to_vec();
    }

    pub fn plot_wires(&mut self) {
        const X: usize = 0;
        const Y: usize = 1;

        self.location1 = [0; 2];
        self.location2 = [0; 2];

        for inst in self.wire1_instructions.iter() {
            let mut nwire : Wire = Wire::default();
            nwire.direction = inst.direction;
            nwire.length = inst.distance;
            nwire.start = self.location1;

            let mut end : [i32; 2] = self.location1;
            match nwire.direction { // U R L D
                'U' => end[Y] = end[Y] + nwire.length,
                'L' => end[X] = end[X] - nwire.length,
                'R' => end[X] = end[X] + nwire.length,
                'D' => end[Y] = end[Y] - nwire.length,
                _ => {println!("Invalid direction caught")} // TODO: add proper catch here
            }
            nwire.end = end;
            self.wires1.push(nwire);

            //update location
            self.location1 = end;
        }

        for inst in self.wire2_instructions.iter() {
            let mut nwire : Wire = Wire::default();
            nwire.direction = inst.direction;
            nwire.length = inst.distance;
            nwire.start = self.location2;

            let mut end : [i32; 2] = self.location2;
            match nwire.direction { // U R L D
                'U' => end[Y] = end[Y] + nwire.length,
                'L' => end[X] = end[X] - nwire.length,
                'R' => end[X] = end[X] + nwire.length,
                'D' => end[Y] = end[Y] - nwire.length,
                _ => {println!("Invalid direction caught")} // TODO: add proper catch here
            }
            nwire.end = end;
            self.wires2.push(nwire);

            //update location
            self.location2 = end;
        }
    }

    pub fn default() -> Manratty {
        Manratty {
            location1: [0; 2],
            location2: [0; 2],
            wire1_instructions: vec![],
            wire2_instructions: vec![],
            wires1: vec![],
            wires2: vec![]
        }
    }
    pub fn new() -> Manratty {
        Manratty {
            location1: [0; 2],
            location2: [0; 2],
            wire1_instructions: vec![],
            wire2_instructions: vec![],
            wires1: vec![],
            wires2: vec![]
        }
    }
}
