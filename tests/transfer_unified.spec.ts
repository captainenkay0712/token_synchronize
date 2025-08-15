import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenSynchronize } from "../target/types/token_synchronize";
import {
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createMint,
  createInitializeMintInstruction,
  createInitializeTransferFeeConfigInstruction,
  createInitializeTransferHookInstruction,
  ExtensionType,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  getMintLen,
  getAccount,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { CustomTransferHook } from "../target/types/custom_transfer_hook";

describe("transfer_unified", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const wallet = provider.wallet as anchor.Wallet;
    const connection = provider.connection;

    const program = anchor.workspace
        .TokenSynchronize as Program<TokenSynchronize>;

    const customTransferHookProgram = anchor.workspace
        .CustomTransferHook as Program<CustomTransferHook>;

    const decimals = 6;
    const amount = new anchor.BN(500_000);
    
    const transferFee = {
        epoch: BigInt(0),
        maximumFee: BigInt(1000),
        transferFeeBasisPoints: 100,
    };

    const setupMint = async (tokenProgram: PublicKey) => {
        const mint = await createMint(
            connection,
            wallet.payer,
            wallet.publicKey,
            null,
            decimals,
            undefined,
            undefined,
            tokenProgram
        );
        return mint;
    };

  const setupAccounts = async (mint: PublicKey, tokenProgram: PublicKey) => {
    const fromAta = await getOrCreateAssociatedTokenAccount(
        connection,
        wallet.payer,
        mint,
        wallet.publicKey,
        undefined,
        undefined,
        undefined,
        tokenProgram
      );
  
      const toWallet = anchor.web3.Keypair.generate();
      const toAta = await getOrCreateAssociatedTokenAccount(
        connection,
        wallet.payer,
        mint,
        toWallet.publicKey,
        undefined,
        undefined,
        undefined,
        tokenProgram
      );

    return { fromAta: fromAta.address, toAta: toAta.address };
  };

  it("transfers tokens (SPL Token)", async () => {
    const mint = await setupMint(TOKEN_PROGRAM_ID);
    const { fromAta, toAta } = await setupAccounts(mint, TOKEN_PROGRAM_ID);

    await mintTo(
      connection,
      wallet.payer,
      mint,
      fromAta,
      wallet.publicKey,
      1_000_000,
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    const beforeFrom = await getAccount(connection, fromAta, undefined, TOKEN_PROGRAM_ID);
    const beforeTo = await getAccount(connection, toAta, undefined, TOKEN_PROGRAM_ID);

    const sig = await program.methods
      .transferUnified(amount)
      .accounts({
        tokenProgramId: TOKEN_PROGRAM_ID,
        from: fromAta,
        mint,
        to: toAta,
        authority: wallet.publicKey,
      })
      .remainingAccounts([])
      .rpc();

    const tx = await connection.getTransaction(sig, {
        commitment: "confirmed",
        maxSupportedTransactionVersion: 0,
    });

    // console.log(tx?.meta?.logMessages?.join("\n"));

    const afterFrom = await getAccount(connection, fromAta, undefined, TOKEN_PROGRAM_ID);
    const afterTo = await getAccount(connection, toAta, undefined, TOKEN_PROGRAM_ID);

    assert.equal(Number(beforeFrom.amount - afterFrom.amount), amount.toNumber());
    assert.equal(Number(afterTo.amount - beforeTo.amount), amount.toNumber());
  });

  it("transfers tokens (Token-2022)", async () => {
    const mint = await setupMint(TOKEN_2022_PROGRAM_ID);
    const { fromAta, toAta } = await setupAccounts(mint, TOKEN_2022_PROGRAM_ID);

    await mintTo(
      connection,
      wallet.payer,
      mint,
      fromAta,
      wallet.publicKey,
      1_000_000,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const beforeFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const beforeTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    const sig = await program.methods
      .transferUnified(amount)
      .accounts({
        tokenProgramId: TOKEN_2022_PROGRAM_ID,
        from: fromAta,
        mint,
        to: toAta,
        authority: wallet.publicKey,
      })
      .remainingAccounts([])
      .rpc();

    const tx = await connection.getTransaction(sig, {
        commitment: "confirmed",
        maxSupportedTransactionVersion: 0,
    });

    // console.log(tx?.meta?.logMessages?.join("\n"));

    const afterFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const afterTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    assert.equal(Number(beforeFrom.amount - afterFrom.amount), amount.toNumber());
    assert.equal(Number(afterTo.amount - beforeTo.amount), amount.toNumber());
  });

  it("transfers tokens (Token-2022 with transfer fee only)", async () => {
    const mintKeypair = anchor.web3.Keypair.generate();

    const mintLen = getMintLen([ExtensionType.TransferFeeConfig]);
    const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

    const txInit = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKeypair.publicKey,
        space: mintLen,
        lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferFeeConfigInstruction(
        mintKeypair.publicKey,
        wallet.publicKey,
        wallet.publicKey, 
        transferFee.transferFeeBasisPoints,
        transferFee.maximumFee,
        TOKEN_2022_PROGRAM_ID
      ),
      createInitializeMintInstruction(
        mintKeypair.publicKey,
        decimals,
        wallet.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID
      )
    );
    await anchor.web3.sendAndConfirmTransaction(connection, txInit, [wallet.payer, mintKeypair]);

    const { fromAta, toAta } = await setupAccounts(mintKeypair.publicKey, TOKEN_2022_PROGRAM_ID);

    await mintTo(
      connection,
      wallet.payer,
      mintKeypair.publicKey,
      fromAta,
      wallet.publicKey,
      1_000_000,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const beforeFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const beforeTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    const sig = await program.methods
      .transferUnified(amount)
      .accounts({
        tokenProgramId: TOKEN_2022_PROGRAM_ID,
        from: fromAta,
        mint: mintKeypair.publicKey,
        to: toAta,
        authority: wallet.publicKey,
      })
      .remainingAccounts([])
      .rpc();

    const tx = await connection.getTransaction(sig, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });
    // console.log(tx?.meta?.logMessages?.join("\n"));

    const afterFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const afterTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    const expectedFee = Math.min(
      Math.floor(amount.toNumber() * transferFee.transferFeeBasisPoints / 10_000),
      Number(transferFee.maximumFee)
    );

    assert.equal(Number(beforeFrom.amount - afterFrom.amount), amount.toNumber());
    assert.equal(Number(afterTo.amount - beforeTo.amount), amount.toNumber() - expectedFee);
  });

  it("transfers tokens (Token-2022 with transfer hook only)", async () => {
    const mintKeypair = anchor.web3.Keypair.generate();

    const mintLen = getMintLen([ExtensionType.TransferHook]);
    const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

    const [extraAccountMetaList] = PublicKey.findProgramAddressSync(
        [Buffer.from("extra-account-metas"), mintKeypair.publicKey.toBuffer()],
        customTransferHookProgram.programId
      );

    const txInit = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: mintKeypair.publicKey,
            space: mintLen,
            lamports,
            programId: TOKEN_2022_PROGRAM_ID,
          }),
          createInitializeTransferHookInstruction(
            mintKeypair.publicKey,
            wallet.publicKey,
            customTransferHookProgram.programId,
            TOKEN_2022_PROGRAM_ID
          ),
          createInitializeMintInstruction(
            mintKeypair.publicKey,
            decimals,
            wallet.publicKey,
            null,
            TOKEN_2022_PROGRAM_ID
          )
    );
    await anchor.web3.sendAndConfirmTransaction(connection, txInit, [wallet.payer, mintKeypair]);

    const { fromAta, toAta } = await setupAccounts(mintKeypair.publicKey, TOKEN_2022_PROGRAM_ID);

    await customTransferHookProgram.methods
        .initializeExtraAccountMetaList()
        .accounts({
            payer: wallet.publicKey,
            extraAccountMetaList: extraAccountMetaList,
            mint: mintKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

    await mintTo(
      connection,
      wallet.payer,
      mintKeypair.publicKey,
      fromAta,
      wallet.publicKey,
      1_000_000,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const beforeFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const beforeTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    const sig = await program.methods
      .transferUnified(amount)
      .accounts({
        tokenProgramId: TOKEN_2022_PROGRAM_ID,
        from: fromAta,
        mint: mintKeypair.publicKey,
        to: toAta,
        authority: wallet.publicKey,
      })
      .remainingAccounts([
        { pubkey: customTransferHookProgram.programId, isWritable: false, isSigner: false },
        { pubkey: extraAccountMetaList, isWritable: false, isSigner: false },
        { pubkey: wallet.publicKey, isWritable: false, isSigner: false },
        { pubkey: mintKeypair.publicKey, isWritable: false, isSigner: false },
      ])
      .rpc();

    const tx = await connection.getTransaction(sig, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });

    const logs = tx?.meta?.logMessages?.join("\n");
    // console.log(logs);

    const afterFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const afterTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    assert.equal(Number(beforeFrom.amount - afterFrom.amount), amount.toNumber());
    assert.equal(Number(afterTo.amount - beforeTo.amount), amount.toNumber());
    // assert.include(logs, "Custom transfer hook invoked!");
  });

  it("transfers tokens (Token-2022 with transfer hook and transfer fee)", async () => {
    const mintKeypair = anchor.web3.Keypair.generate();

    const mintLen = getMintLen([ExtensionType.TransferHook, ExtensionType.TransferFeeConfig]);
    const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

    const [extraAccountMetaList] = PublicKey.findProgramAddressSync(
        [Buffer.from("extra-account-metas"), mintKeypair.publicKey.toBuffer()],
        customTransferHookProgram.programId
      );

    const txInit = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: mintKeypair.publicKey,
            space: mintLen,
            lamports,
            programId: TOKEN_2022_PROGRAM_ID,
          }),
          createInitializeTransferHookInstruction(
            mintKeypair.publicKey,
            wallet.publicKey,
            customTransferHookProgram.programId,
            TOKEN_2022_PROGRAM_ID
          ),
          createInitializeTransferFeeConfigInstruction(
            mintKeypair.publicKey,
            wallet.publicKey,
            wallet.publicKey, 
            transferFee.transferFeeBasisPoints,
            transferFee.maximumFee,
            TOKEN_2022_PROGRAM_ID
          ),
          createInitializeMintInstruction(
            mintKeypair.publicKey,
            decimals,
            wallet.publicKey,
            null,
            TOKEN_2022_PROGRAM_ID
          )
    );
    await anchor.web3.sendAndConfirmTransaction(connection, txInit, [wallet.payer, mintKeypair]);

    const { fromAta, toAta } = await setupAccounts(mintKeypair.publicKey, TOKEN_2022_PROGRAM_ID);

    await customTransferHookProgram.methods
        .initializeExtraAccountMetaList()
        .accounts({
            payer: wallet.publicKey,
            extraAccountMetaList: extraAccountMetaList,
            mint: mintKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

    await mintTo(
      connection,
      wallet.payer,
      mintKeypair.publicKey,
      fromAta,
      wallet.publicKey,
      1_000_000,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const beforeFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const beforeTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    const sig = await program.methods
      .transferUnified(amount)
      .accounts({
        tokenProgramId: TOKEN_2022_PROGRAM_ID,
        from: fromAta,
        mint: mintKeypair.publicKey,
        to: toAta,
        authority: wallet.publicKey,
      })
      .remainingAccounts([
        { pubkey: customTransferHookProgram.programId, isWritable: false, isSigner: false },
        { pubkey: extraAccountMetaList, isWritable: false, isSigner: false },
        { pubkey: wallet.publicKey, isWritable: false, isSigner: false },
        { pubkey: mintKeypair.publicKey, isWritable: false, isSigner: false },
      ])
      .rpc();

    const tx = await connection.getTransaction(sig, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });

    const logs = tx?.meta?.logMessages?.join("\n");
    console.log(logs);

    const afterFrom = await getAccount(connection, fromAta, undefined, TOKEN_2022_PROGRAM_ID);
    const afterTo = await getAccount(connection, toAta, undefined, TOKEN_2022_PROGRAM_ID);

    const expectedFee = Math.min(
        Math.floor(amount.toNumber() * transferFee.transferFeeBasisPoints / 10_000),
        Number(transferFee.maximumFee)
    );

    assert.equal(Number(beforeFrom.amount - afterFrom.amount), amount.toNumber());
    assert.equal(Number(afterTo.amount - beforeTo.amount), amount.toNumber() - expectedFee);
  });
});
