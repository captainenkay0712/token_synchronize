# Solana Token Synchronize Program

A flexible Solana program built with Anchor that provides a unified interface for handling token operations (`approve`, `burn`, etc.) across both the SPL Token and Token-2022 standards.

This program simplifies client-side logic by allowing developers to interact with a single program ID regardless of the underlying token standard being used.

## Features

-   **Unified Interface**: Single set of instructions for both SPL Token and Token-2022.
-   **Supported Instructions**:
    -   `approve`: Delegate spending authority.
    -   `approve_checked`: Delegate spending authority with a check on token decimals.
    -   `burn`: Burn tokens from an account with delegated authority.
    -   `amount_to_ui_amount`: A view function to convert a raw token amount to a UI-friendly string (Token-2022 only).
-   **Built with Anchor**: Leverages the Anchor framework for rapid development and security.

## Instructions

### `approve`
Delegates authority to another account to spend a specified amount of tokens.

**Accounts:**
- `token_program_id`: The program ID of the token standard (`Token` or `Token-2022`).
- `to`: The token account to delegate authority from.
- `delegate`: The account to delegate authority to.
- `authority`: The owner of the `to` account.

### `approve_checked`
Similar to `approve`, but includes a check for the token's decimal precision.

**Accounts:**
- `token_program_id`: The program ID of the token standard (`Token` or `Token-2022`).
- `to`: The token account to delegate authority from.
- `mint`: The mint account for the token.
- `delegate`: The account to delegate authority to.
- `authority`: The owner of the `to` account.

### `burn`
Burns a specified amount of tokens from an account that has delegated authority to the caller.

**Accounts:**
- `token_program_id`: The program ID of the token standard (`Token` or `Token-2022`).
- `mint`: The mint account for the token.
- `from`: The token account to burn tokens from.
- `authority`: The delegated authority for the `from` account.

### `amount_to_ui_amount` (View Function)
Converts a raw token amount (u64) into a UI-displayable string, respecting the token's decimals. This is a read-only view function that uses a CPI to the Token-2022 program.

**Accounts:**
- `token_program_id`: Must be the `Token-2022` program ID.
- `account`: The mint account to read the balance from.

## Getting Started

### Prerequisites

-   Rust
-   Solana Tool Suite
-   Anchor Framework (`avm install latest`, `avm use latest`)
-   Node.js & Yarn

### Build

To build the program, run:

```sh
anchor build
```

### Test

To run the test suite, which covers all instructions for both token standards, run:

```sh
anchor test
```
