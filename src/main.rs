use std::time::{Instant};
use std::collections::HashMap;

mod util;

use crate::util::add_matrix_with_probability;

fn simulate(
    memo: &mut HashMap::<[u8; 5], [f64; 17]>,
    wall: u8,
    waits: [u8; 4],
    action_matrix: [bool; 16],
    step: usize,
) -> [f64; 17] {

    let key: [u8; 5] = [wall, waits[0], waits[1], waits[2], waits[3]];

    if let Some(value) = memo.get(&key) {
        return *value;
    }
    
    let mut result_matrix: [f64; 17] = [0.0; 17];

    if wall == 13 {
        result_matrix[16] = 1.0;

        return result_matrix;
    }

    let mut advance_chance: f64 = 1.0;
    let advance_matrix: [f64; 17] = simulate(memo, wall - 1, waits, action_matrix, (step + 1) % 4);

    for i in 0..4 {
        if waits[i] == 0 {
            continue;
        }

        let action_chance: f64 = (waits[i] as f64) / (wall as f64);
        advance_chance -= action_chance;

        let mut action_result_matrix: [f64; 17] = [0.0; 17];
        
        if action_matrix[4 * i + step] {
            action_result_matrix[4 * i + step] = 1.0;
        } else {
            let mut new_waits = waits; 
            new_waits[i] -= 1;

            action_result_matrix = simulate(memo, wall - 1, new_waits, action_matrix, (step + 1) % 4);
        }

        add_matrix_with_probability(&mut result_matrix, &action_result_matrix, action_chance);
    }
    
    add_matrix_with_probability(&mut result_matrix, &advance_matrix, advance_chance);

    memo.insert(key, result_matrix);

    result_matrix
}

fn main() {
    let mut memo = HashMap::<[u8; 5], [f64; 17]>::new();

    let wall: u8 = 24 + 13;
    let waits: [u8; 4] = [3, 0, 2, 0];
    let step: usize = 1;

    let action_matrix: [bool; 16] = [
        true, false, true, false,
        false, false, false, false,
        true, false, true, false,
        false, false, false, false,
    ];

    let start = Instant::now();
    let result: [f64; 17] = simulate(&mut memo, wall, waits, action_matrix, step);
    let duration = start.elapsed();

    for i in 0..4 {
        println!("{:?}", result[(4 * i)..(4 * i + 4)].iter().map(|x| format!("{:.3}% ", x * 100.0)).collect::<String>());
    }

    println!("Win Rate: {:.3}%", 100.0 * result[0..4].iter().sum::<f64>());
    println!("Deal-in Rate: {:.3}%", 100.0 * (result[4] + result[8] + result[12]));
    println!("{:?}", result[8..12].iter().sum::<f64>());

    println!("Time elapsed is {:?}", duration);
}
