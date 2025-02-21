import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';
import { Memepool } from "../../target/types/memepool";

export const getPortfolioAccount = (user: PublicKey, programId: PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync([
            Buffer.from("portfolio"), user.toBuffer()], 
            programId)[0];
}

export const getPortfolioCounter = async (portfolio: PublicKey, program: anchor.Program<Memepool>) => {
    let counter: BN = new BN(0);
    try {
      const portfolioAccount = await program.account.portfolio.fetch(portfolio);
      counter = portfolioAccount.counter;
    } catch (err) {}

    return counter;
}

export const getWithdrawRequestAccount = (user: PublicKey, counter: BN, programId: PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync([
          Buffer.from("withdraw_request"), user.toBuffer(), counter.toArrayLike(Buffer, "le", 8)], 
          programId)[0];
}