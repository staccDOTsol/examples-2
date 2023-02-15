use {
    crate::*,
    anchor_lang::{prelude::Pubkey, InstructionData},
    anchor_spl::token,
    clockwork_sdk::client::{Client, ClientResult},
    clockwork_sdk::thread_program,
    solana_sdk::instruction::{AccountMeta, Instruction},
};

pub fn subscribe(
    client: &Client,
    subscriber: Pubkey,
    subscription: Pubkey,
    subscriber_token_account: Pubkey,
    subscription_bank: Pubkey,
    mint: Pubkey,
    subscription_thread: Pubkey,
) -> ClientResult<()> {
    let subscribe_ix = Instruction {
        program_id: subscriptions_program::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(subscriber, false),
            AccountMeta::new(subscriber_token_account, false),
            AccountMeta::new(subscription, false),
            AccountMeta::new(subscription_bank, false),
            AccountMeta::new(subscription_thread, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new_readonly(thread_program::ID, false),
        ],
        data: subscriptions_program::instruction::Subscribe {}.data(),
    };

    send_and_confirm_tx(
        client,
        [subscribe_ix].to_vec(),
        None,
        "subscribe".to_string(),
    )?;
    Ok(())
}
