import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Memepool } from "../target/types/memepool";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import * as fs from "fs";
import { PublicKey } from "@solana/web3.js";
import { initSdk, txVersion } from "./raydium/config";
import { DEVNET_PROGRAM_ID, getCpmmPdaAmmConfigId } from "@raydium-io/raydium-sdk-v2";

// https://github.com/raydium-io/raydium-cp-swap/blob/cfdb70a8ca9ea62bb5c304d4492ac0fc371ae8ce/tests/utils/pda.ts#L6C8-L6C78
const POOL_SEED = Buffer.from(anchor.utils.bytes.utf8.encode("pool"));
// https://github.com/raydium-io/raydium-cp-swap/blob/master/tests/utils/pda.ts#L77-L93
function getPoolAddress(
  ammConfig: PublicKey,
  tokenMint0: PublicKey,
  tokenMint1: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [
      POOL_SEED,
      ammConfig.toBuffer(),
      tokenMint0.toBuffer(),
      tokenMint1.toBuffer(),
    ],
    programId
  );
  return [address, bump];
}

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
  
  console.log("Vault pda:", vault.toString());
  console.log("$MEME mint:", memeMint.toString());
  console.log("Secondary wallet pubkey:", secondaryKp.publicKey.toString());

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

    // const poolState = await program.account.poolState.fetch(poolAddress);

    const tx = await program.methods.depositLp()
      .accountsPartial({
        aggregator: secondaryKp.publicKey,
        poolState: poolAddress
      })
      .signers([secondaryKp])
      .rpc();
      
      console.log("Called DepositLp.");
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
