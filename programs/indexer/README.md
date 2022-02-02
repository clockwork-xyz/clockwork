# List Program

The **List Program** creates lists of Solana account addresses. Developers can use these lists to index account data relevant to their programs without relying on an off-chain indexing solution.

## Get Started

```yaml
# Cargo.toml

[dependencies]
index-program = { version = "0.0.2" }
```

Mainnet: `HPVqgVHeD9NPrJFSCd2UnpBudMYvorXYkzAqmy5naHTr`

Devnet: `HPVqgVHeD9NPrJFSCd2UnpBudMYvorXYkzAqmy5naHTr`

## Instructions

#### [`create_list`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/instructions/create_list.rs)

- Initializes a new [`List`](https://github.com/faktorfi/faktor/blob/main/programs/list/src/state/list.rs) account.
- Errors if:
  - The list already exists for the given owner and namespace.

#### [`delete_list`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/instructions/delete_list.rs)

- Closes an [`List`](https://github.com/faktorfi/faktor/blob/main/programs/list/src/state/list.rs) account.
- Returns rent to owner.

#### [`push_element`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/instructions/push_element.rs)

- Initializes a new [`Element`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/state/element.rs) account.
- Appends the element to the index's data structure.
- Errors if:
  - The list is already at max capacity.

#### [`pop_element`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/instructions/pop_element.rs)

- Closes an [`Element`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/state/element.rs) account.
- Removes the element from the index's data structure.
- Returns rent to owner.

## State

#### [`List`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/state/list.rs)

- Metadata for managing a list of addresses.
- PDA is a hash of the owner's address and a custom namespace address.

#### [`Element`](https://github.com/faktorfi/workspace/blob/main/programs/list/src/state/element.rs)

- An address value with a position in a list.
- PDA is a hash of the list's address and the pointer's position.

## Examples

These examples are for Solana programs that need to create and manage their own on-chain lists. These examples show an Anchor program that has a singleton "authority account" for signing instructions on behalf of the program.

### Create a list

Here is an example instruction `create_my_list` that creates an list owned by the program.

```rs
// create_my_list.rs

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateMyList<'info> {
    #[account(
        mut,
        seeds = [SEED_AUTHORITY],
        bump = authority.bump,
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,

    #[account(mut)]
    pub list: AccountInfo<'info>,

    #[account(address = list_program::ID)]
    pub list_program: Program<'info, list_program::program::ListProgram>,

    #[account(
        init,
        payer = signer,
        space = 8 + size_of<Namespace>()
    )]
    pub namespace: Account<'info, Namespace>

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMyIndex>, bump: u8) -> ProgramResult {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let list = &ctx.accounts.list;
    let list_program = &ctx.accounts.list_program;
    let namespace = &ctx.accounts.namespace;
    let signer = &ctx.accounts.signer;
    let system_program = &ctx.accounts.system_program;

    // Create a list owned by the program authority.
    list_program::cpi::create_index(
        CpiContext::new_with_signer(
            list_program.to_account_info(),
            list_program::cpi::accounts::CreateList {
                index: index.to_account_info(),
                namespace: namespace.to_account_info(),
                owner: authority.to_account_info(),
                payer: signer.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority.bump]]],
        ),
        bump,
    )
}
```

### Push an element

Here is an example instruction `push_my_element` that adds an element to a list owned by the program.

```rs
// create_my_pointer.rs

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(reference: Pubkey, bump: u8)]
pub struct CreateMyPointer<'info> {
    #[account(
        mut,
        seeds = [SEED_AUTHORITY],
        bump = authority.bump,
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,

    #[account()]
    pub element: AccountInfo<'info>,

    #[account(
      mut,
      constraint = list.owner == authority.key(),
      constraint = list.namespace == namespace.key(),
      owner = list_program.key()
    )]
    pub list: AccountInfo<'info>,

    #[account(address = list_program::ID)]
    pub list_program: Program<'info, list_program::program::IndexProgram>,

    #[account()]
    pub namespace: Account<'info, Namespace>

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<CreateMyPointer>,
  reference: Pubkey,
  bump: u8,
) -> ProgramResult {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let element = &ctx.accounts.element;
    let list = &ctx.accounts.list;
    let list_program = &ctx.accounts.list_program;
    let signer = &ctx.accounts.signer;
    let system_program = &ctx.accounts.system_program;

    // Add the reference address into the list.
    list_program::cpi::push_element(
        CpiContext::new_with_signer(
            list_program.to_account_info(),
            list_program::cpi::accounts::PushElement {
                element: element.to_account_info(),
                list: list.to_account_info(),
                owner: authority.to_account_info(),
                payer: signer.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority.bump]]],
        ),
        reference,
        bump,
    )
}
```
