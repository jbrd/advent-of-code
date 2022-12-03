/*
--- Day 3: Rucksack Reorganization ---
One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf
didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. 
The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding 
the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types 
    of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number 
of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while 
the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items 
vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both 
compartments is lowercase p.
The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both
compartments is uppercase L.
The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
The fourth rucksack's compartments only share item type v.
The fifth rucksack's compartments only share item type t.
The sixth rucksack's compartments only share item type s.
To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
In the above example, the priority of the item type that appears in both compartments of each rucksack is
 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
*/
use std::cmp::*;
use std::fs::File;
use std::io::*;
use std::str;

fn get_priority(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 'a' as i32 + 1
    }
    else {
        c as i32 - 'A' as i32 + 27
    }
}

fn part_1() {
    let input = File::open("data/2022-12-03-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut total_priority = 0;
    while let Some(result) = lines.next() {
        if let Ok(line) = result {
            let half_len = line.len() / 2;
            let (left, right) = line.split_at(half_len);
            let mut left_bytes: Vec<char> = left.chars().collect();
            let mut right_bytes: Vec<char> = right.chars().collect();
            left_bytes.sort();
            right_bytes.sort();

            let (mut i, mut j) = (0, 0);
            while i < half_len && j < half_len && left_bytes[i] != right_bytes[j] {
                if left_bytes[i] > right_bytes[j] {
                    j += 1;
                } else {
                    i += 1;
                }
            }

            assert!(left_bytes[i] == right_bytes[j]);
            total_priority += get_priority(left_bytes[i]);
        }
    }
    println!("Total Priority: {0}", total_priority);
}

/*
--- Part Two ---
As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency,
within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is
item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be
carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to
be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is
the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type.
So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges.
In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the
first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item
types?
*/

fn part_2() {
    let input = File::open("data/2022-12-03-input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut total_priority = 0;
    
    while let Some(Ok(line_a)) = lines.next() {
        if let Some(Ok(line_b)) = lines.next() {
            if let Some(Ok(line_c)) = lines.next() {
                let mut group: [Vec<char>; 3] = [
                    line_a.chars().collect(),
                    line_b.chars().collect(),
                    line_c.chars().collect()
                ];
                group[0].sort();
                group[1].sort();
                group[2].sort();
                
                let (mut i, mut j, mut k) = (0, 0, 0);
                while i < group[0].len() && j < group[1].len() && k < group[2].len() {
                    if group[0][i] == group[1][j] && group[1][j] == group[2][k] {
                        break;
                    }
                    if group[0][i] > group[1][j] {
                        if group[1][j] > group[2][k] {
                            k += 1;
                        } else {
                            j += 1;
                        }
                    } else {
                        if group[0][i] > group[2][k] {
                            k += 1;
                        } else {
                            i += 1;
                        }
                    }
                }

                assert!(group[0][i] == group[1][j] && group[1][j] == group[2][k]);
                total_priority += get_priority(group[0][i]);
            }
        }
    }
    println!("Total Priority: {0}", total_priority);
}

fn main() {
    part_1();
    part_2();
}