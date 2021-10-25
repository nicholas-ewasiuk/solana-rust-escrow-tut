use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

use crate::processor::Processor;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction(
    program_id: &Pubkey, // Public key of the account the program was loaded into
    accounts: &[AccountInfo], // The accounts for initializing escrow
    instruction_data: &[u8], // instruction to use and expected amount of token to receive
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}

