use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};
use solana_program::program::invoke_signed;
use std::cmp::Ordering;

pub fn sol_transfer_with_signer<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    signers: &[&[&[u8]]; 1],
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke_signed(&ix, &[source, destination, system_program], signers)?;
    Ok(())
}

pub fn sol_transfer_user<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke(&ix, &[source, destination, system_program])?;
    Ok(())
}

pub fn puffed_out_string(s: &String, size: usize) -> String {
    let mut array_of_zeroes = vec![];

    let puff_amount = size - s.len();
    while array_of_zeroes.len() < puff_amount {
        array_of_zeroes.push(0u8);
    }
    s.clone() + std::str::from_utf8(&array_of_zeroes).unwrap()
}

pub struct HashStruct {
    pub nonce: u64,
    pub initial_seed: u64,
}

pub fn resize_account<'info>(
    account_info: AccountInfo<'info>,
    new_space: usize,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_space);
    let current_balance = account_info.lamports();

    match new_minimum_balance.cmp(&current_balance) {
        Ordering::Greater => {
            let lamports_diff = new_minimum_balance.saturating_sub(current_balance);
            invoke(
                &transfer(&payer.key(), &account_info.key(), lamports_diff),
                &[payer.clone(), account_info.clone(), system_program.clone()],
            )?;
        }
        Ordering::Less => {
            let lamports_diff = current_balance.saturating_sub(new_minimum_balance);
            **account_info.try_borrow_mut_lamports()? = new_minimum_balance;
            **payer.try_borrow_mut_lamports()? = payer
                .lamports()
                .checked_add(lamports_diff)
                .expect("Add error");
        }
        Ordering::Equal => {}
    }
    account_info.realloc(new_space, false)?;
    Ok(())
}
