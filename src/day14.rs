use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Rule {
    target: Chemical,
    reaction: Vec<Chemical>,
}

#[derive(Clone, Debug)]
struct Chemical {
    name: &'static str,
    amount: u128,
}

#[derive(Clone, Debug)]
struct Store {
    chemicals: HashMap<&'static str, u128>,
    ores: u128,
}

impl Store {
    pub fn init(keys: &Vec<&'static str>) -> Store {
        let mut chemicals = HashMap::new();
        for key in keys {
            chemicals.insert(*key, 0);
        }
        Store {
            chemicals,
            ores: 0_u128,
        }
    }

    // get from the store the amount. Return the missing amount after updating the store
    fn get(&mut self, target: &str, amount: u128) -> u128 {
        let value = self.chemicals.get_mut(target).unwrap();
        let available = *(value);
        if amount >= available {
            *value = 0;
            return amount - available;
        } else {
            *value = available - amount;
            return 0;
        }
    }

    fn add(&mut self, target: &str, amount: u128) {
        let available = self.chemicals.get_mut(target).unwrap();
        *available += amount;
    }

    fn add_ore(&mut self, amount: u128) {
        self.ores += amount;
    }
}

fn parse_chemical(chem: &'static str) -> Chemical {
    let parts: Vec<&'static str> = chem.trim().split_whitespace().collect();
    Chemical {
        name: parts[1],
        amount: parts[0].parse::<u128>().unwrap(),
    }
}

fn parse_rule(line: &'static str) -> Rule {
    let parts: Vec<&'static str> = line.split(" => ").collect();
    let target: Chemical = parse_chemical(parts[1]);
    let reactions: Vec<&str> = parts[0].split(",").collect();
    let reaction = reactions.iter().map(|ch| parse_chemical(ch)).collect();
    Rule { target, reaction }
}

fn parse_input(mut input: String) -> HashMap<&'static str, Rule> {
    input.pop(); // removes the trailing "\n"

    // Convert the string to a 'static &str (with a leak)
    let input_str = Box::leak(input.into_boxed_str());
    let rules_vec = input_str.lines().map(parse_rule).collect::<Vec<Rule>>();
    let mut rules: HashMap<&str, Rule> = HashMap::new();
    for rule in rules_vec {
        rules.insert(rule.target.name, rule);
    }
    rules
}

pub fn div_up(a: u128, b: u128) -> u128 {
    (a + (b - 1)) / b
}

fn process(needed: u128, target: &str, rules: &HashMap<&str, Rule>, store: &mut Store) {
    let rule = rules.get(target).unwrap();
    // If we create 2A but we need 3A, we have to process multiple times
    let number_process = div_up(needed, rule.target.amount);
    for _ in 0..number_process {
        for chem in &rule.reaction {
            if chem.name != "ORE" {
                let missing_chem = store.get(chem.name, chem.amount);
                process(missing_chem, chem.name, rules, store);
            } else {
                store.add_ore(chem.amount);
            }
        }
    }
    // I need 7 A, but I create 10, then I have 3 A left in the store.
    store.add(
        rule.target.name,
        rule.target.amount * number_process - needed,
    );
}

pub fn part1(input: String) {
    let rules = parse_input(input);
    let keys = rules.keys().map(|s| *s).collect::<Vec<&'static str>>();
    let mut store: Store = Store::init(&keys);
    process(1, "FUEL", &rules, &mut store);
    println!("Solution: {:?}", store.ores);
}

pub fn part2(input: String) {
    println!("Please launch this with --release, like `cargo run --release 14`, if so, it should take approx 30 minutes");
    let rules = parse_input(input);
    let keys = rules.keys().map(|s| *s).collect::<Vec<&'static str>>();
    let mut store: Store = Store::init(&keys);
    let mut fuels: u128 = 0;
    // Probably there is a better way than brote-force, but couldn't find it
    while store.ores < 1000000000000_u128 {
        fuels = fuels + 1;
        process(1, "FUEL", &rules, &mut store);
        if (fuels % 10000) == 0 {
            println!(
                "{:?} {:.2} %",
                store.ores,
                (store.ores as f64 / 1000000000000_f64) * 100_f64
            );
        }
    }
    println!("Solution: {:?}", fuels - 1);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day14_part1() {
        let input = String::from(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
",
        );

        let rules = parse_input(input);
        let keys = rules.keys().map(|s| *s).collect::<Vec<&'static str>>();

        let mut store: Store = Store::init(&keys);
        process(1, "B", &rules, &mut store);
        assert_eq!(store.ores, 1);

        store = Store::init(&keys);
        process(1, "A", &rules, &mut store);
        assert_eq!(store.ores, 10);

        store = Store::init(&keys);
        process(1, "C", &rules, &mut store);
        assert_eq!(store.ores, 11);

        store = Store::init(&keys);
        process(1, "FUEL", &rules, &mut store);
        assert_eq!(store.ores, 31);
    }
}
