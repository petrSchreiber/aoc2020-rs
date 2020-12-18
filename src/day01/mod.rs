use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn solve(file_name: &str) -> Result<(), Error> {
    println!("-= Day 01 =-");

    match get_multiple_of_two_items_which_sum_2020(file_name) {
        Ok(result) => {
            println!("Result of part A: {:?}", result);
        }
        Err(error) => return Err(error),
    };

    match get_multiple_of_three_items_which_sum_2020(file_name) {
        Ok(result) => {
            println!("Result of part B: {:?}", result);
        }
        Err(error) => return Err(error),
    };

    Ok(())
}

fn get_multiple_of_two_items_which_sum_2020(file_name: &str) -> Result<i32, Error> {
    match read_vector_of_int32(file_name) {
        Ok(integers) => {
            for a in &integers {
                if a > &2019 {
                    continue;
                };

                for b in &integers {
                    if a + b == 2020 {
                        return Ok(a * b);
                    }
                }
            }

            Err(Error::new(ErrorKind::NotFound, file_name))
        }
        Err(err) => Err(err),
    }
}

fn get_multiple_of_three_items_which_sum_2020(file_name: &str) -> Result<i32, Error> {
    match read_vector_of_int32(file_name) {
        Ok(integers) => {
            for a in &integers {
                if a > &2019 {
                    continue;
                };

                for b in &integers {
                    if a + b > 2019 {
                        continue;
                    };

                    for c in &integers {
                        if a + b + c == 2020 {
                            return Ok(a * b * c);
                        }
                    }
                }
            }

            Err(Error::new(ErrorKind::NotFound, file_name))
        }
        Err(err) => Err(err),
    }
}

fn read_vector_of_int32(file_name: &str) -> Result<Vec<i32>, Error> {
    let file = File::open(file_name)?;

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
