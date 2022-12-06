use std::{
        fs::read_to_string, 
        io::Result
    };

pub mod task_code;

fn main() -> Result<()> {
    let data = do_task()?;
    println!("{data}");
    Ok(())
}

fn do_task() -> Result<u64> {
    let data_loc = format!("./task_data/6.txt");
    let data = read_to_string(data_loc)?;
    Ok(task_code::task_6::part_1(data))
}