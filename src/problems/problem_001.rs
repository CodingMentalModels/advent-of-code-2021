use crate::input::input::InputParser;

pub fn solve_problem_001a(input: Vec<i32>) -> usize {
    input.iter().zip(input.iter().skip(1)).filter(|(a, b)| a < b).count()
}

fn solve_problem_001b(input: Vec<i32>) -> usize {
    let windows: Vec<i32> = input.iter().zip(input.iter().skip(1)).zip(input.iter().skip(2)).map(|((a, b), c)| a + b + c).collect();
    solve_problem_001a(windows)
}

#[cfg(test)]
mod test_problem_001a {

    use super::*;

    #[test]
    fn test_problem_001a_passes() {
        
        let input = InputParser::new().parse_as_i32("input_001.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| *i).collect();

        assert_eq!(solve_problem_001a(shorted_input), 6);

        let answer = solve_problem_001a(input);
        assert_eq!(answer, 1548);
    }
    
    #[test]
    fn test_problem_001b_passes() {
        
        let input = InputParser::new().parse_as_i32("input_001.txt").unwrap();
        let shortened_input = input.iter().take(10).map(|i| *i).collect();

        assert_eq!(solve_problem_001b(shortened_input), 4);

        let answer = solve_problem_001b(input);
        assert_eq!(answer, 1589);
    }

    #[test]
    fn test_zipping_and_summing_ignores_nones() {
        let a = vec![1, 2, 3];
        let b = vec![4];

        assert_eq!(a.iter().zip(b.iter()).map(|(a, b)| a + b).collect::<Vec<i32>>(), vec![5]);
    }
}