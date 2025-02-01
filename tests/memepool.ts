import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Memepool } from "../target/types/memepool";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("memepool", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Memepool as Program<Memepool>;

  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault")], program.programId)[0];
  const memeMint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("meme")], program.programId)[0];
  
  console.log("Vault pda:", vault.toString());
  console.log("$MEME mint:", memeMint.toString());

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
});
