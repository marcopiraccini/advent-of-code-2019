use crate::intcode;
use crate::map;

fn create_map(input: String) -> map::Map<char> {
    let mut program = intcode::get_program(input); // We are not using this
    let mut width = 0;
    let mut index = 0;
    let mut chars = Vec::new();
    while !program.done {
        let out = program.process(0, false).to_le_bytes()[0] as char;
        if width == 0 && out == '\n' {
            width = index;
        }
        // We skip newlines
        if out != '\n' && out != '\u{0}' {
            chars.push(out);
        }
        index = index + 1;
    }
    map::Map::init(chars, width)
}

pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let scaff = create_map(input);
    let all = scaff.get_all_pos();
    let res = all.iter().fold(0, |sum, pos| {
        if scaff.is_cross('#', *pos) {
            return sum + pos.x * pos.y;
        }
        sum
    });
    println!("Solution part 1 {:?}", res);
}
