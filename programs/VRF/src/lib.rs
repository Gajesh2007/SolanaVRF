use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock, program_option::COption, sysvar};
use switchboard_v2::AggregatorAccountData;
use std::convert::TryInto;

declare_id!("6gwQp7cAiVxTzmqKaxroD2Maeaa7bixveTe44XM5zQ4k");

#[program]
pub mod vrf {
    use super::*;

    pub fn read_result(ctx: Context<ReadResult>, _params: ReadResultParams) -> Result<u128> {  
        let c = clock::Clock::get().unwrap();
        
        let btc_aggregator = &ctx.accounts.btc_aggregator;
        let eth_aggregator = &ctx.accounts.eth_aggregator;
        let sol_aggregator = &ctx.accounts.sol_aggregator;

        let btc_val: f64 = AggregatorAccountData::new(btc_aggregator)?.get_result()?.try_into()?;
        let eth_val: f64 = AggregatorAccountData::new(eth_aggregator)?.get_result()?.try_into()?;
        let sol_val: f64 = AggregatorAccountData::new(sol_aggregator)?.get_result()?.try_into()?;

        let mut final_val: u128 = 0;
        if c.unix_timestamp % 2 == 0 {
            final_val = (btc_val*eth_val*sol_val).round() as u128;
        } else {
            final_val = (btc_val*sol_val).round() as u128;
        }

        msg!("Current feed result is {}!", final_val);
        Ok(final_val)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(params: ReadResultParams)]
pub struct ReadResult<'info> {
    /// CHECK: TODO
    pub btc_aggregator: AccountInfo<'info>,
    /// CHECK: TODO
    pub eth_aggregator: AccountInfo<'info>,
    /// CHECK: TODO
    pub sol_aggregator: AccountInfo<'info>,


}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReadResultParams {}
