import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

const DEVNET_AIRDROP_RPC = "https://api.devnet.solana.com";
const DEFAULT_MIN_BALANCE_SOL = 2;
const DEFAULT_MAX_AIRDROP_ATTEMPTS = 4;
const DEFAULT_AIRDROP_CHUNK_SOL = 2;

export interface EnsureDevnetWalletFundedOptions {
  minBalanceSol?: number;
  maxAirdropAttempts?: number;
  airdropChunkSol?: number;
  logger?: (line: string) => void;
}

export function formatSol(lamports: number): string {
  return (lamports / LAMPORTS_PER_SOL).toFixed(2);
}

function toLamports(sol: number): number {
  return Math.ceil(sol * LAMPORTS_PER_SOL);
}

async function sleep(ms: number): Promise<void> {
  await new Promise(resolve => setTimeout(resolve, ms));
}

export async function ensureDevnetWalletFunded(
  conn: Connection,
  payer: Keypair,
  options: EnsureDevnetWalletFundedOptions = {},
): Promise<number> {
  const logger = options.logger ?? (() => undefined);
  const minBalanceSol = options.minBalanceSol ?? DEFAULT_MIN_BALANCE_SOL;
  const maxAirdropAttempts = options.maxAirdropAttempts ?? DEFAULT_MAX_AIRDROP_ATTEMPTS;
  const airdropChunkSol = options.airdropChunkSol ?? DEFAULT_AIRDROP_CHUNK_SOL;

  if (!Number.isFinite(minBalanceSol) || minBalanceSol <= 0) {
    throw new Error(`Invalid minBalanceSol: ${minBalanceSol}`);
  }

  const minLamports = toLamports(minBalanceSol);
  let balance = await conn.getBalance(payer.publicKey, "confirmed");

  if (balance >= minLamports) return balance;

  logger(
    `Low wallet balance (${formatSol(balance)} SOL). ` +
      `Attempting devnet airdrop to reach at least ${minBalanceSol.toFixed(2)} SOL...`,
  );

  for (let attempt = 1; attempt <= maxAirdropAttempts && balance < minLamports; attempt++) {
    const remainingLamports = minLamports - balance;
    const requestSol = Math.max(1, Math.min(airdropChunkSol, Math.ceil(remainingLamports / LAMPORTS_PER_SOL)));
    const requestLamports = toLamports(requestSol);

    try {
      const signature = await conn.requestAirdrop(payer.publicKey, requestLamports);
      await conn.confirmTransaction(signature, "confirmed");
      logger(`Airdrop attempt ${attempt}/${maxAirdropAttempts}: requested ${requestSol} SOL.`);
    } catch (error) {
      logger(
        `Airdrop attempt ${attempt}/${maxAirdropAttempts} failed: ${
          error instanceof Error ? error.message : String(error)
        }`,
      );
    }

    await sleep(1500);
    balance = await conn.getBalance(payer.publicKey, "confirmed");
  }

  if (balance < minLamports) {
    const shortfall = minLamports - balance;
    throw new Error(
      `Insufficient devnet balance for A/B rehearsal. ` +
        `Have ${formatSol(balance)} SOL, need at least ${minBalanceSol.toFixed(2)} SOL ` +
        `(short by ${formatSol(shortfall)} SOL). ` +
        `Fund ${payer.publicKey.toBase58()} manually or retry later when devnet airdrop is available (${DEVNET_AIRDROP_RPC}). ` +
        `You can also try https://faucet.solana.com for alternate test SOL sources.`,
    );
  }

  logger(`Wallet funded: ${formatSol(balance)} SOL`);
  return balance;
}
