use std::collections::HashSet;

use crate::input::input::InputParser;

fn parse_boards(input: Vec<String>) -> Vec<BingoBoard> {

    let board_row_strings: Vec<Vec<String>> = InputParser::chunk(
        input[1..].into_iter().map(|x| x.to_string()).collect(),
        6
    ).unwrap().into_iter().map(
        |chunk| chunk.into_iter().skip(1).take(5).collect()
    ).collect();

    let mut boards = Vec::new();
    for row_strings in board_row_strings {
        let board = BingoBoard::from_row_strings(row_strings);
        boards.push(board);
    }

    return boards;
}

pub fn solve_problem_04a(input: Vec<String>) -> u32 {
    let guesses = &input[0];
    let mut boards = parse_boards(input.clone());

    for guess in guesses.split(",") {
        let guess_value = guess.parse::<u32>().unwrap();
        let mut board_n = 0;
        for board in &mut boards {
            board.mark(guess_value);
            if board.get_bingo_sum() > 0 {
                let unmarked_number_sum = board.get_unmarked_number_sum();
                println!("Bingo on board {} after guess {}!", board_n, guess_value);
                let answer = unmarked_number_sum * guess_value;
                return answer;
            }
            board_n += 1;
        }
    }
    panic!("Reached the end of the file without bingo!");
}

fn solve_problem_04b(input: Vec<String>) -> u32 {

    let guesses = &input[0];
    let mut boards = parse_boards(input.clone());
    let mut last_winning_board = None;
    let mut last_winning_guess = None;
    let mut already_won: HashSet<u32> = HashSet::new();
    for guess in guesses.split(",") {
        let guess_value = guess.parse::<u32>().unwrap();
        let mut board_n = 0;
        for board in &mut boards {
            board.mark(guess_value);
            if !already_won.contains(&board_n) && board.get_bingo_sum() > 0 {
                last_winning_board = Some(board.clone());
                last_winning_guess = Some(guess_value);
                already_won.insert(board_n);
            }
            board_n += 1;
        }
    }
    let last_winning_board = last_winning_board.unwrap();
    let last_winning_guess = last_winning_guess.unwrap();
    let unmarked_number_sum = last_winning_board.get_unmarked_number_sum();
    println!("Last bingo on board {:?} after guess {}!", last_winning_board, last_winning_guess);
    let answer = unmarked_number_sum * last_winning_guess;
    return answer;
}

#[derive(Debug, Clone)]
struct BingoBoard {
    rows: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
}

impl BingoBoard {

    pub fn new(rows: Vec<Vec<u32>>) -> Self {
        assert_eq!(rows.len(), 5);
        assert!(rows.iter().all(|r| r.len() == 5));
        Self { rows, marked: vec![vec![false; 5]; 5] }
    }

    pub fn from_row_strings(row_strings: Vec<String>) -> Self {
        let rows = row_strings.into_iter().map(|row_string| {
            row_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()
        }).collect();
        Self::new(rows)
    }

    pub fn mark(&mut self, number: u32) {
        for (row_index, row) in self.rows.iter().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                if *cell == number {
                    self.marked[row_index][column_index] = true;
                }
            }
        }
    }

    pub fn is_marked(&self, row_index: usize, column_index: usize) -> bool {
        self.marked[row_index][column_index]
    }

    pub fn clear(&mut self) {
        self.marked = vec![vec![false; 5]; 5];
    }

    pub fn columns(&self) -> Vec<Vec<u32>> {
        let mut columns = vec![vec![]; 5];
        for row in self.rows.iter() {
            for (column_index, cell) in row.iter().enumerate() {
                columns[column_index].push(*cell);
            }
        }
        columns
    }

    pub fn marked_columns(&self) -> Vec<Vec<bool>> {
        let mut marked_columns = vec![vec![]; 5];
        for row in self.marked.iter() {
            for (column_index, cell) in row.iter().enumerate() {
                marked_columns[column_index].push(*cell);
            }
        }
        marked_columns
    }

    pub fn diagonals(&self) -> Vec<Vec<u32>> {
        let mut diagonals = vec![vec![]; 2];
        for (row_index, row) in self.rows.iter().enumerate() {
            diagonals[0].push(row[row_index]);
            diagonals[1].push(row[4 - row_index]);
        }
        diagonals
    }

    pub fn marked_diagonals(&self) -> Vec<Vec<bool>> {
        let mut marked_diagonals = vec![vec![]; 2];
        for (row_index, row) in self.marked.iter().enumerate() {
            marked_diagonals[0].push(row[row_index]);
            marked_diagonals[1].push(row[4 - row_index]);
        }
        marked_diagonals
    }

    pub fn sum(&self) -> u32 {
        self.rows.iter().flatten().sum()
    }

    pub fn get_bingo_sum(&self) -> u32 {
        self.get_bingo_sum_row() + self.get_bingo_sum_column()
    }

    pub fn get_marked_number_sum(&self) -> u32 {
        self.rows.iter().flatten().zip(self.marked.iter().flatten()).filter(|(_, marked)| **marked).map(|(number, _)| *number).sum()
    }
    
    pub fn get_unmarked_number_sum(&self) -> u32 {
        self.sum() - self.get_marked_number_sum()
    }

    pub fn get_bingo_sum_row(&self) -> u32 {
        self.marked.iter().enumerate().map(|row| {
            if row.1.iter().all(|x| *x) {
                self.rows[row.0].iter().sum()
            } else {
                0
            }
        }).sum()
    }

    pub fn get_bingo_sum_column(&self) -> u32 {
        self.marked_columns().iter().enumerate().map(|column| {
            if column.1.iter().all(|x| *x) {
                self.columns()[column.0].iter().sum()
            } else {
                0
            }
        }).sum()
    }

    pub fn get_bingo_sum_diagonal(&self) -> u32 {
        self.marked_diagonals().iter().enumerate().map(|diagonal| {
            if diagonal.1.iter().all(|x| *x) {
                self.diagonals()[diagonal.0].iter().sum()
            } else {
                0
            }
        }).sum()
    }
    
}

enum BingoType {
    Row(usize),
    Column(usize)
}

#[cfg(test)]
mod test_problem_04 {

    use super::*;

    #[test]
    fn test_problem_04a_passes() {
        
        let input = InputParser::new().parse_as_string("input_04.txt").unwrap();

        let answer = solve_problem_04a(input);
        assert_eq!(answer, 87456); // 31584 is wrong
    }
    
    #[test]
    fn test_problem_04b_passes() {
        let input = InputParser::new().parse_as_string("input_04.txt").unwrap();

        let answer = solve_problem_04b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_get_bingo_sum() {
        let mut board = BingoBoard::new(
            vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25],
            ]
        );
        assert_eq!(board.get_bingo_sum(), 0);
        board.mark(1);
        assert_eq!(board.get_bingo_sum(), 0);
        board.mark(2);
        board.mark(3);
        board.mark(4);
        assert_eq!(board.get_bingo_sum(), 0);
        board.mark(5);
        assert_eq!(board.get_bingo_sum(), 15);

        board.clear();

        assert_eq!(board.get_bingo_sum(), 0);
        board.mark(6);
        board.mark(7);
        board.mark(8);
        board.mark(9);
        board.mark(10);
        assert_eq!(board.get_bingo_sum(), 40);

        board.clear();

        assert_eq!(board.get_bingo_sum(), 0);
        board.mark(5);
        board.mark(10);
        board.mark(15);
        board.mark(20);
        board.mark(25);
        assert_eq!(board.get_bingo_sum(), 75);
        board.mark(7);
        assert_eq!(board.get_bingo_sum(), 75);
        
    }

}