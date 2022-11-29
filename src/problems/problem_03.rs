use crate::input::input::InputParser;

fn solve_problem_03a(input: Vec<u32>) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..12 {
        let xor_mask = 1 << i;
        let (zeros, ones) = count_bits_at_position(&input, i);
        (gamma, epsilon) = if zeros < ones {
            (gamma | xor_mask, epsilon)
        } else if zeros > ones {
            (gamma, epsilon | xor_mask)
        } else {
            panic!("Equal zeros and ones!");
        };
    }
    return gamma * epsilon;
}

fn recursively_filter_on_criterion_from_the_left(
    input: Vec<u32>,
    criterion: fn(&Vec<u32>, usize) -> u32,
    starting_point: usize
) -> u32 {
    let mut remaining = input.clone();
    let mut i = starting_point;
    loop { 
        let criterion_bit = criterion(&remaining, i);
        remaining = remaining.into_iter().filter(|x| x & (1 << i) == criterion_bit).collect();
        if (remaining.len() <= 1) {
            break;
        }
        i -= 1;
    }
    assert!(remaining.len() != 0);
    assert!(remaining.len() == 1);
    return remaining[0];
}

fn get_most_common_bit_at_position(input: &Vec<u32>, i: usize) -> u32 {
    let xor_mask = 1 << i;
    let (zeros, ones) = count_bits_at_position(&input, i);
    if zeros <= ones {
        return xor_mask;
    } else {
        return 0;
    }
}

fn get_least_common_bit_at_position(input: &Vec<u32>, i: usize) -> u32 {
    let xor_mask = 1 << i;
    let (zeros, ones) = count_bits_at_position(&input, i);
    if zeros > ones {
        return xor_mask;
    } else {
        return 0;
    }
}

fn count_bits_at_position(input: &Vec<u32>, i: usize) -> (u32, u32) {
    let xor_mask = 1 << i;
    input.iter().fold((0, 0), |state, item| {
        if item & xor_mask == 0 {
            (state.0 + 1, state.1)
        } else {
            (state.0, state.1 + 1)
        }
    })
}

fn solve_problem_03b(input: Vec<u32>) -> u32 {
    let oxygen = recursively_filter_on_criterion_from_the_left(input.clone(), get_most_common_bit_at_position, 11);
    let co2 = recursively_filter_on_criterion_from_the_left(input.clone(), get_least_common_bit_at_position, 11);

    return oxygen * co2;
}

#[cfg(test)]
mod test_problem_03 {

    use super::*;

    #[test]
    fn test_problem_03a_passes() {
        
        let input = InputParser::new().parse_as_binary("input_03.txt").unwrap();
        let shortened_input = input.iter().take(3).map(|s| *s).collect::<Vec<u32>>();

        // 111100101100
        // 101100110001
        // 100110100101
        
        // => (0, 3), (2, 1), (1, 2), (0, 3), (2, 1), (3, 0), (0, 3), (2, 1), (2, 1), (1, 2), (3, 0), (1, 2)
        // => Gamma = 101100100101
        // => Epsilon = 010011011010
        assert_eq!(solve_problem_03a(shortened_input), 0b101100100101 * 0b010011011010);

        assert_eq!(solve_problem_03a(input), 845186);
    }

    #[test]
    fn test_problem_03b_passes() {
        
        let input = InputParser::new().parse_as_binary("input_03.txt").unwrap();
        let shortened_input = input.iter().take(4).map(|s| *s).collect::<Vec<u32>>();

        // 111100101100
        // 101100110001
        // 100110100101
        // 001101100010

        // => Oxygen = 101100110001
        // => CO2 = 001101100010
    
        assert_eq!(solve_problem_03b(shortened_input), 0b101100110001 * 0b001101100010);
        assert_eq!(solve_problem_03b(input), 0);
    }

    #[test]
    fn test_get_most_and_least_common_bit() {
        
        let input = InputParser::new().parse_as_binary("input_03.txt").unwrap();
        let shortened_input = input.iter().take(4).map(|s| *s).collect::<Vec<u32>>();

        // 111100101100
        // 101100110001
        // 100110100101
        // 001101100010
    
        assert_eq!(get_most_common_bit_at_position(&shortened_input, 0), 0b1);
        assert_eq!(get_least_common_bit_at_position(&shortened_input, 0), 0b0);

        assert_eq!(get_most_common_bit_at_position(&shortened_input, 11), 0b1 << 11);
        assert_eq!(get_least_common_bit_at_position(&shortened_input, 11), 0b0 << 11);

        assert_eq!(get_most_common_bit_at_position(&shortened_input, 1), 0b0 << 1);
        assert_eq!(get_least_common_bit_at_position(&shortened_input, 1), 0b1 << 1);
    }
}