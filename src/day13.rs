use crate::intcode;

pub fn part1(input: String) {
    let mut program = intcode::get_program(input);
    let mut nblock = 0;
    while !program.done {
        let _x = program.process(0, false);
        let _y = program.process(0, false);
        let tile_id = program.process(0, false);
        if tile_id == 2 {
            nblock = nblock + 1;
        }
    }

    println!("Solution: {:?}", nblock);
}
