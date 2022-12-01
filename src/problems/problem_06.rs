use std::{iter, collections::HashMap};

use crate::input::input::InputParser;

pub fn solve_problem_06a(input: Vec<u32>) -> usize {
    let mut population = Population::new(input);
    for _i in 0..80 {
        population.tick()
    }
    return population.len();
}

fn solve_problem_06b(input: Vec<u32>) -> usize {
    let mut populations = (0..7).into_iter().map(|n| Population::new(vec![n])).collect::<Vec<_>>();
    populations.iter_mut().for_each(|population| (0..32).into_iter().for_each(|_n| population.tick()));

    input.into_iter().map(|timer| populations[timer as usize].len()).sum()
}

struct Population {
    timers: Vec<u32>,
}

impl Population {

    pub fn new(timers: Vec<u32>) -> Self {
        Self {timers, cached: HashMap::new()}
    }

    pub fn len(&self) -> usize {
        self.timers.len()
    }

    pub fn tick(&mut self) {
        let n_new_timers = self.timers.iter().filter(|t| **t == 0).count();
        let mut temp_timers: Vec<u32> = self.timers.iter().map(|t| if *t == 0 {6} else {t - 1}).collect();
        temp_timers.extend(iter::repeat(8).take(n_new_timers));
        self.timers = temp_timers;
    }

    pub fn get_timers(&self) -> &Vec<u32> {
        &self.timers
    }

    pub fn tick_for(&mut self, n_generations: u32) {
        let n_partitions = (n_generations / 32) - 1;
        let cache_size = n_generations / n_partitions;
        let mut populations = (0..9).into_iter().map(|n| Population::new(vec![n])).collect::<Vec<_>>();
        populations.iter_mut().for_each(|population| (0..n_generations).into_iter().for_each(|_n| population.tick()));
        let cache = populations.into_iter().enumerate().map(|(n, p)| (n, p.get_timers().clone())).collect::<HashMap<_, _>>();
    }
}

#[cfg(test)]
mod test_problem_06 {

    use super::*;

    #[test]
    fn test_problem_06a_passes() {
        
        let shorted_input: Vec<u32> = vec![
            3,
            4,
            3,
            1,
            2
        ];

        assert_eq!(solve_problem_06a(shorted_input), 5934);

        let input = InputParser::new().parse_as_string("input_06.txt").unwrap();
        assert_eq!(input.len(), 1);
        let timers: Vec<u32> = input[0].split(",").into_iter().map(|s| s.parse::<u32>().expect("We should be able to parse into a number.")).collect();
        
        let answer = solve_problem_06a(timers);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_06b_passes() {
  
        let shorted_input: Vec<u32> = vec![
            3,
            4,
            3,
            1,
            2
        ];

        assert_eq!(solve_problem_06b(shorted_input), 26984457539);

        // let input = InputParser::new().parse_as_string("input_06.txt").unwrap();
        // assert_eq!(input.len(), 1);
        // let timers: Vec<u32> = input[0].split(",").into_iter().map(|s| s.parse::<u32>().expect("We should be able to parse into a number.")).collect();
        
        // let answer = solve_problem_06b(timers);
        // assert_eq!(answer, 0);
    }

    #[test]
    fn test_population_ticks() {
        let mut population = Population::new(vec![1, 2, 3]);
        assert_eq!(population.len(), 3);
        assert_eq!(population.get_timers(), &vec![1, 2, 3]);

        population.tick();
        assert_eq!(population.len(), 3);
        assert_eq!(population.get_timers(), &vec![0, 1, 2]);

        population.tick();
        assert_eq!(population.len(), 4);
        assert_eq!(population.get_timers(), &vec![6, 0, 1, 8]);
    }
}