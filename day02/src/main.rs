use std::env;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str;
use regex::Regex;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
   let file = File::open(filename)?;
   Ok(io::BufReader::new(file).lines())
}


fn get_solution_part1(path: &str) -> i32 {
   let max_red_cubes = 12;
   let max_green_cubes = 13;
   let max_blue_cubes = 14;
   let re = Regex::new(r"\d+|:|[a-z]+|;").unwrap();
   let mut result = 0;
   if let Ok(lines) = read_lines(path) {
      let mut game_number:i32 = 0;
      for line in lines {
         let current_line = line.unwrap().to_lowercase();
         let m: Vec<&str> = re.find_iter(current_line.as_str()).map(|m| m.as_str()).collect();
         let sets:Vec<_> = m.split(|&e| e == ";").filter(|v| !v.is_empty()).collect();
         let mut highest_found_red_cubes = 0;
         let mut highest_found_green_cubes = 0;
         let mut highest_found_blue_cubes = 0;
         for string in sets {
            for i in 0..string.len() {
               match string[i] {
                  "game" => game_number = string[i + 1].parse::<i32>().unwrap(),
                  "red" => {
                     let current_cubes = string[i - 1].parse::<i32>().unwrap();
                     if highest_found_red_cubes < current_cubes {
                        highest_found_red_cubes = current_cubes;
                     }
                  }, 
                  "green" => {
                     let current_cubes = string[i - 1].parse::<i32>().unwrap();
                     if highest_found_green_cubes < current_cubes {
                        highest_found_green_cubes = current_cubes;
                     }
                  },
                  "blue" => {
                     let current_cubes = string[i - 1].parse::<i32>().unwrap();
                     if highest_found_blue_cubes < current_cubes {
                        highest_found_blue_cubes = current_cubes;
                     }
                  },
                   _=> (),
               };
            }
         }
         if highest_found_red_cubes <= max_red_cubes && highest_found_green_cubes <= max_green_cubes && highest_found_blue_cubes <= max_blue_cubes {
            result += game_number;
            println!("pass: red:{}, green:{}, blue:{}, result {}", highest_found_red_cubes, highest_found_green_cubes, highest_found_blue_cubes, result);
         } else {
            println!("fail: red:{}, green:{}, blue:{}, result {}", highest_found_red_cubes, highest_found_green_cubes, highest_found_blue_cubes, result);
         }
         
      }
   }
   return result;
}


fn get_solution_part2(path: &str) -> i32 {
   let re = Regex::new(r"\d+|:|[a-z]+|;").unwrap();
   let mut result = 0;
   if let Ok(lines) = read_lines(path) {
      for line in lines {
         let current_line = line.unwrap().to_lowercase();
         let m: Vec<&str> = re.find_iter(current_line.as_str()).map(|m| m.as_str()).collect();
         let sets:Vec<_> = m.split(|&e| e == ";").filter(|v| !v.is_empty()).collect();
         let mut highest_found_red_cubes = 0;
         let mut highest_found_green_cubes = 0;
         let mut highest_found_blue_cubes = 0;
         for string in sets {
            for i in 0..string.len() {
               match string[i] {
                  "red" => {
                     let current_cubes = string[i - 1].parse::<i32>().unwrap();
                     if highest_found_red_cubes < current_cubes {
                        highest_found_red_cubes = current_cubes;
                     }
                  }, 
                  "green" => {
                     let current_cubes = string[i - 1].parse::<i32>().unwrap();
                     if highest_found_green_cubes < current_cubes {
                        highest_found_green_cubes = current_cubes;
                     }
                  },
                  "blue" => {
                     let current_cubes = string[i - 1].parse::<i32>().unwrap();
                     if highest_found_blue_cubes < current_cubes {
                        highest_found_blue_cubes = current_cubes;
                     }
                  },
                   _=> (),
               };
            }
         }
         result +=  highest_found_red_cubes * highest_found_green_cubes * highest_found_blue_cubes;
         
      }
   }
   return result;
}

fn main() {
   let part = match env::var("part") {
      Ok(val) => val,
      Err(_e) => "part1".to_string(),
   };

   if part == "part2" {
      println!("{}", get_solution_part2("input.txt"));
   } else {
      println!("{}", get_solution_part1("input.txt"));
   }
}