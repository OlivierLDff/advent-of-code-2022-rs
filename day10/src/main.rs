fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: \n{}", part2(input));
}

fn part1(input: &str) -> i32 {
    let reg_states = get_reg_state_at_each_cycles(input);
    let cycles = vec![20, 60, 100, 140, 180, 220];
    get_signal_strength_sum_at_cycles(&reg_states, &cycles)
}

fn get_signal_strength_sum_at_cycles(reg_states: &[i32], cycles: &[i32]) -> i32 {
    let signel_at_cycles = cycles
        .iter()
        .map(|&c| reg_states[(c - 1) as usize] * c)
        .collect::<Vec<_>>();
    signel_at_cycles.iter().sum()
}

fn get_reg_state_at_each_cycles(input: &str) -> Vec<i32> {
    let mut reg = 1;
    let mut reg_states = Vec::new();
    for line in input.lines() {
        let mut cmd_split = line.split_whitespace();
        let cmd = cmd_split.next().unwrap();
        match cmd {
            "noop" => {
                // One cycle no operation
                reg_states.push(reg.clone());
            }
            "addx" => {
                // Simulate the two cycles
                reg_states.push(reg.clone());
                reg_states.push(reg.clone());

                // Update register X
                let add_value = cmd_split.next().unwrap().parse::<i32>().unwrap();
                let reg = &mut reg;
                *reg += add_value;
            }
            _ => panic!("Unknown command: {}", cmd),
        }
    }
    reg_states
}

fn draw_crt(reg_states: &[i32], width: usize, height: usize) -> String {
    let mut crt = String::new();
    for (idx, reg) in reg_states.iter().enumerate() {
        let current_y = idx / width;
        if current_y == height {
            break;
        }

        let current_x = (idx % width + 1) as i32;
        static SPRITE_SIZE: i32 = 3;
        let sprite_position = *reg;
        let pixel_visible =
            current_x >= sprite_position && current_x < sprite_position + SPRITE_SIZE;

        match pixel_visible {
            true => crt.push('#'),
            false => crt.push('.'),
        }

        if current_x as usize == width {
            crt.push('\n');
        }
    }

    crt
}

fn part2(input: &str) -> String {
    let cpu_states = get_reg_state_at_each_cycles(input);
    draw_crt(&cpu_states, 40, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    static _EXAMPLE_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_get_cyles_at_each_cycles() {
        let reg_states = get_reg_state_at_each_cycles(
            "noop
addx 3
addx -5",
        );
        assert_eq!(reg_states.len(), 5);
        assert_eq!(reg_states[0], 1);
        assert_eq!(reg_states[1], 1);
        assert_eq!(reg_states[2], 1);
        assert_eq!(reg_states[3], 4);
        assert_eq!(reg_states[4], 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(_EXAMPLE_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(_EXAMPLE_INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
