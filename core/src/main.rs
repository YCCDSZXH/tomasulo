use anyhow::Result;
use core::comp::rs::RS;
use core::comp::Tomasulo;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let rs = RS.write().unwrap();
    println!("{}", rs);
    drop(rs);
    let mut file = File::open("test/1.s")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tomasulo = Tomasulo::default();
    tomasulo.init_instruction(&contents)?;
    tomasulo.run_to(10);

    Ok(())
}
