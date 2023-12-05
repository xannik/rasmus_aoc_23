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

   let re = Regex::new(r"\d").unwrap();
   let mut result = 0;
   if let Ok(lines) = read_lines(path) {
      for line in lines {
         let current_line = line.unwrap();
         let m: Vec<&str> = re.find_iter(current_line.as_str()).map(|m| m.as_str()).collect();
         println!("Found matches: {:?}", m);
         let mut number_str: String = "".to_owned();
         number_str.push_str(m.first().unwrap());
         number_str.push_str(m.last().unwrap());
         
         
         println!("number: {}", number_str);
         result += number_str.parse::<i32>().unwrap();
         
      }
   }
   println!("{}", result);
   return result;
}


fn get_solution_part2(path: &str) -> i32 {
   
   let re = Regex::new(r"\d").unwrap();
   let mut result = 0;
   let word_map = HashMap::from([
      ("one","o1e"),
      ("two","t2o"),
      ("three","t3e"),
      ("four","f4r"),
      ("five","f5e"),
      ("six","s6x"),
      ("seven","s7n"),
      ("eight","e8t"),
      ("nine","n9e"),
   ]);
   if let Ok(lines) = read_lines(path) {
      for line in lines {
         let mut current_line = line.unwrap();
         for (key,value) in &word_map {
            current_line = current_line.replace(key, value);
            //println!("{}",current_line);
         }
         let m: Vec<&str> = re.find_iter(current_line.as_str()).map(|m| m.as_str()).collect();
         let mut number_str: String = "".to_owned();
         number_str.push_str(m.first().unwrap());
         number_str.push_str(m.last().unwrap());
      
         
         result += number_str.parse::<i32>().unwrap();
         //println!("Found matches: {:?}, index {}", m, i);  

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