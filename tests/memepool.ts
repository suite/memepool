import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Memepool } from "../target/types/memepool";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import * as fs from "fs";
import { PublicKey } from "@solana/web3.js";
import { getAuthAddress, getPoolLpMintAddress, initSdk, txVersion } from "./raydium/config";
import { DEVNET_PROGRAM_ID, getCpmmPdaAmmConfigId } from "@raydium-io/raydium-sdk-v2";


describe("memepool", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Memepool as Program<Memepool>;

  // Load the secondary keypair
  const secondaryKp = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("target/deploy/aggregator-keypair.json").toString()))
  );
  
  // Create a wallet for the secondary keypair that can be used to sign transactions
  const secondaryWallet = new anchor.Wallet(secondaryKp);
  
  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault")], program.programId)[0];
  const memeMint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("meme")], program.programId)[0];
  
  const cpSwapProgram = new PublicKey("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW");

  console.log("Vault pda:", vault.toString());
  console.log("$MEME mint:", memeMint.toString());
  console.log("Secondary wallet pubkey:", secondaryKp.publicKey.toString());
  console.log("Raydium CP Swap Program:", cpSwapProgram.toString());

  it("Initializes Vault", async () => {
      const tx = await program.methods.initializeVault()
        .accountsPartial({
          vault,
          admin: provider.wallet.publicKey,
          memeMint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        }).rpc();

    console.log("Vault initialized.");
    console.log("Your transaction signature", tx);
  });

  it("Deposits SOL into vault", async () => {
    const deposit = new BN(250_000_000); // 0.25 SOL
    const depositerMemeAta = getAssociatedTokenAddressSync(memeMint, provider.wallet.publicKey);
    
    const tx = await program.methods.depositVault(deposit)
      .accountsPartial({
        depositer: provider.wallet.publicKey,
        vault,
        memeMint,
        depositerMemeAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).rpc();

    console.log("Deposited SOL.");
    console.log("Your transaction signature", tx);
  });

  it("Withdraws SOL using MEME", async () => {
    const withdraw = new BN(250_000_000); // 0.25 $MEME
    const withdrawerMemeAta = getAssociatedTokenAddressSync(memeMint, provider.wallet.publicKey);
    
    const tx = await program.methods.withdrawVault(withdraw)
      .accountsPartial({
        withdrawer: provider.wallet.publicKey,
        vault,
        memeMint,
        withdrawerMemeAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).rpc();

      console.log("Withdrew SOL.");
      console.log("Your transaction signature", tx);
  });

  it("Deposits into LP", async() => {
    // SOL/MEMEPOOLTEST lp
    const poolAddress = new PublicKey("2zQi1M8QrJpXxLWNyBuec3N7hNG1x7DmChctYYeE5HLT");
    const raydium = await initSdk({ loadToken: true });
    const poolInfo = await raydium.cpmm.getRpcPoolInfo(poolAddress.toString());

    console.log("Pool info", poolInfo);

    const lpTokenAmount = new BN(10);
    const maximumToken0Amount = new BN(10);
    const maximumToken1Amount = new BN(10);

    const [authority] = getAuthAddress(cpSwapProgram); // CONSTANT FOR ALL POOLS

    const [lpMintAddress] = getPoolLpMintAddress(
      poolAddress,
      cpSwapProgram
    );
    
    const [ownerLpToken] = PublicKey.findProgramAddressSync(
      [
        vault.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        lpMintAddress.toBuffer(),
      ],
      ASSOCIATED_PROGRAM_ID
    );

    const ownerToken0 = getAssociatedTokenAddressSync(
      poolInfo.mintA,
      vault,
      true,
      poolInfo.mintProgramA
    );

    const ownerToken1 = getAssociatedTokenAddressSync(
      poolInfo.mintB,
      vault,
      true,
      poolInfo.mintProgramB
    );

    const tx = await program.methods.depositLp(lpTokenAmount, maximumToken0Amount, maximumToken1Amount)
      .accountsPartial({
        vault,
        aggregator: secondaryKp.publicKey,
        cpSwapProgram,
        authority,
        poolState: poolAddress,
        ownerLpToken,
        token0Account: ownerToken0,
        token1Account: ownerToken1,
        token0Vault: poolInfo.vaultA,
        token1Vault: poolInfo.vaultB,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenProgram2022: TOKEN_2022_PROGRAM_ID,
        vault0Mint: poolInfo.mintA,
        vault1Mint: poolInfo.mintB,
        lpMint: poolInfo.mintLp,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([secondaryKp])
      .rpc();
      
      console.log("Called DepositLp. new");
      console.log("Your transaction signature", tx);
  });


  // create a pool using raydium-sdk-v2 (for testing on devnet)
  it("creates pool using raydium sdk", async () => {
    const raydium = await initSdk({ loadToken: true });

    // SOL
    const mintA = await raydium.token.getTokenInfo("So11111111111111111111111111111111111111112");
    // MEMEPOOLTEST
    const mintB = await raydium.token.getTokenInfo('DcPRHwtoWCtzt8WwtD7VdMHvMLtHya7WPknH6kmUsUbw');
    const feeConfigs = await raydium.api.getCpmmConfigs();

    if (raydium.cluster === 'devnet') {
      feeConfigs.forEach((config) => {
        config.id = getCpmmPdaAmmConfigId(DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM, config.index).publicKey.toBase58()
      });
    }

    const { execute, extInfo } = await raydium.cpmm.createPool({
      // poolId: // your custom publicKey, default sdk will automatically calculate pda pool id
      programId: DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM,
      poolFeeAccount: DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_FEE_ACC,
      mintA,
      mintB,
      mintAAmount: new BN(100),
      mintBAmount: new BN(100),
      startTime: new BN(0),
      feeConfig: feeConfigs[0],
      associatedOnly: false,
      ownerInfo: {
        useSOLBalance: true,
      },
      txVersion,
      // optional: set up priority fee here
      // computeBudgetConfig: {
      //   units: 600000,
      //   microLamports: 46591500,
      // },
    });

    const { txId } = await execute({ sendAndConfirm: true })
    console.log('pool created', {
      txId,
      poolKeys: Object.keys(extInfo.address).reduce(
        (acc, cur) => ({
          ...acc,
          [cur]: extInfo.address[cur as keyof typeof extInfo.address].toString(),
        }),
        {}
      ),
    });
  });
});
