use anchor_lang::prelude::*;

declare_id!("");

pub const ANCHOR_DISCRIMINATION_SIZE: usize = 8;

#[program]
pub mod favorites{
    use super ::*;

    pub fn set_favorites()-> Return <()>{
        // Body
    }
}