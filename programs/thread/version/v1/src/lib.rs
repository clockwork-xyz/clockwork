pub mod utils;

use anchor_lang::solana_program::entrypoint::ProgramResult;

anchor_gen::generate_cpi_interface!(
    idl_path = "idl.json",
    zero_copy(TickArray, Tick),
    packed(TickArray, Tick)
);

anchor_lang::prelude::declare_id!("3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv");
