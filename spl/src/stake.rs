use anchor_lang::{
    context::CpiContext,
    solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        stake::{
            self,
            program::ID,
            state::{StakeAuthorize, StakeState},
        },
    },
    Accounts, Result,
};
use borsh::BorshDeserialize;
use std::ops::Deref;

// CPI functions

pub fn authorize<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Authorize<'info>>,
    stake_authorize: StakeAuthorize,
    custodian: Option<AccountInfo<'info>>,
) -> Result<()> {
    let ix = stake::instruction::authorize(
        ctx.accounts.stake.key,
        ctx.accounts.authorized.key,
        ctx.accounts.new_authorized.key,
        stake_authorize,
        custodian.as_ref().map(|c| c.key),
    );
    let mut account_infos = vec![
        ctx.accounts.stake,
        ctx.accounts.clock,
        ctx.accounts.authorized,
    ];
    if let Some(c) = custodian {
        account_infos.push(c);
    }
    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}

// CPI accounts

#[derive(Accounts)]
pub struct Authorize<'info> {
    /// The stake account to be updated
    pub stake: AccountInfo<'info>,

    /// The existing authority
    pub authorized: AccountInfo<'info>,

    /// The new authority to replace the existing authority
    pub new_authorized: AccountInfo<'info>,

    /// Clock sysvar
    pub clock: AccountInfo<'info>,
}

// state

#[derive(Clone)]
pub struct StakeAccount(StakeState);

impl anchor_lang::AccountDeserialize for StakeAccount {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        StakeState::deserialize(buf).map(Self).map_err(Into::into)
    }
}

impl anchor_lang::AccountSerialize for StakeAccount {}

impl anchor_lang::Owner for StakeAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for StakeAccount {
    type Target = StakeState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
