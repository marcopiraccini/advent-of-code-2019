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

pub fn part2(mut input: String) {
    // set 2, play for free!
    input.replace_range(0..1, "2");
    let mut program = intcode::get_program(input);
    let mut paddle_x = 0;
    let mut score = 0;
    let mut direction = 0;
    while !program.done {
        let x = program.process(direction, false);
        let y = program.process(direction, false);
        let tile = program.process(direction, false);
        if tile == 4 {
            // ball, decide here the paddle direction
            if x > paddle_x {
                direction = 1;
            } else if x < paddle_x {
                direction = -1;
            } else {
                direction = 0
            }
        }
        if tile == 3 {
            paddle_x = x;
        }
        if x == -1 && y == 0 {
            score = tile;
        }
    }
    println!("Solution: {:?}", score);
}
