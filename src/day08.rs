const SIZE_X: usize = 25;
const SIZE_Y: usize = 6;
const DIM: usize = SIZE_X * SIZE_Y;

fn count(v: &[usize], val: usize) -> usize {
    v.iter().fold(0, |acc, x| {
        if *x == val {
            return acc + 1;
        }
        acc
    })
}

pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let pixels: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let n_layers = pixels.len() / DIM;
    let mut min = usize::MAX;
    let mut min_layer = &pixels[0..DIM - 1];
    for n in 0..n_layers {
        let layer = &pixels[n * DIM..n * DIM + DIM];
        let c = count(layer, 0);
        if c < min {
            min = c;
            min_layer = layer;
        }
    }
    println!("min  {}", min);
    println!(
        "Solution part 1: {}",
        count(min_layer, 1) * count(min_layer, 2)
    );
}

pub fn part2(mut input: String) {
    input.pop(); // removes the trailing \n
    let pixels: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let n_layers = pixels.len() / DIM;
    println!("n layers {} {}", pixels.len(), n_layers);
    let mut image = vec![2; DIM];
    for n in 0..n_layers {
        let layer = &pixels[n * DIM..n * DIM + DIM];
        for index in 0..DIM {
            if image[index] == 2 {
                image[index] = layer[index]
            }
        }
    }
    println!("Solution part 2:");
    for (x, _) in image.iter().enumerate() {
        let c = if image[x] == 0_usize { ' ' } else { '█' };
        print!("{}", c);
        if x != 0 && x % SIZE_X == 0 {
            println!();
        }
    }
    println!();
}
