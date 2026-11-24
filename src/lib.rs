use anchor_lang::prelude::*;

declare_id!("BktHP1Lm4PfQfMrj8HApbreMMDENNympxqMNy6jDNZkJ");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(ctx: Context<Hello>, data: Vec<u8>) -> Result<()> {
        require!(
            ctx.remaining_accounts.is_empty(),
            HelloError::AccountsNotAllowed
        );
        require!(data.is_empty(), HelloError::DataNotAllowed);

        msg!("Hello Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}

#[error_code]
pub enum HelloError {
    #[msg("The hello instruction does not accept accounts")]
    AccountsNotAllowed,
    #[msg("The hello instruction does not accept data")]
    DataNotAllowed,
}
