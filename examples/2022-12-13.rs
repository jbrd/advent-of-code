use std::fs::*;
use std::io::*;
use std::cell::*;
use std::iter::*;
use std::str::*;
use std::cmp::*;

enum PacketEntry {
    Integer(i32),
    List(RefCell<Vec::<PacketEntry>>)
}

fn read_packet_pairs() -> Vec<(String, String)> {
    let input = File::open("data/2022-12-13-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut result = Vec::<(String, String)>::new();
    while let Some(Ok(line)) = lines.next() {
        let next = lines.next().unwrap().unwrap();
        lines.next();
        result.push((line, next));
    }
    return result;
}

fn parse_packet_string<T:Iterator<Item=char>>(mut iterator: T) -> (T, RefCell<Vec::<PacketEntry>>) {
    let mut result: RefCell<Vec::<PacketEntry>> = RefCell::new(
        Vec::<PacketEntry>::new()
    );
    let mut number = String::new();
    while let Some(c) = iterator.next() {
        match c {
            '[' => {
                let (next_iterator, list) = parse_packet_string(iterator);
                result.borrow_mut().push(PacketEntry::List(list));
                iterator = next_iterator;
            },
            ']' => {
                if !number.is_empty() {
                    result.borrow_mut().push(PacketEntry::Integer(number.parse::<i32>().unwrap()));
                    number.clear();
                }
                return (iterator, result);
            }
            ',' => {
                if !number.is_empty() {
                    result.borrow_mut().push(PacketEntry::Integer(number.parse::<i32>().unwrap()));
                    number.clear();
                }
            },
            _ => {
                if c.is_alphanumeric() {
                    number.push(c);
                    continue;
                }    
                panic!(); 
            }
        }
    }

    return (iterator, result);
}

fn parse_packet_pairs(pairs: Vec<(String, String)>) -> Vec<(RefCell<Vec::<PacketEntry>>, RefCell<Vec::<PacketEntry>>)> {
    let mut result = Vec::<(RefCell::<Vec::<PacketEntry>>, RefCell::<Vec::<PacketEntry>>)>::new();
    for (left, right) in pairs {

        let (_left_it, left_list) = parse_packet_string(left.chars().skip(1));
        let (_right_it, right_list) = parse_packet_string(right.chars().skip(1));
        result.push((left_list, right_list));
    }
    return result;
}

fn list_to_string(list: &Ref::<Vec::<PacketEntry>>) -> String {
    let mut s = String::new();
    s.push('[');
    let mut comma = false;
    for item in list.iter() {
        if comma {
            s.push(',');
        }
        match item {
            PacketEntry::List(list) => {
                s += list_to_string(&list.borrow()).as_str();
                comma = true;
            },
            PacketEntry::Integer(i) => {
                s += i.to_string().as_str();
                comma = true;
            }

        }
    }
    s.push(']');
    return s;
}

fn compare_lists(left: Ref<Vec<PacketEntry>>, right: Ref<Vec<PacketEntry>>) -> Ordering {
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        match &left[i] {
            PacketEntry::List(left_list) => {
                match &right[i] {
                    PacketEntry::List(right_list) => {
                        let comparison = compare_lists(left_list.borrow(), right_list.borrow());
                        if comparison != Ordering::Equal {
                            return comparison;
                        }
                    },
                    PacketEntry::Integer(right_number) => {
                        let right_list = RefCell::new(Vec::<PacketEntry>::new());
                        right_list.borrow_mut().push(PacketEntry::Integer(*right_number));
                        let comparison = compare_lists(left_list.borrow(), right_list.borrow());
                        if comparison != Ordering::Equal {
                            return comparison;
                        }
                    }
                }
            },
            PacketEntry::Integer(left_number) => {
                match &right[i] {
                    PacketEntry::List(right_list) => {
                        let left_list = RefCell::new(Vec::<PacketEntry>::new());
                        left_list.borrow_mut().push(PacketEntry::Integer(*left_number));
                        let comparison = compare_lists(left_list.borrow(), right_list.borrow());
                        if comparison != Ordering::Equal {
                            return comparison;
                        }
                    },
                    PacketEntry::Integer(right_number) => {
                        if left_number > right_number {
                            return Ordering::Greater;
                        }
                        else if left_number < right_number {
                            return Ordering::Less;
                        }
                    }
                }
            }
        }
        i += 1;
        j += 1;
    }

    if i == j && i == left.len() && i == right.len() {
        return Ordering::Equal;
    }

    if i == left.len() {
        return Ordering::Less;
    }
    else {
        return Ordering::Greater;
    }
}

fn main() {
    let packet_pairs = read_packet_pairs();
    let parsed_packet_pairs = parse_packet_pairs(packet_pairs);
    let mut total = 0;

    // Part 1
    for i in 0..parsed_packet_pairs.len() {
        let (left, right) = &parsed_packet_pairs[i];
        let result = compare_lists(left.borrow(), right.borrow());
        if result == Ordering::Less {
            total += i + 1;
        }
    }
    println!("Part One - Total: {0}", total);

    // Part 2
    let mut packets: Vec<RefCell<Vec::<PacketEntry>>> = Vec::new();
    packets.push(RefCell::new([PacketEntry::Integer(2)].into()));
    packets.push(RefCell::new([PacketEntry::Integer(6)].into()));
    for (left, right) in parsed_packet_pairs {
        packets.push(left);
        packets.push(right);
    }
    packets.sort_by(|a, b| compare_lists(a.borrow(), b.borrow()));

    let mut first_index = 0;
    let mut second_index = 0;
    for i in 0..packets.len() {
        let first_divider = RefCell::new([PacketEntry::Integer(2)].into());
        let second_divider = RefCell::new([PacketEntry::Integer(6)].into());
        if compare_lists(packets[i].borrow(), first_divider.borrow()) == Ordering::Equal {
            first_index = i + 1;
        }
        else if compare_lists(packets[i].borrow(), second_divider.borrow()) == Ordering::Equal {
            second_index = i + 1;
        }
    }

    println!("Part Two - Decoder Key: {0}", first_index * second_index);
}