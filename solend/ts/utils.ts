import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import * as fs from "fs";

export const getCreatorKeypair = () => {
    const pk = Uint8Array.from(
      JSON.parse(fs.readFileSync(`/Users/coredev0927/.config/solana/id.json`) as unknown as string)
    );
    const keypair = Keypair.fromSecretKey(pk);
    return keypair;
}

export const getPublicKey = (name: string) =>
  new PublicKey(
    JSON.parse(fs.readFileSync(`../keys/${name}_pub.js`) as unknown as string)
  );

export const writePublicKey = (publicKey: PublicKey, name: string) => {
    fs.writeFileSync(
      `../keys/${name}_pub.js`,
      JSON.stringify(publicKey.toString())
    );
};
export const writePublicKeys = (publicKeys: string, name: string) => {
    fs.writeFileSync(
      `../keys/${name}_pubs.js`,
      publicKeys
    );
};

export const convert_to_wei = (val) => (parseFloat(val) * 1e9).toString();

export const getATAPublicKey = async (tokenMint: PublicKey, owner: PublicKey) => {
  return await getAssociatedTokenAddress(
    tokenMint,
    owner,
    true,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  )
}