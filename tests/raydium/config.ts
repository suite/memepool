import * as anchor from "@coral-xyz/anchor";
import { Raydium, TxVersion, parseTokenAccountResp } from '@raydium-io/raydium-sdk-v2';
import { Connection, Keypair, PublicKey, clusterApiUrl } from '@solana/web3.js';

export const owner: Keypair = Keypair.fromSecretKey(
  Uint8Array.from(
    JSON.parse(
      require('fs').readFileSync(
        require('os').homedir() + '/.config/solana/id.json',
        'utf-8'
      )
    )
  )
);
export const txVersion = TxVersion.V0; // or TxVersion.LEGACY

const cluster = 'devnet'; // 'mainnet' | 'devnet'
export const connection = new Connection(clusterApiUrl(cluster));

let raydium: Raydium | undefined;
export const initSdk = async (params?: { loadToken?: boolean }) => {
  if (raydium) return raydium;

  raydium = await Raydium.load({
    owner,
    connection,
    cluster,
    disableFeatureCheck: true,
    disableLoadToken: !params?.loadToken,
    blockhashCommitment: 'finalized',
  });

  return raydium;
}

const POOL_SEED = Buffer.from(anchor.utils.bytes.utf8.encode("pool"));
const POOL_AUTH_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault_and_lp_mint_auth_seed")
);

export const POOL_LPMINT_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("pool_lp_mint")
);

export const ORACLE_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("observation")
);

export function getPoolAddress(
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

export function getAuthAddress(
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [POOL_AUTH_SEED],
    programId
  );
  return [address, bump];
}

export function getPoolLpMintAddress(
  pool: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [POOL_LPMINT_SEED, pool.toBuffer()],
    programId
  );
  return [address, bump];
}

export function getOrcleAccountAddress(
  pool: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [ORACLE_SEED, pool.toBuffer()],
    programId
  );
  return [address, bump];
}

// This is our programs vault pool to keep track of pool positions
export const getVaultPoolAccount = (poolState: PublicKey, programId: PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync([
            Buffer.from("vault_pool"), poolState.toBuffer()], 
            programId)[0];
}
