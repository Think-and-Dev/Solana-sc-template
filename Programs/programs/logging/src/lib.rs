use anchor_lang::prelude::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    log::{sol_log_compute_units, sol_log_params, sol_log_slice},
    msg,
    pubkey::Pubkey,
};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod logging {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Log a string
        msg!("static string");

        // Log a slice
        //sol_log_slice(instruction_data); --> Not working

        // Log a formatted message, use with caution can be expensive
        //msg!("formatted {}: {:?}", "message", instruction_data);

        // Log a public key
        id().log();

        // Log all the program's input parameters
        //sol_log_params(accounts, instruction_data);

        // Log the number of compute units remaining that the program can consume.
        sol_log_compute_units();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
