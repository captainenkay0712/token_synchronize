// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { TokenSynchronize } from "../target/types/token_synchronize";
// import {
//   Keypair,
//   SystemProgram,
//   PublicKey,
// } from "@solana/web3.js";
// import {
//   TOKEN_PROGRAM_ID,
//   TOKEN_2022_PROGRAM_ID,
//   createMint,
//   createAccount,
//   mintTo,
//   getAccount,
//   getOrCreateAssociatedTokenAccount,
//   getAccountLen,
//   ACCOUNT_SIZE,
//   MINT_SIZE,
//   getMintCloseAuthority,
//   getMintLen,
//   ExtensionType,
//   getMint,
//   createInitializeMint2Instruction,
//   createInitializeMintInstruction,
//   createInitializeTransferFeeConfigInstruction,
//   NATIVE_MINT,
//   getAssociatedTokenAddressSync,
//   createAssociatedTokenAccountInstruction,
//   freezeAccount,
//   calculateFee,
// } from "@solana/spl-token";
// import { assert } from "chai";

// describe("token-synchronize", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const wallet = provider.wallet as anchor.Wallet;
//   const connection = provider.connection;

//   const program = anchor.workspace.TokenSynchronize as Program<TokenSynchronize>;

//   const delegate = Keypair.generate();
//   const amount = new anchor.BN(100);
//   const decimals = 6;

//   async function create_and_mint_token(
//     tokenProgramId: PublicKey,
//     decimals: number,
//     authority?: PublicKey,
//     freezeAuthority?: PublicKey
//   ) {
//     const mint = await createMint(
//       connection,
//       wallet.payer,
//       wallet.publicKey,
//       freezeAuthority || null,
//       decimals,
//       undefined,
//       undefined,
//       tokenProgramId
//     );

//     const owner = authority || wallet.publicKey;
//     let tokenAccount: PublicKey;

//     if (authority && !PublicKey.isOnCurve(authority.toBuffer())) {
//         const tokenAccountKeypair = Keypair.generate();
//         tokenAccount = await createAccount(
//             connection,
//             wallet.payer,
//             mint,
//             owner,
//             tokenAccountKeypair,
//             undefined,
//             tokenProgramId
//         );
//     } else {
//         const ata = await getOrCreateAssociatedTokenAccount(
//             connection,
//             wallet.payer,
//             mint,
//             owner,
//             false,
//             undefined,
//             undefined,
//             tokenProgramId
//         );
//         tokenAccount = ata.address;
//     }

//     await mintTo(
//       connection,
//       wallet.payer,
//       mint,
//       tokenAccount,
//       wallet.payer,
//       Number(amount),
//       [],
//       undefined,
//       tokenProgramId
//     );

//     return { mint, tokenAccount };
//   }

//   // --- Tests ---
//   describe("pda_authority", () => {
//     it("Approves with PDA authority", async () => {
//       const seeds = [Buffer.from("pda-authority")];
//       const [pda, bump] = PublicKey.findProgramAddressSync(seeds, program.programId);
//       const bump_buf = Buffer.from([bump]);
//       const authority_seeds = [seeds[0], bump_buf];
  
//       const { tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6, pda);
  
//       await program.methods
//         .approve(amount, authority_seeds)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           to: tokenAccount,
//           delegate: delegate.publicKey,
//           authority: pda,
//         })
//         .rpc();
  
//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
//     });
//   });
  
//   describe("amount_to_ui_amount", () => {
//     it("Fails for non-Token-2022 programs", async () => {
//         const { tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);
//         try {
//             await program.methods
//             .amountToUiAmount(amount)
//             .accounts({
//                 tokenProgramId: TOKEN_PROGRAM_ID,
//                 account: tokenAccount,
//             })
//             .rpc();
//             assert.fail("Should have failed for non-Token-2022 program");
//         } catch (err) {
//           assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
//         }
//     });

//     it("Converts amount to UI amount for Token-2022", async () => {
//         const { mint } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);

//         const result = await program.methods
//         .amountToUiAmount(new anchor.BN(12345000000))
//         .accounts({
//             tokenProgramId: TOKEN_2022_PROGRAM_ID,
//             account: mint,
//         })
//         .view();

//         assert.strictEqual(result, "12345");
//     });
//   });

//   describe("approve", () => {
//     it("Approves for SPL Token", async () => {
//       const { tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);

//       await program.methods
//         .approve(amount, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           to: tokenAccount,
//           delegate: delegate.publicKey,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
//     });

//     it("Approves for Token-2022", async () => {
//       const { tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);

//       await program.methods
//         .approve(amount, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           to: tokenAccount,
//           delegate: delegate.publicKey,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
//     });
//   });

//   describe("approve_checked", () => {
//     it("Approves for SPL Token (Checked)", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, decimals);

//       await program.methods
//         .approveChecked(amount, decimals, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           to: tokenAccount,
//           mint: mint,
//           delegate: delegate.publicKey,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
//     });

//     it("Approves for Token-2022 (Checked)", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, decimals);

//       await program.methods
//         .approveChecked(amount, decimals, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           to: tokenAccount,
//           mint: mint,
//           delegate: delegate.publicKey,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());
//     });
//   });

//   describe("burn", () => {
//     const burnAmount = new anchor.BN(50);

//     it("Burns for SPL Token", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 9, wallet.publicKey);

//       const accountInfoBeforeBurn = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       const accountBefore = new anchor.BN(accountInfoBeforeBurn.amount);

//       await program.methods.approve(burnAmount, null).accounts({
//         tokenProgramId: TOKEN_PROGRAM_ID,
//         to: tokenAccount,
//         delegate: delegate.publicKey,
//         authority: wallet.publicKey,
//       }).rpc();

//       await program.methods.burn(burnAmount, null).accounts({
//         tokenProgramId: TOKEN_PROGRAM_ID,
//         mint: mint,
//         from: tokenAccount,
//         authority: delegate.publicKey,
//       }).signers([delegate]).rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.equal(accountInfo.amount.toString(), (accountBefore.sub(burnAmount)).toString());
//     });

//     it("Burns for Token-2022", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 0, wallet.publicKey);
      
//       const accountInfoBeforeBurn = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       const accountBefore = new anchor.BN(accountInfoBeforeBurn.amount);

//       await program.methods.approve(burnAmount, null).accounts({
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//         to: tokenAccount,
//         delegate: delegate.publicKey,
//         authority: wallet.publicKey,
//       }).rpc();

//       await program.methods.burn(burnAmount, null).accounts({
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//         mint: mint,
//         from: tokenAccount,
//         authority: delegate.publicKey,
//       }).signers([delegate]).rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.equal(accountInfo.amount.toString(), (accountBefore.sub(burnAmount)).toString());
//     });
//   });

//   describe("burn_checked", () => {
//     const burnAmount = new anchor.BN(50);

//     it("Fails for non-Token-2022 programs", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, decimals, wallet.publicKey);
//       await program.methods.approveChecked(amount, decimals, null).accounts({
//         tokenProgramId: TOKEN_PROGRAM_ID,
//         to: tokenAccount,
//         mint: mint,
//         delegate: delegate.publicKey,
//         authority: wallet.publicKey,
//       }).rpc();

//       try {
//         await program.methods.burnChecked(burnAmount, decimals, null).accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           mint: mint,
//           from: tokenAccount,
//           authority: delegate.publicKey,
//         }).signers([delegate]).rpc();

//         assert.fail("Should have failed for non-Token-2022 program");
//       } catch (err) {
//         assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
//       }
//     });

//     it("Burns for Token-2022 (Checked)", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, decimals, wallet.publicKey);

//       const accountInfoBeforeBurn = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       const accountBefore = new anchor.BN(accountInfoBeforeBurn.amount);

//       await program.methods.approveChecked(amount, decimals, null).accounts({
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//         to: tokenAccount,
//         mint: mint,
//         delegate: delegate.publicKey,
//         authority: wallet.publicKey,
//       }).rpc();

//       await program.methods.burnChecked(burnAmount, decimals, null).accounts({
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//         mint: mint,
//         from: tokenAccount,
//         authority: delegate.publicKey,
//       }).signers([delegate]).rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.equal(accountInfo.amount.toString(), (accountBefore.sub(burnAmount)).toString());
//     });
//   });

//   describe("close_account", () => {
//     it("Closes an SPL Token account", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 9);

//       await program.methods.burn(new anchor.BN(amount), null).accounts({
//         mint: mint,
//         from: tokenAccount,
//         authority: wallet.publicKey,
//         tokenProgramId: TOKEN_PROGRAM_ID,
//       }).rpc();

//       await program.methods.closeAccount(null).accounts({
//         account: tokenAccount,
//         destination: wallet.publicKey,
//         authority: wallet.publicKey,
//         tokenProgramId: TOKEN_PROGRAM_ID,
//       }).rpc();

//       const accountInfo = await connection.getAccountInfo(tokenAccount);
//       assert.isNull(accountInfo, "Account should be closed");
//     });

//     it("Closes a Token-2022 account", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);

//       await program.methods.burn(new anchor.BN(amount), null).accounts({
//         mint: mint,
//         from: tokenAccount,
//         authority: wallet.publicKey,
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//       }).rpc();

//       await program.methods.closeAccount(null).accounts({
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//         account: tokenAccount,
//         destination: wallet.publicKey,
//         authority: wallet.publicKey,
//       }).rpc();

//       const accountInfo = await connection.getAccountInfo(tokenAccount);
//       assert.isNull(accountInfo, "Account should be closed");
//     });
//   });

//   describe("freeze_account", () => {
//     it("Freezes an SPL Token account", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 9, null, wallet.publicKey);

//       await program.methods.freezeAccount(null).accounts({
//         account: tokenAccount,
//         mint: mint,
//         authority: wallet.publicKey,
//         tokenProgramId: TOKEN_PROGRAM_ID,
//       }).rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);

//       assert.isTrue(accountInfo.isFrozen, "true");
//     });

//     it("Freezes a Token-2022 account", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 0, null, wallet.publicKey);

//       await program.methods.freezeAccount(null).accounts({
//         account: tokenAccount,
//         mint: mint,
//         authority: wallet.publicKey,
//         tokenProgramId: TOKEN_2022_PROGRAM_ID,
//       }).rpc();

//       const accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.isFrozen, "true");
//     });
//   })

//   describe("initialize_account", () => {
//     it("Initializes a new SPL Token account", async () => {
//       const { mint } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);
//       const newAccount = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeAccount(null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           account: newAccount.publicKey,
//           mint: mint,
//           authority: wallet.publicKey,
//           rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//         })
//         .signers([newAccount])
//         .preInstructions([
//           anchor.web3.SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: newAccount.publicKey,
//             space: ACCOUNT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE),
//             programId: TOKEN_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const accountInfo = await getAccount(connection, newAccount.publicKey, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(accountInfo.mint.equals(mint));
//       assert.isTrue(accountInfo.owner.equals(wallet.publicKey));
//     });

//     it("Initializes a new Token-2022 account", async () => {
//       const { mint } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);
//       const newAccount = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeAccount(null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           account: newAccount.publicKey,
//           mint: mint,
//           authority: wallet.publicKey,
//           rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//         })
//         .signers([newAccount])
//         .preInstructions([
//           anchor.web3.SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: newAccount.publicKey,
//             space: ACCOUNT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE),
//             programId: TOKEN_2022_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const accountInfo = await getAccount(connection, newAccount.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.mint.equals(mint));
//       assert.isTrue(accountInfo.owner.equals(wallet.publicKey));
//     });
//   });

//   describe("initialize_account3", () => {
//     it("Initializes a new SPL Token account", async () => {
//       const { mint } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);
//       const newAccount = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeAccount3(null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           account: newAccount.publicKey,
//           mint: mint,
//           authority: wallet.publicKey,
//         })
//         .signers([newAccount])
//         .preInstructions([
//           anchor.web3.SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: newAccount.publicKey,
//             space: ACCOUNT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE),
//             programId: TOKEN_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const accountInfo = await getAccount(connection, newAccount.publicKey, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(accountInfo.mint.equals(mint));
//       assert.isTrue(accountInfo.owner.equals(wallet.publicKey));
//     });

//     it("Initializes a new Token-2022 account", async () => {
//       const { mint } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);
//       const newAccount = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeAccount3(null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           account: newAccount.publicKey,
//           mint: mint,
//           authority: wallet.publicKey,
//         })
//         .signers([newAccount])
//         .preInstructions([
//           anchor.web3.SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: newAccount.publicKey,
//             space: ACCOUNT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE),
//             programId: TOKEN_2022_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const accountInfo = await getAccount(connection, newAccount.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.mint.equals(mint));
//       assert.isTrue(accountInfo.owner.equals(wallet.publicKey));
//     });
//   });

//   describe("initialize_immutable_owner", () => {
//     it("Fails for non-Token-2022 programs", async () => {
//       const mint = await createMint(
//         connection,
//         wallet.payer,
//         wallet.publicKey,
//         null,
//         6,
//         undefined,
//         undefined,
//         TOKEN_PROGRAM_ID
//       );

//       const account = anchor.web3.Keypair.generate();
//       const extensions = [ExtensionType.ImmutableOwner];
//       const accountLen = getAccountLen(extensions);
//       const lamports = await connection.getMinimumBalanceForRentExemption(accountLen);

//       const transaction = new anchor.web3.Transaction().add(
//         SystemProgram.createAccount({
//           fromPubkey: wallet.publicKey,
//           newAccountPubkey: account.publicKey,
//           space: accountLen,
//           lamports,
//           programId: TOKEN_PROGRAM_ID,
//         })
//       );

//       await anchor.web3.sendAndConfirmTransaction(connection, transaction, [wallet.payer, account]);

//       try {
//         await program.methods
//         .initializeImmutableOwner(null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           account: account.publicKey,
//         })
//         .rpc();

//         assert.fail("Should have failed for non-Token-2022 program");
//       } catch (err) {
//         assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
//       }
//     });
  
//     it("Initializes immutable owner for a Token-2022 account", async () => {
//       const mint = await createMint(
//         connection,
//         wallet.payer,
//         wallet.publicKey,
//         null,
//         6,
//         undefined,
//         undefined,
//         TOKEN_2022_PROGRAM_ID
//       );

//       const account = anchor.web3.Keypair.generate();
//       const extensions = [ExtensionType.ImmutableOwner];
//       const accountLen = getAccountLen(extensions);
//       const lamports = await connection.getMinimumBalanceForRentExemption(accountLen);

//       const createMintIx = SystemProgram.createAccount({
//         fromPubkey: wallet.publicKey,
//         newAccountPubkey: account.publicKey,
//         space: accountLen,
//         lamports,
//         programId: TOKEN_2022_PROGRAM_ID,
//       })

//       await program.methods
//         .initializeImmutableOwner(null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           account: account.publicKey,
//         })
//         .signers([account])
//         .preInstructions([createMintIx])
//         .rpc();

//       await program.methods
//         .initializeAccount3(null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           account: account.publicKey,
//           mint: mint,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfo = await getAccount(connection, account.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.isInitialized);
//     });
//   });

//   describe("initialize_mint", () => {
//     it("Initializes a new SPL Token mint", async () => {
//       const mint = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeMint(decimals, wallet.publicKey, null, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           mint: mint.publicKey,
//           rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//         })
//         .signers([mint])
//         .preInstructions([
//           SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: mint.publicKey,
//             space: MINT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(MINT_SIZE),
//             programId: TOKEN_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const mintInfo = await getMint(connection, mint.publicKey, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(mintInfo.mintAuthority.equals(wallet.publicKey));
//       assert.isTrue(mintInfo.freezeAuthority === null);
//       assert.equal(mintInfo.decimals, decimals);
//     });

//     it("Initializes a new Token-2022 mint", async () => {
//       const mint = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeMint(decimals, wallet.publicKey, null, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           mint: mint.publicKey,
//           rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//         })
//         .signers([mint])
//         .preInstructions([
//           SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: mint.publicKey,
//             space: MINT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(MINT_SIZE),
//             programId: TOKEN_2022_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const mintInfo = await getMint(connection, mint.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(mintInfo.mintAuthority.equals(wallet.publicKey));
//       assert.isTrue(mintInfo.freezeAuthority === null);
//       assert.equal(mintInfo.decimals, decimals);
//     });
//   });

//   describe("initialize_mint2", () => {
//     it("Initializes a new SPL Token mint", async () => {
//       const mint = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeMint2(decimals, wallet.publicKey, null, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           mint: mint.publicKey,
//         })
//         .signers([mint])
//         .preInstructions([
//           SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: mint.publicKey,
//             space: MINT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(MINT_SIZE),
//             programId: TOKEN_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const mintInfo = await getMint(connection, mint.publicKey, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(mintInfo.mintAuthority.equals(wallet.publicKey));
//       assert.isTrue(mintInfo.freezeAuthority === null);
//       assert.equal(mintInfo.decimals, decimals);
//     });

//     it("Initializes a new Token-2022 mint", async () => {
//       const mint = anchor.web3.Keypair.generate();

//       await program.methods
//         .initializeMint2(decimals, wallet.publicKey, null, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           mint: mint.publicKey,
//         })
//         .signers([mint])
//         .preInstructions([
//           SystemProgram.createAccount({
//             fromPubkey: wallet.publicKey,
//             newAccountPubkey: mint.publicKey,
//             space: MINT_SIZE,
//             lamports: await connection.getMinimumBalanceForRentExemption(MINT_SIZE),
//             programId: TOKEN_2022_PROGRAM_ID,
//           }),
//         ])
//         .rpc();

//       const mintInfo = await getMint(connection, mint.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(mintInfo.mintAuthority.equals(wallet.publicKey));
//       assert.isTrue(mintInfo.freezeAuthority === null);
//       assert.equal(mintInfo.decimals, decimals);
//     });
//   });

//   describe("initialize_mint_close_authority", () => {
//     it("Fails for non-Token-2022 programs", async () => {
//       const mint = anchor.web3.Keypair.generate();
//       const closeAuthority = anchor.web3.Keypair.generate().publicKey;

//       try {
//         await program.methods
//           .initializeMintCloseAuthority(closeAuthority, null)
//           .accounts({
//             tokenProgramId: TOKEN_PROGRAM_ID,
//             mint: mint.publicKey,
//           })
//           .signers([mint])
//           .preInstructions([
//             SystemProgram.createAccount({
//               fromPubkey: wallet.publicKey,
//               newAccountPubkey: mint.publicKey,
//               space: MINT_SIZE,
//               lamports: await connection.getMinimumBalanceForRentExemption(MINT_SIZE),
//               programId: TOKEN_PROGRAM_ID,
//             }),
//           ])
//           .rpc();
//         assert.fail("Should have failed for non-Token-2022 program");
//       } catch (err) {
//         assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
//       }
//     });

//     it("Initializes mint close authority for a Token-2022 mint", async () => {
//       const mint = anchor.web3.Keypair.generate();
//       const closeAuthority = anchor.web3.Keypair.generate();

//       const extensions = [ExtensionType.MintCloseAuthority];
//       const mintLen = getMintLen(extensions);
//       const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

//       const createMintIx = SystemProgram.createAccount({
//         fromPubkey: wallet.publicKey,
//         newAccountPubkey: mint.publicKey,
//         space: mintLen,
//         lamports,
//         programId: TOKEN_2022_PROGRAM_ID,
//       });

//       await program.methods
//         .initializeMintCloseAuthority(closeAuthority.publicKey, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           mint: mint.publicKey,
//         })
//         .signers([mint])
//         .preInstructions([createMintIx])
//         .postInstructions([
//           createInitializeMint2Instruction(
//             mint.publicKey,
//             0,
//             wallet.publicKey,
//             null,
//             TOKEN_2022_PROGRAM_ID
//           ),
//         ])
//         .rpc();

//       const mintInfo = await getMint(connection, mint.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
//       const mintCloseAuthority = getMintCloseAuthority(mintInfo);
      
//       assert.isTrue(mintCloseAuthority.closeAuthority.equals(closeAuthority.publicKey));
//     });
//   });

//   describe("mint_to", () => {
//     const mintAmount = new anchor.BN(1000);

//     it("Mints to an SPL Token account", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, decimals);
//       const accountInfoBefore = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);

//       await program.methods
//         .mintTo(mintAmount, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           mint: mint,
//           to: tokenAccount,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfoAfter = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.equal(accountInfoAfter.amount.toString(), (new anchor.BN(accountInfoBefore.amount).add(mintAmount)).toString());
//     });

//     it("Mints to a Token-2022 account", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);
//       const accountInfoBefore = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);

//       await program.methods
//         .mintTo(mintAmount, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           mint: mint,
//           to: tokenAccount,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfoAfter = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.equal(accountInfoAfter.amount.toString(), (new anchor.BN(accountInfoBefore.amount).add(mintAmount)).toString());
//     });
//   });

//   describe("mint_to_checked", () => {
//     const mintAmount = new anchor.BN(1000);

//     it("Fails for non-Token-2022 programs", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, decimals);

//       try {
//         await program.methods
//           .mintToChecked(mintAmount, decimals, null)
//           .accounts({
//             tokenProgramId: TOKEN_PROGRAM_ID,
//             mint: mint,
//             to: tokenAccount,
//             authority: wallet.publicKey,
//           })
//           .rpc();
//         assert.fail("Should have failed for non-Token-2022 program");
//       } catch (err) {
//         assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
//       }
//     });

//     it("Mints to a Token-2022 account (Checked)", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, decimals);
//       const accountInfoBefore = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);

//       await program.methods
//         .mintToChecked(mintAmount, decimals, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           mint: mint,
//           to: tokenAccount,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const accountInfoAfter = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.equal(accountInfoAfter.amount.toString(), (new anchor.BN(accountInfoBefore.amount).add(mintAmount)).toString());
//     });
//   });

//   describe("revoke", () => {
//     it("Revokes delegate from an SPL Token account", async () => {
//       const { tokenAccount } = await create_and_mint_token(
//           TOKEN_PROGRAM_ID,
//           decimals
//       );
//       const delegate = anchor.web3.Keypair.generate();

//       await program.methods
//         .approve(amount, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           to: tokenAccount,
//           delegate: delegate.publicKey,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       let accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());

//       await program.methods
//         .revoke(null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           source: tokenAccount,
//           authority: wallet.publicKey,
//         })
//         .rpc();
  
//       accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       assert.equal(accountInfo.delegatedAmount, BigInt(0));
//       assert.equal(accountInfo.delegate, null);
//     });
  
//     it("Revokes delegate from a Token-2022 account", async () => {
//       const { tokenAccount } = await create_and_mint_token(
//           TOKEN_2022_PROGRAM_ID,
//           decimals
//       );
//       const delegate = anchor.web3.Keypair.generate();

//       await program.methods
//         .approve(amount, null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           to: tokenAccount,
//           delegate: delegate.publicKey,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       let accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(accountInfo.delegate.equals(delegate.publicKey));
//       assert.equal(accountInfo.delegatedAmount.toString(), amount.toString());

//       await program.methods
//         .revoke(null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           source: tokenAccount,
//           authority: wallet.publicKey,
//         })
//         .rpc();
  
//       accountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.equal(accountInfo.delegatedAmount, BigInt(0));
//       assert.equal(accountInfo.delegate, null);
//     });
//   });

//   describe("set_authority", () => {
//     it("Sets mint authority for an SPL Token mint", async () => {
//       const { mint } = await create_and_mint_token(
//           TOKEN_PROGRAM_ID,
//           decimals
//       );
//       const newAuthority = anchor.web3.Keypair.generate();

//       await program.methods
//         .setAuthority(0, newAuthority.publicKey, null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           currentAuthority: wallet.publicKey,
//           accountOrMint: mint,
//         })
//         .rpc();

//       const mintInfo = await getMint(connection, mint);
//       assert.deepEqual(mintInfo.mintAuthority, newAuthority.publicKey);
//     });

//     it("Sets mint authority for a Token-2022 mint", async () => {
//         const { mint } = await create_and_mint_token(
//             TOKEN_2022_PROGRAM_ID,
//             decimals
//         );

//         const newAuthority = anchor.web3.Keypair.generate();

//         await program.methods
//           .setAuthority(0, newAuthority.publicKey, null)
//           .accounts({
//             tokenProgramId: TOKEN_2022_PROGRAM_ID,
//             currentAuthority: wallet.publicKey,
//             accountOrMint: mint,
//           })
//           .rpc();

//         const mintInfo = await getMint(connection, mint, undefined, TOKEN_2022_PROGRAM_ID);
//         assert.deepEqual(mintInfo.mintAuthority, newAuthority.publicKey);
//     });

//     it("Sets owner for an SPL Token account", async () => {
//         const { tokenAccount } = await create_and_mint_token(
//             TOKEN_PROGRAM_ID,
//             decimals
//         );

//         const newOwner = anchor.web3.Keypair.generate();

//         await program.methods
//           .setAuthority(2, newOwner.publicKey, null)
//           .accounts({
//             tokenProgramId: TOKEN_PROGRAM_ID,
//             currentAuthority: wallet.publicKey,
//             accountOrMint: tokenAccount,
//           })
//           .rpc();

//         const tokenAccountInfo = await getAccount(connection, tokenAccount);
//         assert.deepEqual(tokenAccountInfo.owner, newOwner.publicKey);
//     });
//   });

//   describe("sync_native", () => {
//     it("Syncs native SOL for an SPL Token account", async () => {
//       const amount = 1 * anchor.web3.LAMPORTS_PER_SOL;
//       const ata = getAssociatedTokenAddressSync(NATIVE_MINT, wallet.publicKey, false, TOKEN_PROGRAM_ID);

//       const tx = new anchor.web3.Transaction().add(
//         createAssociatedTokenAccountInstruction(wallet.publicKey, ata, wallet.publicKey, NATIVE_MINT, TOKEN_PROGRAM_ID),
//         anchor.web3.SystemProgram.transfer({
//           fromPubkey: wallet.publicKey,
//           toPubkey: ata,
//           lamports: amount,
//         })
//       );
//       await anchor.web3.sendAndConfirmTransaction(connection, tx, [wallet.payer]);

//       let account = await getAccount(connection, ata, undefined, TOKEN_PROGRAM_ID);
//       assert.equal(Number(account.amount), 0);

//       await program.methods.syncNative(null).accounts({
//         tokenProgramId: TOKEN_PROGRAM_ID,
//         account: ata,
//       }).rpc();

//       account = await getAccount(connection, ata, undefined, TOKEN_PROGRAM_ID);
//       assert.equal(Number(account.amount), amount);
//     });
//   });

//   describe("thaw_account", () => {
//     it("Thaws a frozen SPL Token account", async () => {
//       const mint = await createMint(connection, wallet.payer, wallet.publicKey, wallet.publicKey, 2);
//       const tokenAccount = await createAccount(connection, wallet.payer, mint, wallet.publicKey);

//       await freezeAccount(connection, wallet.payer, tokenAccount, mint, wallet.publicKey);
//       let tokenAccountInfo = await getAccount(connection, tokenAccount);
//       assert.isTrue(tokenAccountInfo.isFrozen);

//       await program.methods
//         .thawAccount(null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           account: tokenAccount,
//           mint: mint,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       tokenAccountInfo = await getAccount(connection, tokenAccount);
//       assert.isFalse(tokenAccountInfo.isFrozen);
//     });

//     it("Thaws a frozen Token-2022 account", async () => {
//       const mint = await createMint(connection, wallet.payer, wallet.publicKey, wallet.publicKey, 2, undefined, undefined, TOKEN_2022_PROGRAM_ID);
//       const tokenAccount = await createAccount(connection, wallet.payer, mint, wallet.publicKey, undefined, undefined, TOKEN_2022_PROGRAM_ID);

//       await freezeAccount(connection, wallet.payer, tokenAccount, mint, wallet.publicKey, [], undefined, TOKEN_2022_PROGRAM_ID);
//       let tokenAccountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isTrue(tokenAccountInfo.isFrozen);

//       await program.methods
//         .thawAccount(null)
//         .accounts({
//           tokenProgramId: TOKEN_2022_PROGRAM_ID,
//           account: tokenAccount,
//           mint: mint,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       tokenAccountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//       assert.isFalse(tokenAccountInfo.isFrozen);
//     });
//   });

//   describe("transfer", () => {
//     const transferAmount = 100;

//     it("Transfers native SOL", async () => {
//       const transferAmount = anchor.web3.LAMPORTS_PER_SOL;
//       const toWallet = anchor.web3.Keypair.generate();

//       const fromBalanceBefore = await connection.getBalance(wallet.publicKey);
//       const toBalanceBefore = await connection.getBalance(toWallet.publicKey);

//       const tx = await program.methods
//         .transfer(new anchor.BN(transferAmount), null)
//         .accounts({
//           tokenProgramId: anchor.web3.SystemProgram.programId,
//           from: wallet.publicKey,
//           to: toWallet.publicKey,
//           authority: wallet.publicKey,
//         })
//         .transaction();

//       const signature = await anchor.web3.sendAndConfirmTransaction(connection, tx, [wallet.payer]);
//       const txDetails = await connection.getTransaction(signature, { commitment: "confirmed" });
//       const fee = txDetails.meta.fee;

//       const fromBalanceAfter = await connection.getBalance(wallet.publicKey);
//       const toBalanceAfter = await connection.getBalance(toWallet.publicKey);

//       assert.equal(fromBalanceAfter, fromBalanceBefore - transferAmount - fee);
//       assert.equal(toBalanceAfter, toBalanceBefore + transferAmount);
//     });

//     it("Transfers SPL Tokens", async () => {
//       const { mint, tokenAccount } = await create_and_mint_token(
//         TOKEN_PROGRAM_ID,
//         decimals,
//       );

//       const toWallet = anchor.web3.Keypair.generate();
//       const toAccount = await createAccount(connection, wallet.payer, mint, toWallet.publicKey, undefined, undefined, TOKEN_PROGRAM_ID);

//       await program.methods
//         .transfer(new anchor.BN(transferAmount), null)
//         .accounts({
//           tokenProgramId: TOKEN_PROGRAM_ID,
//           from: tokenAccount,
//           to: toAccount,
//           authority: wallet.publicKey,
//         })
//         .rpc();

//       const fromAccountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//       const toAccountInfo = await getAccount(connection, toAccount, undefined, TOKEN_PROGRAM_ID);

//       assert.equal(Number(fromAccountInfo.amount), Number(amount) - transferAmount);
//       assert.equal(Number(toAccountInfo.amount), transferAmount);
//     });

//     it("Fails to transfer Token-2022 tokens", async () => {
//       try {
//         const { mint, tokenAccount } = await create_and_mint_token(
//           TOKEN_2022_PROGRAM_ID,
//           decimals,
//         );

//         const toWallet = anchor.web3.Keypair.generate();
//         const toAccount = await createAccount(connection, wallet.payer, mint, toWallet.publicKey, undefined, undefined, TOKEN_2022_PROGRAM_ID);

//         await program.methods
//           .transfer(new anchor.BN(transferAmount), null)
//           .accounts({
//             tokenProgramId: TOKEN_2022_PROGRAM_ID,
//             from: tokenAccount,
//             to: toAccount,
//             authority: wallet.publicKey,
//           })
//           .rpc();

//         assert.fail("Should have failed because transfer is deprecated for Token-2022");
//       } catch (err) {
//         assert.equal(err.error.errorCode.code, "Token2022TransferDeprecated");
//       }
//     });
//   });

//   describe("transfer_checked", () => {
//     const transferAmount = 100;

//     it("Transfers SPL Tokens", async () => {
//         const { mint, tokenAccount } = await create_and_mint_token(
//             TOKEN_PROGRAM_ID,
//             decimals,
//         );

//         const toWallet = anchor.web3.Keypair.generate();
//         const toAccount = await createAccount(connection, wallet.payer, mint, toWallet.publicKey, undefined, undefined, TOKEN_PROGRAM_ID);

//         await program.methods
//             .transferChecked(new anchor.BN(amount), decimals, null, null)
//             .accounts({
//                 tokenProgramId: TOKEN_PROGRAM_ID,
//                 from: tokenAccount,
//                 mint: mint,
//                 to: toAccount,
//                 authority: wallet.publicKey,
//             })
//             .rpc();

//         const fromAccountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_PROGRAM_ID);
//         const toAccountInfo = await getAccount(connection, toAccount, undefined, TOKEN_PROGRAM_ID);

//         assert.equal(Number(fromAccountInfo.amount), Number(amount) - transferAmount);
//         assert.equal(Number(toAccountInfo.amount), transferAmount);
//     });

//     it("Transfers Token-2022 tokens", async () => {
//         const amount = 100;
//         const mintAmount = 1000;
//         const { mint, tokenAccount } = await create_and_mint_token(
//             TOKEN_2022_PROGRAM_ID,
//             decimals,
//         );

//         const toWallet = anchor.web3.Keypair.generate();
//         const toAccount = await createAccount(connection, wallet.payer, mint, toWallet.publicKey, undefined, undefined, TOKEN_2022_PROGRAM_ID);

//         await program.methods
//             .transferChecked(new anchor.BN(amount), decimals, null, null)
//             .accounts({
//                 tokenProgramId: TOKEN_2022_PROGRAM_ID,
//                 from: tokenAccount,
//                 mint: mint,
//                 to: toAccount,
//                 authority: wallet.publicKey,
//             })
//             .rpc();

//         const fromAccountInfo = await getAccount(connection, tokenAccount, undefined, TOKEN_2022_PROGRAM_ID);
//         const toAccountInfo = await getAccount(connection, toAccount, undefined, TOKEN_2022_PROGRAM_ID);

//         assert.equal(Number(fromAccountInfo.amount), Number(amount) - transferAmount);
//         assert.equal(Number(toAccountInfo.amount), transferAmount);
//     });

//     it("Transfers Token-2022 tokens with fee", async () => {
//         const mint = anchor.web3.Keypair.generate();
//         const transferFee = {
//           epoch: BigInt(0),
//           maximumFee: BigInt(1000),
//           transferFeeBasisPoints: 100,
//         };

//         const mintLen = getMintLen([ExtensionType.TransferFeeConfig]);
//         const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

//         const transaction = new anchor.web3.Transaction().add(
//             anchor.web3.SystemProgram.createAccount({
//                 fromPubkey: wallet.publicKey,
//                 newAccountPubkey: mint.publicKey,
//                 space: mintLen,
//                 lamports,
//                 programId: TOKEN_2022_PROGRAM_ID,
//             }),
//             createInitializeTransferFeeConfigInstruction(
//                 mint.publicKey,
//                 wallet.publicKey,
//                 wallet.publicKey,
//                 transferFee.transferFeeBasisPoints,
//                 transferFee.maximumFee,
//                 TOKEN_2022_PROGRAM_ID
//             ),
//             createInitializeMintInstruction(mint.publicKey, decimals, wallet.publicKey, null, TOKEN_2022_PROGRAM_ID)
//         );
//         await anchor.web3.sendAndConfirmTransaction(connection, transaction, [wallet.payer, mint]);

//         const fromAccount = await createAccount(connection, wallet.payer, mint.publicKey, wallet.publicKey, undefined, undefined, TOKEN_2022_PROGRAM_ID);
//         const toWallet = anchor.web3.Keypair.generate();
//         const toAccount = await createAccount(connection, wallet.payer, mint.publicKey, toWallet.publicKey, undefined, undefined, TOKEN_2022_PROGRAM_ID);
    
//         await mintTo(connection, wallet.payer, mint.publicKey, fromAccount, wallet.publicKey, Number(amount), [], undefined, TOKEN_2022_PROGRAM_ID);

//         const fee = calculateFee(transferFee, BigInt(transferAmount));

//         await program.methods
//             .transferChecked(new anchor.BN(transferAmount), decimals, new anchor.BN(fee.toString()), null)
//             .accounts({
//                 tokenProgramId: TOKEN_2022_PROGRAM_ID,
//                 from: fromAccount,
//                 mint: mint.publicKey,
//                 to: toAccount,
//                 authority: wallet.publicKey,
//             })
//             .rpc();

//         const fromAccountInfo = await getAccount(connection, fromAccount, undefined, TOKEN_2022_PROGRAM_ID);
//         const toAccountInfo = await getAccount(connection, toAccount, undefined, TOKEN_2022_PROGRAM_ID);

//         assert.equal(Number(fromAccountInfo.amount), Number(amount) - transferAmount);
//         assert.equal(Number(toAccountInfo.amount), transferAmount - Number(fee));
//     });
//   });

//   describe("ui_amount_to_amount", () => {
//     it("Fails for non-Token-2022 programs", async () => {
//         const { tokenAccount } = await create_and_mint_token(TOKEN_PROGRAM_ID, 6);
//         try {
//             await program.methods
//             .uiAmountToAmount(amount.toString())
//             .accounts({
//                 tokenProgramId: TOKEN_PROGRAM_ID,
//                 account: tokenAccount,
//             })
//             .rpc();
//             assert.fail("Should have failed for non-Token-2022 program");
//         } catch (err) {
//           assert.equal(err.error.errorCode.code, "InvalidTokenProgram");
//         }
//     });

//     it("Converts UI amount to amount for Token-2022", async () => {
//         const { mint } = await create_and_mint_token(TOKEN_2022_PROGRAM_ID, 6);

//         const result = await program.methods
//         .uiAmountToAmount("12345")
//         .accounts({
//             tokenProgramId: TOKEN_2022_PROGRAM_ID,
//             account: mint,
//         })
//         .view();

//         assert.strictEqual(result.toNumber(), 12345000000);
//     });
//   });
// });
