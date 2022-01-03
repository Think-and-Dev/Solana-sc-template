use anchor_lang::prelude::*;
/**WE REQUIRE THE STRUCT SetData**/
use cpi_puppet::cpi::accounts::SetData;
/**WE REQUIRE THE PROGRAM **/
use cpi_puppet::program::CpiPuppet;
/**WE REQUIRE SELF(?) AND THE STRUCT DATA**/
use cpi_puppet::{self, Data};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
pub mod cpi_puppet_master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> ProgramResult {
        /**GET PROGRAM ACCOUNT**/
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        /**Definition of the struct SetData**/
        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet.to_account_info(),
        };
        /** GENERATE PUPPET CONTEXT **/
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        /** CALL PROGRAM METHOD ***/
        cpi_puppet::cpi::set_data(cpi_ctx, data)
    }

}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, CpiPuppet>,
}