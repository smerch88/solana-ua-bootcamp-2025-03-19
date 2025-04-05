import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";

const keypair = Keypair.generate();

console.log(`The public key is: `, keypair.publicKey.toBase58());

console.log(`The secret key is (Uint8Array): `, keypair.secretKey);

const base58Key = bs58.encode(keypair.secretKey);
console.log(`The secret key in base58 format: `, base58Key);

console.log(`✅ Finished!`);
