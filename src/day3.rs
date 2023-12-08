use partition::partition_index;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String>
{
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1 (input: &[String]) -> isize {
    const LENGTH : usize = 12;

    let fold = input.iter().fold([0i128;LENGTH], |seed, b|{
        let val = |c| match c {'0' => 0, '1' => 1, _ => panic!("uh oh")};
        let digit = |d| seed[d] + val(b.chars().nth(d).unwrap());
        [digit(0), digit(1), digit(2), digit(3), 
         digit(4), digit(5), digit(6), digit(7), 
         digit(8), digit(9), digit(10), digit(11)]
    });

    let gamma = fold.iter().map(|i|if  2 * i  > input.len() as i128 {'1'} else {'0'}).collect::<String>();
    let epsilon = fold.iter().map(|i|if  2 * i  >  input.len()  as i128 {'0'} else {'1'}).collect::<String>();


    let gamma = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2 (input: &[String]) -> isize {
    const LENGTH : usize = 12;

    let mut oxygen_generator = input.to_vec().clone();

    for i in 0..LENGTH {
        let count = partition_index(&mut oxygen_generator, |p|{
            p.chars().nth(i).unwrap() == '0'
        });

        if 2 * count > oxygen_generator.len() {
            oxygen_generator.drain(count..);
        }
        else {
            oxygen_generator.drain(0..count);
        }

        if oxygen_generator.len() == 1 { break; }
    }

    let mut co2_scrubber = input.to_vec().clone();

    for i in 0..LENGTH {
        let count = partition_index(&mut co2_scrubber, |p|{
            p.chars().nth(i).unwrap() == '0'
        });
        
        if 2 * count > co2_scrubber.len() {
            co2_scrubber.drain(0..count);
        }
        else {
            co2_scrubber.drain(count..);
        }

        if co2_scrubber.len() == 1 { break; }
    }

 //   println!("oxygen {:?}", oxygen_generator);
 //   println!("co2 {:?}", co2_scrubber);

    let oxygen_generator_rating = oxygen_generator.iter().next().unwrap();
    let co2_scrubber_rating = co2_scrubber.iter().next().unwrap();


    let oxygen_generator_rating = isize::from_str_radix(oxygen_generator_rating.as_str(), 2).unwrap();
    let co2_scrubber_rating = isize::from_str_radix(co2_scrubber_rating.as_str(), 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    //use super::*;

}
