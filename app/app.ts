import { BubblegumCommonCollection } from "../target/types/bubblegum_common_collection";
import idl from "../target/idl/bubblegum_common_collection.json";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import {
  AnchorProvider,
  Program,
  setProvider,
  Wallet,
} from "@coral-xyz/anchor";
import {
  createSignerFromKeypair,
  keypairIdentity,
  publicKey,
  Umi,
} from "@metaplex-foundation/umi";
import {
  getMerkleTreeSize,
  MPL_BUBBLEGUM_PROGRAM_ID,
  mplBubblegum,
} from "@metaplex-foundation/mpl-bubblegum";
import {
  SPL_ACCOUNT_COMPRESSION_ADDRESS,
  SPL_NOOP_ADDRESS,
} from "@solana/spl-account-compression";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createAccount } from "@metaplex-foundation/mpl-toolbox";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import "rpc-websockets/dist/lib/client";

const connection = new Connection("http://localhost:8899", "confirmed");
const walletKeypair = Keypair.generate();
const merkleTreeKeypair = Keypair.generate();

const provider = new AnchorProvider(connection, new Wallet(walletKeypair));
setProvider(provider);

const program = new Program(idl as BubblegumCommonCollection, provider);

async function main() {
  const airdropTx = await connection.requestAirdrop(
    walletKeypair.publicKey,
    500_000_000_000
  );
  await connection.confirmTransaction(airdropTx);
  console.log("airdropTx:", airdropTx);

  /**
   * @description create tree IX args
   */
  const createTreeConfigInstructionArgsWrapper = {
    maxDepth: 3,
    maxBufferSize: 8,
    public: null,
  };

  await createCollectionTreeWithUMI(
    walletKeypair,
    merkleTreeKeypair,
    createTreeConfigInstructionArgsWrapper.maxDepth,
    createTreeConfigInstructionArgsWrapper.maxBufferSize
  );

  const [treeConfigPDA] = PublicKey.findProgramAddressSync(
    [merkleTreeKeypair.publicKey.toBuffer()],
    new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID)
  );

  await program.methods.initializeCollectionAuthority().accounts({}).rpc();

  const tx = await program.methods
    .createCollectionTree(createTreeConfigInstructionArgsWrapper)
    .accounts({
      payer: walletKeypair.publicKey,
      bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
      treeConfig: treeConfigPDA,
      merkleTree: merkleTreeKeypair.publicKey,
      logWrapper: new PublicKey(SPL_NOOP_ADDRESS),
      compressionProgram: new PublicKey(SPL_ACCOUNT_COMPRESSION_ADDRESS),
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .signers([walletKeypair])
    .rpc();
  console.log("Create tree TX:", tx);

  // TODO!
  await sendAndConfirmTxWithClientSideReplication();
}

main().then(() => console.log("done!"));

const createCollectionTreeWithUMI = async (
  walletKeypair: Keypair,
  merkleTreeKeypair: Keypair,
  maxDepth: number,
  maxBufferSize: number
) => {
  const umi = createUmi("http://127.0.0.1:8899", "confirmed").use(
    mplBubblegum()
  );
  const walletSigner = keyPairToUMISigner(walletKeypair, umi);
  const merkleTreeSigner = keyPairToUMISigner(merkleTreeKeypair, umi);
  umi.use(keypairIdentity(walletSigner));

  const space = getMerkleTreeSize(maxDepth, maxBufferSize);

  const builder = await createAccount(umi, {
    payer: walletSigner,
    newAccount: merkleTreeSigner,
    lamports: await umi.rpc.getRent(space),
    programId: publicKey(SPL_ACCOUNT_COMPRESSION_ADDRESS),
    space: space,
  });
  await builder.sendAndConfirm(umi);
};

const keyPairToUMISigner = (keypair: Keypair, umi: Umi) => {
  return createSignerFromKeypair(umi, {
    publicKey: publicKey(keypair.publicKey.toString()),
    secretKey: keypair.secretKey,
  });
};

/**
 * @description Basically what we have to do here is set "confirmed" as RPC state for Connection
 * and preflights, set the repeat sends to 0 and instead re-send TX from client until it is
 * confirmed or until `blockheight` is no longer valid (more than 150 slots outdated).
 * As it is instructed in https://solana.com/docs/advanced/retry#when-to-re-sign-transactions
 */
const sendAndConfirmTxWithClientSideReplication = async () => {};
