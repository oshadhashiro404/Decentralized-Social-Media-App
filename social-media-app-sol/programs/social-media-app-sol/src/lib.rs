/// Include libraries for program
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;

// Declare program ID
declare_id!("GNSDgX32cCeuuN96GRiBmFE4BAXqgFpftqxSCzDrdkTd");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod decentra {
    use super::*;

    /// Create state to save the post counts
    /// There is only one state in the program
    /// This account should be initialized before post
    pub fn create_state(
        ctx: Context<CreateState>,
    ) -> ProgramResult {
        
        let state = &mut ctx.accounts.state;

        state.authority = ctx.accounts.authority.key();

        state.post_count = 0;
        Ok(())
    }

    /// Create post
    /// @param text:        text of post
    /// @param poster_name: name of post creator
    /// @param poster_url:  url of post creator avatar
    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        poster_name: String,
        poster_url: String,
    ) -> ProgramResult {

        let state = &mut ctx.accounts.state;

        let post = &mut ctx.accounts.post;
        post.authority = ctx.accounts.authority.key();
        post.text = text;
        post.poster_name = poster_name;
        post.poster_url = poster_url;
        post.comment_count = 0;
        post.index = state.post_count;
        post.post_time = ctx.accounts.clock.unix_timestamp;
        state.post_count += 1;
        Ok(())
    }

    /// Create comment for post
    /// @param text:            text of comment
    /// @param commenter_name:  name of comment creator
    /// @param commenter_url:   url of comment creator avatar
    pub fn create_comment(
        ctx: Context<CreateComment>,
        text: String,
        commenter_name: String,
        commenter_url: String,
    ) -> ProgramResult {

        let post = &mut ctx.accounts.post;

        let comment = &mut ctx.accounts.comment;
        comment.authority = ctx.accounts.authority.key();
        comment.text = text;
        comment.commenter_name = commenter_name;
        comment.commenter_url = commenter_url;
        comment.index = post.comment_count;
        comment.post_time = ctx.accounts.clock.unix_timestamp;

        post.comment_count += 1;

        Ok(())
    }
}

/// Contexts
/// CreateState context
#[derive(Accounts)]
pub struct CreateState<'info> {
    // Authenticating state account
    #[account(
        init,
        seeds = [b"state".as_ref()],
        bump,
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

/// CreatePost context
#[derive(Accounts)]
pub struct CreatePost<'info> {
    // Authenticate state account
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    // Authenticate post account
    #[account(
        init,
        // Post account use string "post" and index of post as seeds
        seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
    )]
    pub post: Account<'info, PostAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

/// CreateComment context
#[derive(Accounts)]
pub struct CreateComment<'info> {
    // Authenticate post account
    #[account(mut, seeds = [b"post".as_ref(), post.index.to_be_bytes().as_ref()], bump)]
    pub post: Account<'info, PostAccount>,

    // Authenticate comment account
    #[account(
        init,
        // Post account use string "comment", index of post and index of comment per post as seeds
        seeds = [b"comment".as_ref(), post.index.to_be_bytes().as_ref(), post.comment_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<CommentAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
    )]
    pub comment: Account<'info, CommentAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

// State Account Structure
#[account]
pub struct StateAccount {
    // Signer address
    pub authority: Pubkey,

    // Post count
    pub post_count: u64,
}

// Post Account Structure
#[account]
pub struct PostAccount {
    // Signer address
    pub authority: Pubkey,

    // Post text
    pub text: String,

    // Post creator name
    pub poster_name: String,

    // Post creator url
    pub poster_url: String,

    // Comment counts of post
    pub comment_count: u64,

    // Post index
    pub index: u64,

    // Post time
    pub post_time: i64,
}

// Comment Account Structure
#[account]
pub struct CommentAccount {
    // Signer address
    pub authority: Pubkey,

    // Comment text
    pub text: String,

    // commenter_name
    pub commenter_name: String,

    // commenter_url
    pub commenter_url: String,

    // Comment index
    pub index: u64,

    // Post time
    pub post_time: i64,
}
