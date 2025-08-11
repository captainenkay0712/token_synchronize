import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenSynchronize } from "../target/types/token_synchronize";
import {
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { assert } from "chai";

describe("token-synchronize", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const program = anchor.workspace.TokenSynchronize as Program<TokenSynchronize>;

  const delegate = Keypair.generate();
  const amount = new anchor.BN(100);

  async function create_and_mint_token(
    tokenProgramId: PublicKey,
    decimals: number
  ) {
    const mint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      decimals,
      undefined,
      undefined,
      tokenProgramId
    );

    const tokenAccount = await createAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey,
      undefined,
      undefined,
      tokenProgramId
    );

    await mintTo(
      connection,
      wallet.payer,
      mint,
      tokenAccount,
      wallet.payer,
      1000 * 10 ** decimals,
      [],
      undefined,
      tokenProgramId
    );

    return { mint, tokenAccount };
  }

  // --- Tests ---
  describe("approve", () => {
    it("Approves for SPL Token", async () => {
      const { tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);

      await program.methods
        .approve(amount, null)
        .accounts({
          tokenProgramId: TOKEN_PROGRAM_ID,
          to: tokenAccount,
          delegate: delegate.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();

      const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
      assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
      assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
    });

    it("Approves for Token-2022", async () => {
      const { tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);

      await program.methods
        .approve(amount, null)
        .accounts({
          tokenProgramId: TOKEN_2022_PROGRAM_ID,
          to: tokenAccount,
          delegate: delegate.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();

      const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
      assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
      assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
    });
  });

  describe("approve_checked", () => {
    const decimals = 9;

    it("Approves for SPL Token (Checked)", async () => {
      const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, decimals);

      await program.methods
        .approveChecked(amount, decimals, null)
        .accounts({
          tokenProgramId: TOKEN_PROGRAM_ID,
          to: tokenAccount,
          mint: mint,
          delegate: delegate.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();

      const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
      assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
      assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
    });

    it("Approves for Token-2022 (Checked)", async () => {
      const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, decimals);

      await program.methods
        .approveChecked(amount, decimals, null)
        .accounts({
          tokenProgramId: TOKEN_2022_PROGRAM_ID,
          to: tokenAccount,
          mint: mint,
          delegate: delegate.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();

      const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
      assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
      assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
    });
  });

  describe("amount_to_ui_amount", () => {
    it("Fails for non-Token-2022 programs", async () => {
        const { tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);
        try {
            await program.methods
            .amountToUiAmount(amount)
            .accounts({
                tokenProgramId: TOKEN_PROGRAM_ID,
                account: tokenAccount,
            })
            .rpc();
            assert.fail("Should have failed for non-Token-2022 program");
        } catch (err) {
          assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
        }
    });

    it("Converts amount to UI amount for Token-2022", async () => {
        const { mint, tokenAccount  } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);

        const result = await program.methods
        .amountToUiAmount(new anchor.BN(12345000000))
        .accounts({
            tokenProgramId: TOKEN_2022_PROGRAM_ID,
            account: mint,
        })
        .view();

        assert.strictEqual(result, "12345");
    });
  });
});
