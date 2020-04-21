#[derive(Copy, Clone)]
pub struct Instruction {
    pub(crate) direction : char,
    pub(crate) distance : i32,
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
    steps_start: i32,
    steps_end: i32,
}

impl Wire {
    pub fn default() -> Wire {
        Wire {
            direction : '0',
            length : 0,
            start : [0; 2],
            end: [0; 2],
            steps_start: i32::max_value(),
            steps_end: i32::max_value(),
        }
    }

    pub fn new(dir : char, len : i32, st : [i32; 2], en : [i32; 2]) -> Wire {
        Wire {
            direction : dir,
            length : len,
            start : st,
            end : en,
            steps_start: i32::max_value(),
            steps_end: i32::max_value(),
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

    pub fn store_instructions(&mut self, w1 : Vec<Instruction>, w2 : Vec<Instruction>) {
        self.wire1_instructions = w1;
        self.wire2_instructions = w2;
    }

    pub fn plot_wires(&mut self) {
        const X: usize = 0;
        const Y: usize = 1;

        self.location1 = [0; 2];
        self.location2 = [0; 2];

        let mut steps1: i32 = 0;
        for inst in self.wire1_instructions.iter() {
            let mut nwire : Wire = Wire::default();
            nwire.direction = inst.direction;
            nwire.length = inst.distance;
            nwire.start = self.location1;
            println!("Nwire1 | Dir: {}, Len: {}, start: <{},{}>", nwire.direction, nwire.length, nwire.start[X], nwire.start[Y]);

            let mut end : [i32; 2] = self.location1;

            match nwire.direction {
                'U' => end[Y] = end[Y] + nwire.length,
                'L' => end[X] = end[X] - nwire.length,
                'R' => end[X] = end[X] + nwire.length,
                'D' => end[Y] = end[Y] - nwire.length,
                _ => {println!("Invalid direction caught. Direction: {}", nwire.direction)} // TODO: add proper catch here
            }
            nwire.end = end;

            nwire.steps_start = steps1;
            steps1 = steps1 + nwire.length;
            nwire.steps_end = steps1;

            self.wires1.push(nwire);

            //update location
            self.location1 = end;
        }

        let mut steps2: i32 = 0;
        for inst in self.wire2_instructions.iter() {
            let mut nwire : Wire = Wire::default();
            nwire.direction = inst.direction;
            nwire.length = inst.distance;
            nwire.start = self.location2;
            println!("Nwire2 | Dir: {}, Len: {}, start: <{},{}>", nwire.direction, nwire.length, nwire.start[X], nwire.start[Y]);

            let mut end : [i32; 2] = self.location2;
            match nwire.direction { // U R L D
                'U' => end[Y] = end[Y] + nwire.length,
                'L' => end[X] = end[X] - nwire.length,
                'R' => end[X] = end[X] + nwire.length,
                'D' => end[Y] = end[Y] - nwire.length,
                _ => {println!("Invalid direction caught. Direction: {}", nwire.direction)} // TODO: add proper catch here
            }
            nwire.end = end;

            nwire.steps_start = steps2;
            steps2 = steps2 + nwire.length;
            nwire.steps_end = steps2;

            self.wires2.push(nwire);

            //update location
            self.location2 = end;
        }
    }

    // helper fn for distance between two points
    pub fn get_m_distance(&self, start: [i32; 2], end: [i32; 2]) -> i32 {
        let mut del_x = 0;
        let mut del_y = 0;

        // Just a ghetto abs()
        if end[0] > start[0] {
            del_x = end[0] - start[0];
        } else {
            del_x = start[0] - end[0];
        }
        if end[1] > start[1] {
            del_y = end[1] - start[1];
        } else {
            del_y = start[1] - end[1];
        }
        let mdist = del_y + del_x;
        mdist
    }

    pub fn find_closest_intersect(&self) -> [i32; 2] {
        const X: usize = 0;
        const Y: usize = 1;
        const X_STEPS: usize = 2;
        const Y_STEPS: usize = 3;
        // As this is just constructed as a series of line segments with no edges
        // the plan is just to iterate through the segments in a O(n^2) order and
        // check for proximity/intersects.
        // The first check will be proximity. If both wires have start/end coordinates
        // further away than the longest length, no intersect is possible.
        // The fact that this is laid out in a grid makes it easier to check.
        // Can just check along the line to see if it can intersect.

        // store intersects
        let mut intersections : Vec<[i32; 2]> = vec![];
        // TODO: Expand this to include intersection distances.
        // TODO: cont. - should just move to internal stuct and
        // TODO: cont. - implement in other fn

        for wire1 in &self.wires1 {
            for wire2 in &self.wires2 {
                //check distance and location
                // First we must check if vert or horiz wire1
                // Then check wire2.
                // Vertical wires have same start.x & end.x
                // Horizontal wire have same start.y & end.y
                match wire1.direction {
                    'U' | 'D' => {
                        if wire2.direction == 'L' || wire2.direction == 'R' {
                            // Check both directions because the wire can be routed either way
                            if ((wire1.start[Y] <= wire2.start[Y]) && (wire2.start[Y] <= wire1.end[Y])) ||
                                ((wire1.start[Y] >= wire2.start[Y]) && (wire2.start[Y] >= wire1.end[Y])) {
                                // Now check for cross
                                if ((wire2.start[X] <= wire1.start[X]) && (wire1.start[X] <= wire2.end[X])) ||
                                    ((wire2.start[X] >= wire1.start[X]) && (wire1.start[X] >= wire2.end[X])) {
                                    // Congrats, intersect was found.
                                    let intersect = [wire1.start[X], wire2.start[Y]];
                                    intersections.push(intersect);
                                }
                            }
                        } else {
                            // else only need to check for vert attach
                            if wire1.start[X] == wire2.start[X] {
                                if (wire2.start[Y] <= wire1.start[Y]) && (wire2.start[Y] >= wire2.end[Y]) {
                                    let intersect = [wire1.start[X], wire1.end[Y]];
                                } else if (wire2.start[Y] >= wire1.start[Y]) && (wire2.start[Y] <= wire2.end[Y]) {
                                    // fill in reset of overlap check. Its oogly.
                                }
                            }
                        }
                    }
                    'L' | 'R' => {
                        // Check both directions because the wire can be routed either way
                        if ((wire1.start[X] <= wire2.start[X]) && (wire2.start[X] <= wire1.end[X])) ||
                            ((wire1.start[X] >= wire2.start[X]) && (wire2.start[X] >= wire1.end[X])) {
                            // Now check for cross
                            if ((wire2.start[Y] <= wire1.start[Y]) && (wire1.start[Y] <= wire2.end[Y])) ||
                                ((wire2.start[Y] >= wire1.start[Y]) && (wire1.start[Y] >= wire2.end[Y])) {
                                // Congrats, intersect was found.
                                let intersect = [wire2.start[X], wire1.start[Y]];
                                intersections.push(intersect);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // find closest.
        let mut smol : [i32; 2] = [0; 2];

        if !intersections.is_empty() {
            let first = intersections.first();
            smol = *first.unwrap();
            for sect in intersections {
                // The checker is finding the intersection at the port. Second condition removes to 0.
                if (sect[X].abs() + sect[Y].abs()) < (smol[X].abs() + smol[Y].abs()) ||
                    ((smol[X] == smol[Y]) && (smol[X] == 0)) {
                    smol = sect;
                }
            }
        }
        smol
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
