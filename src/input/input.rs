use std::{fs::File, io::Read, path::Path};

const INPUT_PATH_HEAD: &str = "C:/Users/cmsdu/repos/advent-of-code-2021/input/";

pub struct InputParser;

impl InputParser {

    pub fn new() -> InputParser {
        InputParser {}
    }

    pub fn parse_as_i32(&self, filepath: &str) -> Result<Vec<i32>, String> {
        self.parse_as_string(filepath)?.iter().map(|line| line.parse::<i32>().map_err(|_| "Unable to parse to i32.".to_string())).collect()
    }

    pub fn parse_as_string(&self, filepath: &str) -> Result<Vec<String>, String> {
        let full_path_string = format!("{}{}", INPUT_PATH_HEAD, filepath);
        let full_path = Path::new(&full_path_string);
        let mut file = File::open(full_path).map_err(|_| "Unable to open file.".to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|_| "Unable to read file.".to_string())?;
        Ok(contents.lines().map(|line| line.to_string()).collect())
    }
}


#[cfg(test)]
mod test_input {
    use super::*;

    #[test]
    fn test_input_parses_input() {
        
        let parser = InputParser::new();
        let parsed = parser.parse_as_i32("input_001.txt").unwrap();

        assert_eq!(parsed.len(), 2000);
    }
}