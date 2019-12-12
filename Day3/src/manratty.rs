
struct instruction {
    direction : char,
    distance : u32,
}

impl instruction {
    pub fn new(blob : string) {
        let temp = 0;
    }
}

struct FieldPoint {
    has_wire1 : bool,
    has_wire2 : bool,
}

struct ManrattyField {
    // const WIRE1 : usize = 0
    // const WIRE2 : usize = 1
    x : Box<[FieldPoint]>,
    y : Box<[FieldPoint]>,
}

struct Manratty {
    field : ManrattyField,  // Field blob
    origin : Vec<u32>,      // {x,y}
}

impl Manratty {
    pub fn plot_wire() {

    }
    pub fn find_origin_closest() {

    }
}