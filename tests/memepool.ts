import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Memepool } from "../target/types/memepool";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("memepool", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Memepool as Program<Memepool>;

  const payer = provider.wallet as NodeWallet;

  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault")], program.programId)[0];
  const memeMint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("meme")], program.programId)[0];

  it("Initializes Vault", async () => {
    // Add your test here.
    const tx = await program.methods.initializeVault()
    .accountsPartial({
      vault,
      admin: provider.wallet.publicKey,
      memeMint,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();

    console.log("Vault initialized.")
    console.log("Your transaction signature", tx);
  });
});
