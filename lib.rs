// lib.rs

use anchor_lang::prelude::*;
use solana_program::sysvar::clock::Clock;
use solana_program::program::invoke_signed;
use spl_token::state::{Account as TokenAccount, Mint};
use spl_transfer_hook_interface::collect_extra_account_metas_signer_seeds;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

// Declare the unique program identifier for Solana Token-2022 Program
declare_id!("A3kVUhYP8SdZRN9VjqXpS2vVbE4BQkj9BK7cgFmX1NGQ");

// Sha256(spl-transfer-hook-interface:execute)[..8]
pub const EXECUTE_IX_TAG_LE: [u8; 8] = [105, 37, 101, 197, 75, 251, 102, 26];

// Solana 2022 Token Standard (Solana Extension)
pub const SOLANA_2022_TOKEN: &str = "Token-2022 Program";
pub const TOKEN_PROGRAM_ID: &Pubkey = &solana_program::system_program::ID;

// Constants for Bark Token
pub const TOKEN_NAME: &str = "Bark Token";
pub const TOKEN_SYMBOL: &str = "BARK";
pub const TOKEN_DECIMALS: u8 = 9;
pub const MAX_SUPPLY: u64 = 20_000_000_000;  // 20 billion Bark tokens
pub const BURNING_RATE: f64 = 0.02;  // 2% Quarterly

// Collecting Transaction fees for Bark
pub const FEE_DECIMALS: u8 = 2;  // Adjust as needed
pub const FEE_SYMBOL: &str = "BARK_FEE";  // Adjust as needed
pub const FEE_PERCENTAGE: f64 = 2.0;  // 2%

// Treasury Wallet for Bark
pub const TREASURY_WALLET: &str = "8DosypWP5rR5REnpkjwbwumpLkdXR1dwyRyzpUYGwqnA";

#[program]
pub mod bark {
    use super::*;

    // Version 1.0.0
    const PROGRAM_VERSION: u8 = 1;

    #[state]
    pub struct State {
        pub owner: Pubkey,
        pub paused: bool,
        pub version: u8,
        pub decimals: u8,
        pub symbol: String,
        pub name: String,
        pub max_supply: u64,
        pub total_supply: u64,
        pub burning_rate: f64,
        pub metadata_decimals: u8,
        pub metadata_symbol: String,
        pub metadata_name: String,
        pub metadata_uri: String,
        pub metadata_creator: String,
        pub metadata_creator_url: String,
        pub description: String,
        pub image: String,
        pub token_program_id: Pubkey,
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, seeds = [authority.key().as_ref()], bump, payer = authority, space = 8 + 320)]
        pub state: Account<'info, State>,
        #[account(init, mint::decimals = decimals, mint::authority = authority, mint::mint_authority = authority, mint::freeze_authority = None, mint::supply = total_supply)]
        pub mint: Account<'info, Mint>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub authority: Signer<'info>,
        // Solana 2022 Token Standard
        pub token_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct UpdateTokenDetails<'info> {
        #[account(mut)]
        pub state: Account<'info, State>,
        // Solana 2022 Token Standard
        pub token_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct MintTo<'info> {
        #[account(mut, mint::mint_authority = authority)]
        pub mint: Account<'info, Mint>,
        #[account(mut)]
        pub dest: Account<'info, TokenAccount>,
        pub authority: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct Burn<'info> {
        #[account(mut, mint::mint_authority = authority)]
        pub mint: Account<'info, Mint>,
        #[account(mut)]
        pub source: Account<'info, TokenAccount>,
        pub authority: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct ChangePausedState<'info> {
        pub authority: Signer<'info>,
        #[account(mut)]
        pub state: Account<'info, State>,
    }

    #[derive(Accounts)]
    pub struct TransferHook<'info> {
        pub source: AccountInfo<'info>,
        pub mint: AccountInfo<'info>,
        pub dest: AccountInfo<'info>,
        pub authority: AccountInfo<'info>,
        #[account(
            seeds = [b"extra-account-metas", mint.key().as_ref()],
            bump)
        ]
        pub extra_account: AccountInfo<'info>,
        pub state: Account<'info, State>,
    }

    #[derive(Accounts)]
    pub struct CollectFees<'info> {
        pub treasury_wallet: Account<'info, TokenAccount>,
        pub state: Account<'info, State>,
    }

    impl<'info> bark<'info> {
        pub fn initialize(ctx: Context<Initialize>, paused: bool) -> Result<()> {
            Self::initialize_state(&mut ctx.accounts.state, paused)?;
            Self::initialize_mint(ctx.accounts.payer.clone(), &ctx.accounts.state)?;

            Ok(())
        }

        // Function for initializing state
        fn initialize_state(state: &mut Account<'info, State>, paused: bool) -> Result<()> {
            state.owner = *ctx.accounts.authority.key;
            state.paused = paused;
            state.token_program_id = *ctx.accounts.token_program.key;
            state.version = PROGRAM_VERSION;

            Ok(())
        }

        // Function for initializing mint
        fn initialize_mint(payer: AccountInfo<'info>, state: &State) -> Result<()> {
            let mint_authority = state.owner.clone();
            let freeze_authority = None;
            let mint = create_mint(
                payer,
                mint_authority.key,
                freeze_authority,
                state.decimals,
            )?;
            Ok(())
        }

        pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
            let state = &ctx.accounts.state;
            if state.total_supply.checked_add(amount).unwrap() > state.max_supply {
                return Err(ProgramError::Custom(789).into());
            }
            let seeds = [b"mint".as_ref(), state.key().as_ref()];
            let signer_seeds = &[&seeds[..]];
            let cpi_accounts = mint_to(
                ctx.accounts.mint.clone(),
                ctx.accounts.dest.clone(),
                ctx.accounts.authority.clone(),
                signer_seeds,
                &[],
                amount,
            )?;
            state.total_supply = state.total_supply.checked_add(amount).unwrap();
            Ok(())
        }

        pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
            let state = &ctx.accounts.state;
            if amount == 0 || amount > state.total_supply {
                return Err(ProgramError::Custom(987).into());
            }
            let seeds = [b"burn".as_ref(), state.key().as_ref()];
            let signer_seeds = &[&seeds[..]];
            let cpi_accounts = burn(
                ctx.accounts.mint.clone(),
                ctx.accounts.source.clone(),
                ctx.accounts.authority.clone(),
                signer_seeds,
                &[],
                amount,
            )?;
            state.total_supply = state.total_supply.checked_sub(amount).unwrap();
            Ok(())
        }

        pub fn change_paused_state(ctx: Context<ChangePausedState>, paused: bool) -> Result<()> {
            if *ctx.accounts.authority.key != ctx.accounts.state.owner {
                return Err(ProgramError::Custom(123).into());
            }
            let state = &mut ctx.accounts.state;
            state.paused = paused;
            Ok(())
        }

        pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
            let state = &ctx.accounts.state;
            if state.paused {
                return Err(ProgramError::Custom(123).into());
            }
            Ok(())
        }

        pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
            let state = &ctx.accounts.state;
            let fee_amount = (FEE_PERCENTAGE / 100.0) * state.total_supply as f64;
            let fee_amount = fee_amount as u64;
            if fee_amount > 0 {
                let seeds = [b"fee".as_ref(), state.key().as_ref()];
                let signer_seeds = &[&seeds[..]];
                let cpi_accounts = mint_to(
                    ctx.accounts.treasury_wallet.clone(),
                    ctx.accounts.treasury_wallet.clone(),
                    ctx.accounts.treasury_wallet.clone(),
                    signer_seeds,
                    &[],
                    fee_amount,
                )?;
                state.total_supply = state.total_supply.checked_add(fee_amount).unwrap();
            }
            Ok(())
        }

        pub fn update_token_details(ctx: Context<UpdateTokenDetails>, description: String, image: String) -> Result<()> {
            let state = &mut ctx.accounts.state;
            state.description = description;
            state.image = image;
            Ok(())
        }
    }
}
