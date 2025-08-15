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
    -   `transfer_unified`: Transfer tokens across SPL Token and Token-2022, including support for Transfer Fee and Transfer Hook extensions.
-   **Built with Anchor**: Leverages the Anchor framework for rapid development and security.

## Instructions

### `transfer_unified`
Unified transfer that works with SPL Token and Token-2022. It automatically:
- Uses `transfer_checked` for SPL Token and Token-2022.
- Applies Transfer Fee if the mint has `TransferFeeConfig`.
- Executes Transfer Hook if the mint has `TransferHook` (requires extra accounts provided via `remaining_accounts`).

**Accounts:** (`programs/token_synchronize/src/context.rs::TransferCheckedContext`)
- `token_program_id`: SPL Token or Token-2022 program ID.
- `from`: Source token account.
- `mint`: Mint account.
- `to`: Destination token account.
- `authority`: Owner or delegated/PDA authority for `from`.

**Signer & extra accounts rules:**
- Single-owner: `signer_accounts` must be empty and `authority.is_signer = true`.
- Multisig: provide all `signer_accounts` (each must be a signer). Do not include `authority` in `signer_accounts`.
- No overlap between `signer_accounts` and `extra_accounts`.
- When mint has Transfer Hook: `extra_accounts[0]` must be the hook program ID (executable). The rest must match the hook’s required accounts order. If order/content is wrong, the hook or token program will fail the transaction.

See tests in `tests/transfer_unified.spec.ts` covering:
- SPL Token
- Token-2022 (no extensions)
- Token-2022 + Transfer Fee
- Token-2022 + Transfer Hook
- Token-2022 + Transfer Fee + Transfer Hook

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

## Notes on Transfer Hook extra accounts

- If you do not know the exact extra accounts required by a given hook, pass them via `remaining_accounts` in the correct order as defined by that hook’s `ExtraAccountMetaList`. If the order is wrong, the hook will revert with a clear error in logs.
- The program performs basic validations (authority/signers, non-overlap of signer vs extra accounts, hook program at `extra_accounts[0]`).
- For strict pre-validation, consider resolving accounts on-chain using the hook’s meta list account (if provided) via `spl_tlv_account_resolution`.

