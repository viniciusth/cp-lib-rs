use std::{
    error::Error,
    io::{BufWriter, Write},
};

use rand::Rng;

pub fn generate_case<T: Write>(mut output: BufWriter<T>) -> Result<(), Box<dyn Error + 'static>> {
    let mut rng = rand::thread_rng();
    writeln!(output, "1")?;
    let n = rng.gen_range(5..=100);
    writeln!(output, "{n}")?;
    for _ in 0..n {
        let x = rng.gen_range(0..2);
        if x == 0 {
            write!(output, ">")?;
        } else {
            write!(output, "<")?;
        }
    }
    writeln!(output)?;
    Ok(())
}
