use anchor_lang::prelude::*;

// Insert Anchor Program Address
declare_id!("");

pub const ANCHOR_DISCRIMINATION_SIZE: usize = 8;

#[program]
pub mod favorites{
    use super ::*;

    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<Strings>)-> Return <()>{
        msg!("Greeting Macro!!!", context.program.id);
        let user_public_key = context.accounts.user.key();

        mes!("User {user_public_key}'s favorite number is {number}, and their favorite color is {color}. Their favorite hobbies are {hobbies:?}")
        context.accounts.favorites.set_innner( Favorites{number, color, hobbies})
    };

    Ok(())
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