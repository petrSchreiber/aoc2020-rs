pub mod solution {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};

    fn read_vector_of_int32(file_name: &str) -> Result<Vec<i32>, Error> {
        let file = File::open(file_name)?;

        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| {
                line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            })
            .collect()
    }

    pub fn get_multiple_of_two_items_which_sum_2020(file_name: &str) -> Result<i32, Error> {
        match read_vector_of_int32(file_name) {
            Ok(integers) => {
                for a in &integers {
                    for b in &integers {
                        if a + b == 2020 {
                            return Ok(a * b);
                        }
                    }
                }

                Err(Error::new(ErrorKind::NotFound, file_name))
            },
            Err(err) => Err(err)
        }
    }
}
