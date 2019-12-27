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
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    pub fn default() -> Coordinate {
        Coordinate {
            x: 10000,
            y: 10000
        }
    }

    pub fn new(ex: u32, why: u32) -> Coordinate {
        Coordinate {
            x: ex,
            y: why
        }
    }
}

#[derive(Copy, Clone)]
pub struct WireLine {
    begin : Coordinate,
    end : Coordinate,
    instruction: Instruction,
}

impl WireLine {
    pub fn default() -> WireLine {
        WireLine {
            begin: Coordinate::default(),
            end: Coordinate::default(),
            instruction: Instruction::default(),
        }
    }

    pub fn new(inst: Instruction) -> WireLine {
        WireLine {
            begin: Coordinate::default(),
            end: Coordinate::default(),
            instruction: inst,  //inst.copy() ?
        }
    }

    pub fn set(&mut self, b: Coordinate) {
        self.begin = b;
        match self.instruction.direction {
            'U' => {
                self.end.y = self.begin.y + self.instruction.distance;
                self.end.x = self.begin.x;
            },
            'L' => {
                self.end.x = self.begin.x - self.instruction.distance;
                self.end.y = self.begin.y;
            },
            'R' => {
                self.end.x = self.begin.x + self.instruction.distance;
                self.end.y = self.begin.y;
            },
            'D' => {
                self.end.y = self.begin.y - self.instruction.distance;
                self.end.x = self.begin.x;
            },
            _ => println!("Default condition reached in set WireLine"),
        }
    }
}

pub struct Manratty {
    origin: Coordinate,
    location: Coordinate,

    wire1_line: Vec<WireLine>,
    wire2_line: Vec<WireLine>,
}

impl Manratty {

    pub fn store_instructions(&mut self, w1 : &Vec<WireLine>, w2 : &Vec<WireLine>) {
        self.wire1_line = w1.to_vec();
        self.wire2_line = w2.to_vec();
    }

    //w1 will go out of scope after this
    pub fn store_raw_instruction_w1(&mut self, mut w1 : WireLine) {
        w1.set(self.location);
        self.location = w1.end.copy();
        self.wire1_line.push(w1);
    }

    // w2 will go out of scope after this. Probably.
    pub fn store_raw_instruction_w2(&mut self, mut w2 : WireLine) {
        w2.set(self.location);
        self.location = w2.end.copy();
        self.wire2_line.push(w2);
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
            origin : Coordinate::default(),
            location: Coordinate::default(),
            wire1_line: vec![],
            wire2_line: vec![],
        }
    }
    pub fn new() -> Manratty {
        Manratty {
            origin : Coordinate::default(),
            location: Coordinate::default(),
            wire1_line: vec![],
            wire2_line: vec![],
        }
    }
}
