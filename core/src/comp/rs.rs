use std::{fmt::Display, sync::RwLock};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

use super::pc::Instrution;
use super::reg::{RegState, REG_GROUP};

lazy_static! {
    pub static ref RS: RwLock<Rs> = RwLock::new(Rs::default());
}

#[derive(Default)]
pub struct Rs {
    pub load: [Slot; 3],
    pub store: [Slot; 3],
    pub add: [Slot; 3],
    pub mul: [Slot; 2],
}

impl Rs {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Display for Rs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "add")?;
        self.add.iter().try_for_each(|v| write!(f, "{}", v))?;
        writeln!(f, "mul")?;
        self.mul.iter().try_for_each(|v| write!(f, "{}", v))?;
        writeln!(f, "load")?;
        self.load.iter().try_for_each(|v| write!(f, "{}", v))?;
        writeln!(f, "store")?;
        self.store.iter().try_for_each(|v| write!(f, "{}", v))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RsType {
    Load,
    Store,
    Add,
    Mul,
}

impl Display for RsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RsType::Add => "add",
                RsType::Mul => "mul",
                RsType::Load => "load",
                RsType::Store => "store",
                // _ => "unknown",
            }
        )
    }
}

#[derive(Default, Clone, Copy)]
pub struct Slot {
    pub busy: bool,
    pub time: i8,
    pub addr: Option<i32>,
    pub op: Option<Instrution>,
    pub vj: Option<i32>,
    pub vk: Option<i32>,
    pub qj: Option<(RsType, u8)>,
    pub qk: Option<(RsType, u8)>,
}

// impl Default for Slot {
//     fn default() -> Self {
//         Self {
//             busy: false,
//             time: 0,
//             addr: None,
//             op: None,
//             vj: None,
//             vk: None,
//             qj: None,
//             qk: None,
//         }
//     }
// }

impl Slot {
    fn reset(&mut self) {
        self.busy = false;
        self.time = 0;
        self.addr = None;
        self.op = None;
        self.vj = None;
        self.vk = None;
        self.qj = None;
        self.qk = None;
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            self.busy,
            self.time,
            self.addr,
            {
                if let Some(instr) = self.op {
                    match instr {
                        Instrution::Add(_, _, _) => "add",
                        Instrution::Sub(_, _, _) => "sub",
                        Instrution::Mul(_, _, _) => "mul",
                        Instrution::Lw(_, _, _) => "lw",
                        Instrution::Sw(_, _, _) => "sw",
                        _ => "unknown",
                    }
                } else {
                    ""
                }
            },
            self.vj,
            self.vk,
            {
                if let Some(qj) = self.qj {
                    format!("{}{}", qj.0, qj.1)
                } else {
                    "".to_string()
                }
            },
            {
                if let Some(qk) = self.qk {
                    format!("{}{}", qk.0, qk.1)
                } else {
                    "".to_string()
                }
            }
        )
    }
}

impl Rs {
    pub fn try_issue(&mut self, instr: Instrution) -> Result<()> {
        match instr {
            Instrution::Lw(rdi, imm, rsi) => {
                let (index, slot) = self
                    .load
                    .iter_mut()
                    .enumerate()
                    .find(|v| !v.1.busy)
                    .ok_or(anyhow!("No Slot"))?;
                slot.busy = true;
                slot.time = 2;
                slot.op = Some(instr);
                let mut rg = REG_GROUP.write().unwrap();
                let rs = rg.get_reg(rsi as u8);
                if rs.state.is_none() {
                    slot.vj = Some(rs.value);
                } else {
                    slot.qj = rs.state;
                }
                slot.addr = Some(imm as i32);
                rg.set_state(rdi as u8, Some((RsType::Load, index as u8)));
            }
            Instrution::Sw(rs1i, imm, rs2i) => {
                let slot = self
                    .store
                    .iter_mut()
                    .find(|v| !v.busy)
                    .ok_or(anyhow!("No Slot"))?;
                slot.busy = true;
                slot.time = 2;
                slot.op = Some(instr);
                let rg = REG_GROUP.read().unwrap();
                let rs1 = rg.get_reg(rs1i as u8);
                let rs2 = rg.get_reg(rs2i as u8);
                if rs1.state.is_none() {
                    slot.vj = Some(rs1.value);
                } else {
                    slot.qj = rs2.state;
                }
                if rs2.state.is_none() {
                    slot.vk = Some(rs2.value);
                } else {
                    slot.qk = rs2.state;
                }
                slot.addr = Some(imm as i32);
            }
            Instrution::Add(rdi, rs1i, rs2i) => {
                let (index, slot) = self
                    .add
                    .iter_mut()
                    .enumerate()
                    .find(|v| !v.1.busy)
                    .ok_or(anyhow!("No Slot"))?;
                slot.busy = true;
                slot.time = 2;
                slot.op = Some(instr);
                let mut rg = REG_GROUP.write().unwrap();
                let rs1 = rg.get_reg(rs1i as u8);
                let rs2 = rg.get_reg(rs2i as u8);
                if rs1.state.is_none() {
                    slot.vj = Some(rs1.value);
                } else {
                    slot.qj = rs1.state;
                }
                if rs2.state.is_none() {
                    slot.vk = Some(rs2.value)
                } else {
                    slot.qk = rs2.state
                }
                rg.set_state(rdi as u8, Some((RsType::Add, index as u8)));
            }
            Instrution::Sub(rdi, rs1i, rs2i) => {
                let (index, slot) = self.add.iter_mut().enumerate().find(|v| !v.1.busy).unwrap();
                slot.busy = true;
                slot.time = 2;
                slot.op = Some(instr);
                let mut rg = REG_GROUP.write().unwrap();
                let rs1 = rg.get_reg(rs1i as u8);
                let rs2 = rg.get_reg(rs2i as u8);
                if rs1.state.is_none() {
                    slot.vj = Some(rs1.value);
                } else {
                    slot.qj = rs1.state;
                }
                if rs2.state.is_none() {
                    slot.vk = Some(rs2.value)
                } else {
                    slot.qk = rs2.state
                }
                rg.set_state(rdi as u8, Some((RsType::Add, index as u8)));
            }
            Instrution::Mul(rdi, rs1i, rs2i) => {
                if let Some((index, slot)) = self.mul.iter_mut().enumerate().find(|v| !v.1.busy) {
                    slot.busy = true;
                    slot.time = 10;
                    slot.op = Some(instr);
                    let mut rg = REG_GROUP.write().unwrap();
                    let rs1 = rg.get_reg(rs1i as u8);
                    let rs2 = rg.get_reg(rs2i as u8);
                    if rs1.state.is_none() {
                        slot.vj = Some(rs1.value);
                    } else {
                        slot.qj = rs1.state;
                    }
                    if rs2.state.is_none() {
                        slot.vk = Some(rs2.value)
                    } else {
                        slot.qk = rs2.state
                    }
                    rg.set_state(rdi as u8, Some((RsType::Mul, index as u8)));
                }
            }
            #[allow(unused)]
            Instrution::Div(r1, r2, r3) => {}
        }
        Ok(())
    }
    pub fn update(&mut self) {
        let mut bus = false;
        let slot = self
            .add
            .iter_mut()
            .enumerate()
            .find(|(_, v)| v.busy && v.vj.is_some() && v.vk.is_some());
        let mut op_done = (None, 0);
        if let Some((index, slot)) = slot {
            if slot.time > 0 {
                slot.time -= 1;
            } else {
                let value = match slot.op {
                    Some(Instrution::Add(_, _, _)) => slot.vj.unwrap() + slot.vk.unwrap(),
                    Some(Instrution::Sub(_, _, _)) => slot.vj.unwrap() - slot.vk.unwrap(),
                    _ => 0,
                };
                op_done = (Some((RsType::Add, index as u8)), value);
                // let value = slot.vj.unwrap() + slot.vk.unwrap();
                slot.reset();
            }
        }
        let mut rg = REG_GROUP.write().unwrap();
        if op_done.0.is_some() && !bus {
            bus = true;
            rg.refresh_reg_state(op_done.0, op_done.1);
            self.refresh(op_done.0, op_done.1);
        }

        op_done = (None, 0);
        let slot = self
            .mul
            .iter_mut()
            .enumerate()
            .find(|(_, v)| v.busy && v.vj.is_some() && v.vk.is_some());
        if let Some((index, slot)) = slot {
            if slot.time > 0 {
                slot.time -= 1;
            } else {
                let value = slot.vj.unwrap() * slot.vk.unwrap();
                op_done = (Some((RsType::Mul, index as u8)), value);
                if !bus {
                    slot.reset();
                }
            }
        }
        if op_done.0.is_some() && !bus {
            bus = true;
            rg.refresh_reg_state(op_done.0, op_done.1);
            self.refresh(op_done.0, op_done.1);
        }

        op_done = (None, 0);
        let slot = self
            .load
            .iter_mut()
            .enumerate()
            .find(|(_, v)| v.busy && v.vj.is_some() && v.addr.is_some());
        if let Some((index, slot)) = slot {
            if slot.time > 0 {
                slot.time -= 1;
            } else {
                // TODO: add mem
                let value = 1;
                op_done = (Some((RsType::Load, index as u8)), value);
                if !bus {
                    slot.reset();
                }
            }
        }
        if op_done.0.is_some() && !bus {
            rg.refresh_reg_state(op_done.0, op_done.1);
            self.refresh(op_done.0, op_done.1);
        }
    }

    fn refresh(&mut self, state: RegState, value: i32) {
        self.add.iter_mut().for_each(|v| {
            if v.qj == state {
                v.qj = None;
                v.vj = Some(value);
            }
            if v.qk == state {
                v.qk = None;
                v.vk = Some(value);
            }
        });
        self.mul.iter_mut().for_each(|v| {
            if v.qj == state {
                v.qj = None;
                v.vj = Some(value);
            }
            if v.qk == state {
                v.qk = None;
                v.vk = Some(value);
            }
        });
        self.load.iter_mut().for_each(|v| {
            if v.qj == state {
                v.qj = None;
                v.vj = Some(value);
            }
            if v.qk == state {
                v.qk = None;
                v.vk = Some(value);
            }
        });
        self.store.iter_mut().for_each(|v| {
            if v.qj == state {
                v.qj = None;
                v.vj = Some(value);
            }
            if v.qk == state {
                v.qk = None;
                v.vk = Some(value);
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::RS;
    use crate::comp::pc::Instrution;

    #[test]
    fn test_issue() {
        let str = "add x15 x8 x8";
        let instr: Instrution = str.into();
        let mut rs = RS.write().unwrap();
        let res = rs.try_issue(instr);
        assert!(res.is_ok())
    }

    #[test]
    fn test_update() {
        let str = "mul x16 x15 x8";
        let instr: Instrution = str.into();
        let mut rs = RS.write().unwrap();
        let res = rs.try_issue(instr);
        rs.update();
        rs.update();
        rs.update();
        rs.update();
        assert!(res.is_ok())
    }
}
