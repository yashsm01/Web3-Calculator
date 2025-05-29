use anchor_lang::prelude::*;

declare_id!("8BxLL43F4vkgtX1uo2Ub1RMt3JsiDYy3G7fRySeF679S");

#[program]
pub mod my_project {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        calculator.result = 0; // Initialize to avoid uninitialized state
        calculator.reminder = 0; // Same here
        Ok(())
    }
    pub fn add(ctx: Context<Addition>, num1:i64, num2:i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn substract(ctx: Context<Subtraction>, num1:i64, num2:i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1:i64, num2:i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1:i64, num2:i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.reminder = num1 % num2;
        Ok(())
    }

    pub fn remainder(ctx: Context<Remainder>, num1:i64, num2:i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.reminder = num1 % num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 64)] // 8 (discriminator) + estimated space
    pub calculator: Account<'info, Calculator>,// Account with dynamic space for String
    #[account(mut)]// Payer account
    pub user: Signer<'info>,// User who pays for the account creation
    pub system_program: Program<'info, System>,// System program for account creation
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}


#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Remainder<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}


#[account]
pub struct Calculator {
    pub greeting: String,   // Anchor dynamically allocates space for String
    pub result: i64,
    pub reminder: i64,
}
