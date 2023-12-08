use std::ops::Neg;
use regex::Regex;
pub struct Position(i64, i64);

#[aoc_generator(day2,part1)]
pub fn input_generator_part1(input: &str) -> Vec<Position>
{
    lazy_static! {
                static ref RE : Regex = Regex::new(r"(\w+) (\d+)").unwrap();
    }

    RE.captures_iter(input)
    .map(|cap|{
        match cap.get(1).unwrap().as_str() {
            "forward" => Position(cap.get(2).unwrap().as_str().parse().unwrap(), 0),
            "up" => Position(0, cap.get(2).unwrap().as_str().parse::<i64>().unwrap().neg()),
            "down" => Position(0, cap.get(2).unwrap().as_str().parse().unwrap()),
            _ => panic!("Unexpected regex cap"),
        }
    }
    ).collect()
}


#[aoc(day2, part1)]
pub fn solve_part1(input: &[Position]) -> i64 {
    let pos = input.iter().fold(Position(0,0), |a,b |{
        Position(a.0 + b.0, a.1 + b.1)
    });
    pos.0 * pos.1
}

pub enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[aoc_generator(day2,part2)]
pub fn input_generator_part2(input: &str) -> Vec<Instruction>
{
    lazy_static! {
                static ref RE : Regex = Regex::new(r"(\w+) (\d+)").unwrap();
    }

    RE.captures_iter(input)
    .map(|cap|{
        match cap.get(1).unwrap().as_str() {
            "forward" => Instruction::Forward(cap.get(2).unwrap().as_str().parse().unwrap()),
            "up" => Instruction::Up(cap.get(2).unwrap().as_str().parse().unwrap()),
            "down" => Instruction::Down(cap.get(2).unwrap().as_str().parse().unwrap()),
            _ => panic!("Unexpected regex cap"),
        }
    }
    ).collect()
}

struct PositionAim {
    h : i64,
    d : i64,
    aim : i64,
}

impl PositionAim {
    fn make(h : i64, d : i64, aim : i64) -> PositionAim{
        PositionAim{h, d, aim}
    }
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Instruction]) -> i64 {
    let pos = input.iter().fold(PositionAim{h:0,d:0,aim:0}, |pa, i |{
       match i{
           Instruction::Forward(f) =>  PositionAim::make(pa.h + f, pa.d + f * pa.aim, pa.aim),
           Instruction::Down(d)=> PositionAim::make(pa.h, pa.d, pa.aim + d),
           Instruction::Up(u)=>PositionAim::make(pa.h, pa.d, pa.aim - u),
       }
    });
    pos.h * pos.d
}


#[cfg(test)]
mod tests {
    //use super::*;

}
