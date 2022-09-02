use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("6Et5YsqS2LiMefFmf7iwQ2L2vmxmrzHHJeeuH12Nboou");

#[derive(Debug)]
#[account]
pub struct Calculator {
    owner: Pubkey,
    display: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub fn init_calculator_handler(mut ctx: Context<InitCalculator>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut calculator = &mut ctx.accounts.calculator;
    let mut calculator = calculator;

    calculator.owner = owner.key();

    Ok(())
}

pub fn reset_calculator_handler(mut ctx: Context<ResetCalculator>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut calculator = &mut ctx.accounts.calculator;

    msg!("{} {} {}", owner.key(), "has reset", calculator.key());

    require!(owner.key() == calculator.owner, ProgramError::E000);

    calculator.display = 0;

    Ok(())
}

pub fn do_operation_handler(
    mut ctx: Context<DoOperation>,
    mut op: Operation,
    mut num: i64,
) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut calculator = &mut ctx.accounts.calculator;

    require!(owner.key() == calculator.owner, ProgramError::E001);

    if op == Operation::ADD {
        calculator.display += num;
    } else {
        if op == Operation::SUB {
            calculator.display -= num;
        } else {
            if op == Operation::MUL {
                calculator.display *= num;
            } else {
                if op == Operation::DIV {
                    calculator.display /= num;
                }
            }
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitCalculator<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["Calculator".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Calculator>()
    )]
    pub calculator: Box<Account<'info, Calculator>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetCalculator<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub calculator: Box<Account<'info, Calculator>>,
}

#[derive(Accounts)]
pub struct DoOperation<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub calculator: Box<Account<'info, Calculator>>,
}

#[program]
pub mod calculator {
    use super::*;

    pub fn init_calculator(ctx: Context<InitCalculator>) -> Result<()> {
        init_calculator_handler(ctx)
    }

    pub fn reset_calculator(ctx: Context<ResetCalculator>) -> Result<()> {
        reset_calculator_handler(ctx)
    }

    pub fn do_operation(ctx: Context<DoOperation>, op: Operation, num: i64) -> Result<()> {
        do_operation_handler(ctx, op, num)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("This is not your calculator ser")]
    E000,
    #[msg("This isnt your calculator ser")]
    E001,
}
