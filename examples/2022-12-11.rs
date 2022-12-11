use std::fs::*;
use std::io::*;
use std::cell::*;
use std::mem::swap;

enum Operation {
    Add(i32),
    Multiply(i32),
    Square()
}

struct Monkey {
    items: Vec<i32>,
    modulo_items: Vec<Vec::<i32>>,
    operation: Operation,
    test_divisor: i32,
    true_monkey: i32,
    false_monkey: i32
}

fn read_monkeys() -> Vec::<RefCell::<Monkey>> {
    let input = File::open("data/2022-12-11-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut monkeys = Vec::<RefCell::<Monkey>>::new();
    while let Some(Ok(line)) = lines.next() {
        if !line.trim().is_empty() {
            assert!(line.starts_with("Monkey "));
            assert!(line.replace("Monkey ", "").replace(":", "").parse::<usize>().unwrap() == monkeys.len());
            let starting_items: Vec<i32> = lines.next().unwrap().unwrap().replace("Starting items: ", "").split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
            let op_tokens: Vec<String> = lines.next().unwrap().unwrap().replace("  Operation: new = old ", "").split(' ').map(|x| x.trim().to_string()).collect();
            let op = match op_tokens[0].as_str() {
                "+" => { Operation::Add(op_tokens[1].parse::<i32>().unwrap()) },
                "*" => { 
                    if op_tokens[1] == "old" {
                        Operation::Square()
                    }
                    else {
                        Operation::Multiply(op_tokens[1].parse::<i32>().unwrap())
                    }
                }
                _  => { panic!(); }
            };
            let divisor = lines.next().unwrap().unwrap().replace("  Test: divisible by ", "").parse::<i32>().unwrap();
            let true_monkey = lines.next().unwrap().unwrap().replace("    If true: throw to monkey ", "").parse::<i32>().unwrap();
            let false_monkey = lines.next().unwrap().unwrap().replace("    If false: throw to monkey ", "").parse::<i32>().unwrap();
            monkeys.push(RefCell::<Monkey>::new( Monkey {
                items: starting_items,
                modulo_items: Vec::<Vec::<i32>>::new(),
                operation: op,
                test_divisor: divisor,
                true_monkey: true_monkey,
                false_monkey: false_monkey
            }));
        }
    }
    return monkeys;
}

fn find_monkey_business_score(inspection_counts: Vec::<usize>) -> usize {
    let mut top_two: [usize; 2] = [ 0, 0 ];
    for count in inspection_counts {
        let mut current_value = count;
        for i in 0..2 {
            if current_value > top_two[i] {
                swap(&mut current_value, &mut top_two[i]);
            }
        }
    }
    return top_two[0] * top_two[1];
}

fn part_1(rounds: usize, worry_divisor: i32) -> usize {
    let monkeys = read_monkeys();
    let mut inspection_counts = Vec::<usize>::new();
    inspection_counts.resize(monkeys.len(), 0);

    for _round in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let monkey = &monkeys[monkey_index];
            inspection_counts[monkey_index] += monkey.borrow().items.len();
            for item_index in 0..monkey.borrow().items.len() {
                let mut worry_level = monkey.borrow().items[item_index];
                match monkey.borrow().operation {
                    Operation::Add(operand) => { worry_level += operand; },
                    Operation::Multiply(operand) => { worry_level *= operand; },
                    Operation::Square() => { worry_level *= worry_level; }
                }
                worry_level /= worry_divisor;
                if (worry_level % monkey.borrow().test_divisor) == 0 {
                    monkeys[monkey.borrow().true_monkey as usize].borrow_mut().items.push(worry_level);
                }
                else {
                    monkeys[monkey.borrow().false_monkey as usize].borrow_mut().items.push(worry_level);
                }
            }
            monkey.borrow_mut().items.clear();
        }
    }
    return find_monkey_business_score(inspection_counts);
}

fn part_2(rounds: usize) -> usize {
    let monkeys = read_monkeys();
    let mut inspection_counts = Vec::<usize>::new();
    inspection_counts.resize(monkeys.len(), 0);

    // For every item, we keep track of the worry value modulo'ed with the monkey's divisor
    for monkey_index in 0..monkeys.len() {
        let monkey = &monkeys[monkey_index];
        let mut modulo_items = Vec::<Vec::<i32>>::new();
        for item_index in 0..monkey.borrow().items.len() {
            let mut modulo_item = Vec::<i32>::new();
            for monkey_modulo_index in 0..monkeys.len() {
                let modulo_value = if monkey_modulo_index == monkey_index {
                    let a = monkeys[monkey_index].borrow();
                    a.items[item_index] % a.test_divisor
                } else {
                    let a = monkeys[monkey_index].borrow();
                    let b = monkeys[monkey_modulo_index].borrow();
                    a.items[item_index] % b.test_divisor
                };
                modulo_item.push(modulo_value);
            }
            modulo_items.push(modulo_item);
        }
        monkey.borrow_mut().modulo_items = modulo_items;
    }

    for _round in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let monkey = &monkeys[monkey_index];
            inspection_counts[monkey_index] += monkey.borrow().modulo_items.len();
            let mut a = monkey.borrow_mut();
            for item_index in 0..a.modulo_items.len() {
                // This item has a modulo value per monkey - update each of these modulo values
                // by applying the monkey's operation to it
                for modulo_index in 0..monkeys.len() {
                    match a.operation {
                        Operation::Add(operand) => { 
                            let divisor = if monkey_index == modulo_index {
                                a.test_divisor
                            } else {
                                monkeys[modulo_index].borrow().test_divisor
                            };
                            a.modulo_items[item_index][modulo_index] = (a.modulo_items[item_index][modulo_index] + operand) % divisor;
                         },
                        Operation::Multiply(operand) => {
                            let divisor = if monkey_index == modulo_index {
                                a.test_divisor
                            } else {
                                monkeys[modulo_index].borrow().test_divisor
                            };
                            a.modulo_items[item_index][modulo_index] = (a.modulo_items[item_index][modulo_index] * operand) % divisor;
                        },
                        Operation::Square() => {
                            let divisor = if monkey_index == modulo_index {
                                a.test_divisor
                            } else {
                                monkeys[modulo_index].borrow().test_divisor
                            };
                            a.modulo_items[item_index][modulo_index] = (a.modulo_items[item_index][modulo_index] * a.modulo_items[item_index][modulo_index]) % divisor;
                        }
                    }    
                }

                // Move modulo item to next monkey
                let worry_level = a.modulo_items[item_index][monkey_index];
                if worry_level == 0 {
                    monkeys[a.true_monkey as usize].borrow_mut().modulo_items.push(a.modulo_items[item_index].clone());
                }
                else {
                    monkeys[a.false_monkey as usize].borrow_mut().modulo_items.push(a.modulo_items[item_index].clone());
                }
            }
            a.modulo_items.clear();
        }
    }
    return find_monkey_business_score(inspection_counts);
}

fn main() {
    // Part 1
    println!("Monkey Business: {0}", part_1(20, 3));
    // Part 2
    println!("Monkey Business: {0}", part_2(10000));
}