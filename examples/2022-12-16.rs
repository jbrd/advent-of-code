use std::collections::hash_map::*;
use std::fs::*;
use std::io::*;

#[derive(Debug)]
struct Node {
    key: String,
    flow_rate: i32
}

fn read_input() -> (HashMap::<String, usize>, Vec::<Node>, Vec::<(usize, usize)>) {
    let input = File::open("data/2022-12-16-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let mut key_to_index = HashMap::<String, usize>::new();
    let mut nodes = Vec::<Node>::new();
    let mut edges = Vec::<(String, String)>::new();

    while let Some(Ok(line)) = lines.next() {
        let tokens = line
            .replace("Valve ", "")
            .replace(" has flow rate=", ",")
            .replace("; tunnel leads to valve ", ",")
            .replace("; tunnels lead to valves ", ",")
            .split(",")
            .map(|x| x.to_string())
            .collect::<Vec::<String>>();
        let key = tokens[0].trim().to_string();

        key_to_index.insert(key.clone(), nodes.len());
        nodes.push(Node {
            key: key.clone(),
            flow_rate: tokens[1].trim().parse::<i32>().unwrap(),
        });

        for edge in tokens.iter().skip(2).map(|x| x.trim().to_string()) {
            edges.push((key.clone(), edge));
        }
    }
    let edge_indices = edges.iter().map(|(x,y)| ((&key_to_index)[x], (&key_to_index)[y])).collect::<Vec::<(usize, usize)>>();
    return (key_to_index, nodes, edge_indices);
}

fn calculate_shortest_path_lengths(nodes: &Vec::<Node>, edges: &Vec::<(usize, usize)>) -> Vec::<i32> {
    // Unsatissfied with my initial brute-force approach, I ended up seeking some guidance on this bit
    // (specifically, using the Floyd-Warshall algorithm to calculate the length of the shortest path
    // of each pair of nodes).
    let mut shortest_path_lengths = Vec::<i32>::with_capacity(nodes.len() * nodes.len());
    shortest_path_lengths.resize(nodes.len() * nodes.len(), i32::MAX);

    for i in 0..edges.len() {
        shortest_path_lengths[edges[i].0 * nodes.len() + edges[i].1] = 1;
    }

    for i in 0..nodes.len() {
        shortest_path_lengths[i * nodes.len() + i] = 0;
    }

    for k in 0..nodes.len() {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if shortest_path_lengths[i * nodes.len() + k] != i32::MAX && shortest_path_lengths[k * nodes.len() + j] != i32::MAX {
                    if shortest_path_lengths[i * nodes.len() + j] > shortest_path_lengths[i * nodes.len() + k] + shortest_path_lengths[k * nodes.len() + j] {
                        shortest_path_lengths[i * nodes.len() + j] = shortest_path_lengths[i * nodes.len() + k] + shortest_path_lengths[k * nodes.len() + j];
                    }
                }
            }
        }
    }

    shortest_path_lengths
}

fn evaluate_path_part_1(path: &Vec::<usize>, shortest_path_lengths: &Vec::<i32>, nodes: &Vec::<Node>) -> (i32, bool) {
    const MAX_MINUTES: i32 = 30;
    let mut total_cost = 0;
    let mut minutes = 0;

    if path.len() > 1 {
        let mut flow = 0;
        let mut prev_index = 0;

        for j in 1..path.len() {
            let tunnels_to_travel = shortest_path_lengths[path[prev_index] * nodes.len() + path[j]];
            let time_to_turn_on_valve = tunnels_to_travel + 1;
            minutes += time_to_turn_on_valve;
            total_cost += flow * time_to_turn_on_valve;
            flow += nodes[path[j]].flow_rate;
            prev_index = j;
        }
        
        if minutes < MAX_MINUTES {
            total_cost += flow * (MAX_MINUTES - minutes);
        }
    }

    let path_is_valid = minutes < MAX_MINUTES;
    return (total_cost, path_is_valid);
}

fn evaluate_path_part_2(path: &Vec::<usize>, shortest_path_lengths: &Vec::<i32>, nodes: &Vec::<Node>) -> (i32, bool) {
    const MAX_MINUTES: i32 = 26;
    let mut total_cost = 0;
    let mut agent_a_minutes = 0;
    let mut agent_b_minutes = 0;

    if path.len() > 1 {
        let split_point = ((path.len() - 1) / 2) + 1;
        let mut agent_a_flow = 0;
        let mut agent_a_cost = 0;
        let mut agent_b_cost = 0;
        let mut agent_b_flow = 0;
        let mut agent_a_index = 0;
        let mut agent_b_index = 0;

        for j in 1..path.len() {
            if j <= split_point {
                let tunnels_to_travel = shortest_path_lengths[path[agent_a_index] * nodes.len() + path[j]];
                let time_to_turn_on_valve = tunnels_to_travel + 1;
                agent_a_minutes += time_to_turn_on_valve;
                agent_a_cost += agent_a_flow * time_to_turn_on_valve;
                agent_a_flow += nodes[path[j]].flow_rate;
                agent_a_index = j;
            }
            else {
                let tunnels_to_travel = shortest_path_lengths[path[agent_b_index] * nodes.len() + path[j]];
                let time_to_turn_on_valve = tunnels_to_travel + 1;
                agent_b_minutes += time_to_turn_on_valve;
                agent_b_cost += agent_b_flow * time_to_turn_on_valve;
                agent_b_flow += nodes[path[j]].flow_rate;
                agent_b_index = j;
            }
        }
        
        if agent_a_minutes < MAX_MINUTES {
            agent_a_cost += agent_a_flow * (MAX_MINUTES - agent_a_minutes);
        }

        if agent_b_minutes < MAX_MINUTES {
            agent_b_cost += agent_b_flow * (MAX_MINUTES - agent_b_minutes);
        }

        total_cost = agent_a_cost + agent_b_cost;
    }

    let path_is_valid = agent_a_minutes < MAX_MINUTES && agent_b_minutes < MAX_MINUTES;
    return (total_cost, path_is_valid);
}

fn enumerate_all_paths<E:FnMut(&Vec::<usize>) -> (i32, bool), T:FnMut(&Vec::<usize>, i32)>(node_to_index: &HashMap::<String, usize>, nodes: &Vec::<Node>, eval_fn: &mut E, visitor_fn: &mut T) {
    
    // Calculate the reduced set of node keys that have flowable valves (to reduce the number of
    // path permutationss we need to consider).
    let flowable_valves = node_to_index.iter()
        .filter(|(_key, index)| nodes[**index].flow_rate > 0)
        .map(|(_key, index)| *index)
        .collect::<Vec::<usize>>();

    // Consider all path permutations that visit every flowable valve.
    // Use a bit mask to keep track of which nodes we have visited.
    let mut working_set = Vec::<(Vec::<usize>, i128)>::new();
    working_set.push(([node_to_index["AA"]].into(), 1 << node_to_index["AA"]));

    let mut max_cost = 0;
    while !working_set.is_empty() {
        let (path, node_bit_mask) = working_set.pop().unwrap();
        let mut path_extended = false;
        for next_valve in &flowable_valves {
            let next_key = &nodes[*next_valve].key;
            let next_index = node_to_index[next_key];
            if (node_bit_mask & (1 << next_index)) == 0 {
                let mut next_path = path.clone();
                next_path.push(next_index);

                let (_total_cost, path_is_valid) = eval_fn(&next_path);
                if !path_is_valid {
                    continue;
                }

                working_set.push((next_path, node_bit_mask | (1 << next_index)));
                path_extended = true;
            }
        }

        if !path_extended {
            let (total_cost, _path_is_valid) = eval_fn(&path);
            if total_cost > max_cost {
                visitor_fn(&path, total_cost);
                max_cost = total_cost;
            }
        }
    }
}

fn main() {
    let (node_to_index, nodes, edges) = read_input();

    // Calculate shortest path lengths
    let shortest_path_lengths = calculate_shortest_path_lengths(&nodes, &edges);

    // Part 1.
    let mut max_pressure = 0;
    let mut max_path = Vec::<usize>::new();
    enumerate_all_paths(
        &node_to_index,
        &nodes,
        &mut |path| {
            evaluate_path_part_1(&path, &shortest_path_lengths, &nodes)
        },
        &mut |path, total|
        if max_pressure < total {
            max_pressure = total;
            max_path = path.clone();
        }
    );
    println!("Part 1 - Maximum Pressure Is: {0}", max_pressure);
    println!("{0:?}", max_path);

    // Part 2.
    let mut max_pressure = 0;
    let mut max_path = Vec::<usize>::new();
    enumerate_all_paths(
        &node_to_index,
        &nodes,
        &mut |path| {
            evaluate_path_part_2(&path, &shortest_path_lengths, &nodes)
        },
        &mut |path, total|
        if max_pressure < total {
            max_pressure = total;
            max_path = path.clone();
        }
    );
    println!("Part 2 - Maximum Pressure Is: {0}", max_pressure);
    println!("{0:?}", max_path);
}