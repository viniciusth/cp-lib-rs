use std::{
    error::Error,
    io::{BufWriter, Write},
};

use rand::Rng;

pub fn generate_case<T: Write>(mut output: BufWriter<T>) -> Result<(), Box<dyn Error + 'static>> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..=15);
    writeln!(
        &mut output,
        "1\n{n}\n"
    )?;

    for _ in 0..n {
        let x = rng.gen_range(1..=n);
        write!(&mut output, "{x} ")?;
    }
    writeln!(&mut output)?;

    Ok(())
}
