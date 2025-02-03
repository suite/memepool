import { Raydium, TxVersion, parseTokenAccountResp } from '@raydium-io/raydium-sdk-v2';
import { Connection, Keypair, clusterApiUrl } from '@solana/web3.js';
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