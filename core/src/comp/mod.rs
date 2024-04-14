use anyhow::Result;

use crate::comp::pc::Instrution;
use crate::comp::pc::PC;
use crate::comp::reg::REG_GROUP;

use self::rs::RS;
pub mod pc;
pub mod reg;
pub mod rs;

#[derive(Default)]
pub struct Tomasulo {}

impl Tomasulo {
    pub fn init_instruction(&self, instr: &str) -> Result<()> {
        let mut rs = RS.write().unwrap();
        rs.reset();
        let mut rg = REG_GROUP.write().unwrap();
        rg.reset();
        let mut pc = PC.write().unwrap();
        let instrs = instr
            .split('\n')
            .filter(|v| v.len() > 2)
            .map(|v| v.into())
            .collect::<Vec<Instrution>>();
        pc.reset_with_instrutions(instrs);
        println!("{}", pc);
        Ok(())
    }
    fn step(&self) {
        let mut pc = PC.write().unwrap();
        let mut rs = RS.write().unwrap();
        rs.update();
        drop(rs);
        let _ = pc.run();
        let rs = RS.read().unwrap();
        println!("{}", rs);
    }
    pub fn run_to(&self, i: i32) {
        for _ in 0..i {
            self.step();
        }
    }
}
