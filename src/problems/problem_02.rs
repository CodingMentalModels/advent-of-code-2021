use crate::input::input::InputParser;

enum SubmarineCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl SubmarineCommand {

    pub fn parse_commands(command_strings: Vec<String>) -> Result<Vec<Self>, String> {
        command_strings.iter().map(|c| Self::parse_command(c.to_string())).collect()
    }

    pub fn parse_command(s: String) -> Result<Self, String> {
        let words = s.split_whitespace().collect::<Vec<&str>>();
        if words.len() != 2 {
            return Err(format!("Invalid command length: {}.", words.len()));
        }
        let command = words[0];
        let value = words[1].parse::<i32>().map_err(|_| "Unable to parse value.".to_string())?;

        match command {
            "forward" => Ok(Self::Forward(value)),
            "up" => Ok(Self::Up(value)),
            "down" => Ok(Self::Down(value)),
            _ => Err(format!("Invalid command: {}.", command)),
        }
    }
}

fn solve_problem_02a(input: Vec<String>) -> i32 {
    let commands = SubmarineCommand::parse_commands(input).unwrap();
    let (total_forward, total_down): (i32, i32) = commands.into_iter().map(|c| {
        match c {
            SubmarineCommand::Forward(n) => (n, 0),
            SubmarineCommand::Down(n) => (0, n),
            SubmarineCommand::Up(n) => (0, -n),
        }
    }).reduce(|accumulator, item| (accumulator.0 + item.0, accumulator.1 + item.1)).expect("Input shouldn't be empty.");
    return total_forward * total_down;
}

fn solve_problem_02b(input: Vec<String>) -> i32 {
    let commands = SubmarineCommand::parse_commands(input).unwrap();
    let (_final_aim, final_forward, final_down): (i32, i32, i32) = commands.iter()
        .fold((0, 0, 0), |state, item| {
            let (delta_aim, delta_forward, delta_down): (i32, i32, i32) = match item {
                SubmarineCommand::Forward(n) => (0, *n, state.0 * n),
                SubmarineCommand::Down(n) => (*n, 0, 0),
                SubmarineCommand::Up(n) => (-n, 0, 0),
            };
            let (new_aim, new_forward, new_down): (i32, i32, i32) = (state.0 + delta_aim, state.1 + delta_forward, state.2 + delta_down);
            (new_aim, new_forward, new_down)
        }
    );
    return final_forward * final_down;
}

#[cfg(test)]
mod test_problem_02 {

    use super::*;

    #[test]
    fn test_problem_02a_passes() {
        
        let input = InputParser::new().parse_as_string("input_02.txt").unwrap();
        let shortened_input = input.iter().take(15).map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(solve_problem_02a(shortened_input), 23*31);
        assert_eq!(solve_problem_02a(input), 1989265);
    }

    #[test]
    fn test_problem_02b_passes() {
        
        let input = InputParser::new().parse_as_string("input_02.txt").unwrap();
        let shortened_input = input.iter().take(15).map(|s| s.to_string()).collect::<Vec<String>>();

        // forward 9 => Aim = 0 => (9, 0)
        // forward 9 => Aim = 0 => (18, 0)
        // forward 3 => Aim = 0 => (21, 0)
        // down 2 => Aim = 2 => (21, 0)
        // forward 8 => Aim 2 = => (29, 16)
        // down 8 => Aim = 10 => (29, 16)
        // forward 1 => Aim = 10 => (30, 26)
        // down 6 => Aim 16 = => (30, 26)
        // down 9 => Aim 25 = => (30, 26)
        // down 9 => Aim 34 = => (30, 26)
        // forward 1 => Aim 34 = => (31, 60)
        // up 5 => Aim = 29 => (31, 60)
        // up 4 => Aim = 25 => (31, 60)
        // up 8 => Aim = 17 => (31, 60)
        // down 6 => Aim = 23 = => (31, 60)
        
        assert_eq!(solve_problem_02b(shortened_input), 31*60);
        assert_eq!(solve_problem_02b(input), 2089174012);
    }
}