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

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<Strings>,
}

pub struct SetFavorites<'info>{
    #[account(mut)]
    pub user: Signer<'info>

    #[account(init_if_needed, payer=user, space=ANCHOR_DISCRIMINATION_SIZE + Favorites::INIT_SPACE, seeds=[b"favorites", user.key().as_ref(), bump])]
    pub favorites: Account<'info, Favorites>,
    
    pub system_program: Program<'info, System>,
}