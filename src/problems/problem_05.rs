use std::collections::{HashMap, HashSet};

use crate::input::input::InputParser;

fn count_intersections(input: Vec<String>, filter: fn(&LineSegment) -> bool) -> usize {
    let segments = LineSegment::parse_all(input).unwrap();
    
    let points: Vec<(u32, u32)> = segments.into_iter().filter(filter)
        .map(|x| x.get_points()).flatten().collect();
    
    let mut frequencies = HashMap::new();
    for point in points {
        match frequencies.get(&point) {
            Some(n) => frequencies.insert(point, n + 1),
            None => frequencies.insert(point, 1)
        };
    }
    
    let intersections = frequencies.into_iter().filter(|(_k, v)| *v > 1).count();
    return intersections;

}

fn solve_problem_05a(input: Vec<String>) -> usize {
    count_intersections(input, |x| x.is_horizontal() || x.is_vertical())
}

fn solve_problem_05b(input: Vec<String>) -> usize {
    count_intersections(input, |x| x.is_horizontal() || x.is_vertical() || x.is_diagonal())
}

struct LineSegment {
    start: (u32, u32),
    end: (u32, u32),
}

impl LineSegment {

    pub fn new(start: (u32, u32), end: (u32, u32)) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn parse_all(input: Vec<String>) -> Result<Vec<Self>, String> {
        input.iter().map(|line| Self::from_string(line)).collect()
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let parts: Vec<_> = s.split_whitespace().collect();
        assert_eq!(parts.len(), 3);
        let start_string = parts[0];
        let middle_string = parts[1];
        let end_string = parts[2];

        assert_eq!(middle_string, "->");
        let start_parts = Self::parse_part(start_string)?;
        let end_parts = Self::parse_part(end_string)?;

        Ok(Self::new((start_parts[0], start_parts[1]), (end_parts[0], end_parts[1])))
    }

    pub fn parse_part(s: &str) -> Result<Vec<u32>, String> {
        let parts: Vec<_> = s.split(',').collect();
        assert_eq!(parts.len(), 2);
        let x = parts[0].parse::<u32>().map_err(|_| "Unable to parse part to u32")?;
        let y = parts[1].parse::<u32>().map_err(|_| "Unable to parse part to u32")?;
        Ok(vec![x, y])
    }

    pub fn l1_norm(&self) -> usize {
        let l_x = (self.start.0 as i32 - self.end.0 as i32).abs() as usize;
        let l_y = (self.start.1 as i32 - self.end.1 as i32).abs() as usize;
        return l_x.max(l_y);
    }
    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn is_diagonal(&self) -> bool {
        (self.start.0 as i32 - self.end.0 as i32).abs() == (self.start.1 as i32 - self.end.1 as i32).abs()
    }

    pub fn get_points(&self) -> HashSet<(u32, u32)> {
        let mut points = vec![];
        if self.is_horizontal() {
            let start = self.start.0.min(self.end.0);
            let end = self.start.0.max(self.end.0);
            for x in start..=end {
                points.push((x, self.start.1));
            }
        } else if self.is_vertical() {
            let start = self.start.1.min(self.end.1);
            let end = self.start.1.max(self.end.1);
            for y in start..=end {
                points.push((self.start.0, y));
            }
        } else if self.is_diagonal() {
            let slope = (self.end.1 as i32 - self.start.1 as i32) / (self.end.0 as i32 - self.start.0 as i32);
            assert!(slope == 1 || slope == -1);
            if slope == 1 {
                let start = (self.start.0.min(self.end.0), self.start.1.min(self.end.1));
                for i in 0..self.l1_norm() + 1 {
                    points.push((start.0 + i as u32, start.1 + i as u32));
                }
            } else {
                let (start, _end) = if self.start.0 < self.end.0 {
                    (self.start, self.end)
                } else {
                    (self.end, self.start)
                };
                for i in 0..self.l1_norm() + 1 {
                    points.push((start.0 + i as u32, start.1 - i as u32));
                }
            }
        } else {
            panic!("Line segment is neither horizontal, vertical, nor diagonal");
        }
        points.into_iter().collect()
    }

}
#[cfg(test)]
mod test_problem_05 {

    use super::*;

    #[test]
    fn test_problem_05a_passes() {
        
        let input = InputParser::new().parse_as_string("input_05.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_05a(shorted_input), 0);

        let answer = solve_problem_05a(input);
        assert_eq!(answer, 4421);
    }
    
    #[test]
    fn test_problem_05b_passes() {
        let input = InputParser::new().parse_as_string("input_05.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_05b(shorted_input), 6);

        let answer = solve_problem_05b(input);
        assert_eq!(answer, 10157);
    }

    #[test]
    fn test_tuple_equality_works_as_expected() {
        let a = (4, 6);
        let b = (8, 7);
        let c = (4, 6);

        assert_eq!(a, a);
        assert_ne!(a, b);
        assert_eq!(a, c);

    }
}

#[test]
fn test_get_points() {
    
    let line = LineSegment::new((0, 10), (2, 12));
    assert!(line.is_diagonal());
    assert_eq!(line.get_points(), vec![
        (0, 10),
        (1, 11),
        (2, 12)
        ].into_iter().collect()
    );

    let line = LineSegment::new((2, 12), (0, 10));
    assert!(line.is_diagonal());
    assert_eq!(line.get_points(), vec![
        (0, 10),
        (1, 11),
        (2, 12)
        ].into_iter().collect()
    );

    let line = LineSegment::new((10, 12), (14, 8));
    assert!(line.is_diagonal());
    assert_eq!(line.get_points(), vec![
        (10, 12),
        (11, 11),
        (12, 10),
        (13, 9),
        (14, 8),
        ].into_iter().collect()
    );
    
    let line = LineSegment::new((14, 8), (10, 12));
    assert!(line.is_diagonal());
    assert_eq!(line.get_points(), vec![
        (10, 12),
        (11, 11),
        (12, 10),
        (13, 9),
        (14, 8),
        ].into_iter().collect()
    );
}
