use super::rs::RsType;
use lazy_static::lazy_static;
use std::{fmt::Display, sync::RwLock};

pub type RegState = Option<(RsType, u8)>;

lazy_static! {
    pub static ref REG_GROUP: RwLock<RegGroup> = RwLock::new(RegGroup::default());
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Reg {
    pub state: RegState,
    pub value: i32,
}
#[derive(Debug)]
pub struct RegGroup {
    pub regs: [Reg; 32],
}

impl Default for RegGroup {
    fn default() -> Self {
        let mut regs = [Reg::default(); 32];
        regs.iter_mut().enumerate().for_each(|(i, v)| {
            v.value = i as i32;
        });
        Self { regs }
    }
}
impl Display for RegGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.regs
            .iter()
            .try_for_each(|v| write!(f, "{:?} ", v.state))
    }
}

impl RegGroup {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn get_reg(&self, index: u8) -> &Reg {
        self.regs.get(index as usize).unwrap()
    }
    pub fn set_state(&mut self, index: u8, state: RegState) {
        self.regs.get_mut(index as usize).unwrap().state = state;
    }
    pub fn refresh_reg_state(&mut self, state: RegState, value: i32) {
        println!("{}\n{:?}", self, state);
        self.regs
            .iter_mut()
            .filter(|v| v.state == state)
            .for_each(|v| {
                v.state = None;
                v.value = value;
            });
        println!("{}\n", self);
    }
}
