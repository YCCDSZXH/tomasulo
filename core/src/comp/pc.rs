use anyhow::{anyhow, Result};
use core::panic;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::RwLock};

use lazy_static::lazy_static;

use super::rs::RS;

lazy_static! {
    pub static ref PC: Box<RwLock<Pc>> = Box::new(RwLock::new(Pc::default()));
}

#[derive(Serialize, Deserialize, Default)]
pub struct Pc {
    pub index: u32,
    pub instrutions: Vec<Instrution>,
}

impl Display for Pc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = serde_json::to_string(self).unwrap();
        write!(f, "{}", res)
    }
}

impl Pc {
    pub fn reset_with_instrutions(&mut self, instrutions: Vec<Instrution>) {
        self.instrutions = instrutions;
        self.index = 0;
    }
    pub fn run(&mut self) -> Result<()> {
        let instr = self
            .instrutions
            .get(self.index as usize)
            .ok_or(anyhow!("No rest instruction"))?;
        let mut rs = RS.write().unwrap();
        rs.try_issue(instr.to_owned())?;
        self.index += 1;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Instrution {
    Lw(i8, i8, i8),
    Sw(i8, i8, i8),
    Add(i8, i8, i8),
    Sub(i8, i8, i8),
    Mul(i8, i8, i8),
    Div(i8, i8, i8),
}

impl Instrution {
    pub fn to_tuple(&self) -> (String, String, String, String) {
        match self {
            Self::Lw(rd, imm, rs) => (
                "lw".to_owned(),
                "x".to_owned() + &rd.to_string(),
                imm.to_string(),
                "x".to_owned() + &rs.to_string(),
            ),
            Self::Sw(rd, imm, rs) => (
                "sw".to_owned(),
                "x".to_owned() + &rd.to_string(),
                imm.to_string(),
                "x".to_owned() + &rs.to_string(),
            ),
            Self::Add(rd, rs1, rs2) => (
                "add".to_owned(),
                "x".to_owned() + &rd.to_string(),
                "x".to_owned() + &rs1.to_string(),
                "x".to_owned() + &rs2.to_string(),
            ),
            Self::Sub(rd, rs1, rs2) => (
                "sub".to_owned(),
                "x".to_owned() + &rd.to_string(),
                "x".to_owned() + &rs1.to_string(),
                "x".to_owned() + &rs2.to_string(),
            ),
            Self::Mul(rd, rs1, rs2) => (
                "mul".to_owned(),
                "x".to_owned() + &rd.to_string(),
                "x".to_owned() + &rs1.to_string(),
                "x".to_owned() + &rs2.to_string(),
            ),
            Self::Div(rd, rs1, rs2) => (
                "div".to_owned(),
                "x".to_owned() + &rd.to_string(),
                "x".to_owned() + &rs1.to_string(),
                "x".to_owned() + &rs2.to_string(),
            ),
        }
    }
}
impl From<&str> for Instrution {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(' ').collect();
        if parts.len() != 4 {
            println!("{}", value);
            panic!("Parse Error")
        }
        match parts[0] {
            "lw" => {
                let r1 = parts[1]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let imm = parts[2].parse::<i8>().expect("Check your instruction");
                let r2 = parts[3]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                Instrution::Lw(r1, imm, r2)
            }
            "sw" => {
                let r1 = parts[1]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let imm = parts[2].parse::<i8>().expect("Check your instruction");
                let r2 = parts[3]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                Instrution::Sw(r1, imm, r2)
            }
            "add" => {
                let r1 = parts[1]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r2 = parts[2]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r3 = parts[3]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                Instrution::Add(r1, r2, r3)
            }
            "sub" => {
                let r1 = parts[1]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r2 = parts[2]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r3 = parts[3]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                Instrution::Sub(r1, r2, r3)
            }
            "mul" => {
                let r1 = parts[1]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r2 = parts[2]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r3 = parts[3]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                Instrution::Mul(r1, r2, r3)
            }
            "div" => {
                let r1 = parts[1]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r2 = parts[2]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                let r3 = parts[3]
                    .split_at(1)
                    .1
                    .parse::<i8>()
                    .expect("Check your instruction");
                Instrution::Div(r1, r2, r3)
            }
            _ => {
                todo!()
            }
        }
    }
}

impl Display for Instrution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Lw(_, _, _) => "load",
                Self::Sw(_, _, _) => "store",
                Self::Add(_, _, _) => "add",
                Self::Mul(_, _, _) => "mul",
                Self::Sub(_, _, _) => "sub",
                Self::Div(_, _, _) => "div",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::Instrution;

    #[test]
    fn string_to_instruction() {
        let str = "lw x15 -20 x8";
        let instr: Instrution = str.into();
        let str = serde_json::to_string(&instr).unwrap();
        assert_eq!(str, r#"{"Lw":[15,-20,8]}"#);
    }
}
