import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import fs from "fs";

import { Keypair, Connection, PublicKey } from "@solana/web3.js";

import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import { JackpotSmartContract } from "../target/types/jackpot_smart_contract";
import {
    createConfigTx,
    createGameTx,
    setWinnerTx,
    claimRewardTx,
    joinGameTx
} from "../lib/scripts";
import { execTx } from "../lib/util";
import {
    SEED_CONFIG,
    TEST_INITIAL_MAX_JOINER_COUNT,
    TEST_INITIAL_PLATFORM_FEE,
    TEST_INITIAL_MIN_DEPOSIT_AMOUNT,
    GAME_GROUND,
} from "../lib/constant";


// Global state for CLI operations
let solConnection: Connection | null = null;
let program: Program<JackpotSmartContract> | null = null;
let payer: NodeWallet | null = null;
let provider: anchor.Provider | null = null;
let feePayer: NodeWallet | null = null;
let feePayerWalletKeypair: Keypair | null = null;
let teamWallet: PublicKey | null = null;
let programId: string | undefined;

/**
 * Set cluster, provider, program
 * If rpc != null use rpc, otherwise use cluster param
 * @param cluster - cluster ex. mainnet-beta, devnet ...
 * @param keypair - wallet keypair
 * @param rpc - rpc
 */
export const setClusterConfig = async (
    cluster: web3.Cluster,
    keypair: string,
    rpc?: string
) => {
    if (!rpc) {
        solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
    } else {
        solConnection = new web3.Connection(rpc);
    }

    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(keypair, "utf-8"))),
        { skipValidation: true }
    );
    payer = new NodeWallet(walletKeypair);

    feePayerWalletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync("../key/uu.json", "utf-8"))),
        { skipValidation: true }
    );
    feePayer = new NodeWallet(feePayerWalletKeypair);

    teamWallet = new PublicKey("EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv");

    console.log("Wallet Address: ", payer.publicKey.toBase58());

    anchor.setProvider(
        new anchor.AnchorProvider(solConnection, payer, {
            skipPreflight: true,
            commitment: "confirmed",
        })
    );

    provider = anchor.getProvider();

    // Generate the program client from IDL.
    program = anchor.workspace.JackpotSmartContract as Program<JackpotSmartContract>;
    programId = program.programId.toBase58();
    console.log("ProgramId: ", program.programId.toBase58());
};

export const configProject = async () => {
    if (!program || !payer || !solConnection || !teamWallet) {
        throw new Error("Cluster configuration not initialized. Run setClusterConfig first.");
    }

    console.log("Configuring project...");
    const authority = new PublicKey("H7YMxhKgLw2NDM9WQnpcUefPvCaLJCCYYaq1ETLHXJuH");
    const payerWallet = new PublicKey("H7YMxhKgLw2NDM9WQnpcUefPvCaLJCCYYaq1ETLHXJuH");

    const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    console.log("Config PDA:", configPda.toBase58());

    let configAccount;
    try {
        configAccount = await program.account.config.fetch(configPda);
        console.log("Existing config:", configAccount);
    } catch (error) {
        console.log("Config account not found, will initialize new config");
        configAccount = { gameRound: new BN(0) };
    }

    const newConfig = {
        authority,
        payerWallet,
        teamWallet,
        gameRound: configAccount.gameRound || new BN(0),
        platformFee: new BN(TEST_INITIAL_PLATFORM_FEE), // 1% platform fee (100 basis points)
        minDepositAmount: new BN(TEST_INITIAL_MIN_DEPOSIT_AMOUNT), // 0.1 SOL minimum
        maxJoinerCount: new BN(TEST_INITIAL_MAX_JOINER_COUNT), // 100 max joiners
        initialized: false,
    };

    const tx = await createConfigTx(
        payer.publicKey,
        newConfig,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
    console.log("Project configuration completed successfully");
};

export const createGame = async (
    roundTime: number,
    minDepositAmount: number,
    maxJoinerCount: number
) => {
    if (!program || !payer || !solConnection || !feePayerWalletKeypair) {
        throw new Error("Cluster configuration not initialized. Run setClusterConfig first.");
    }

    const configPda = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    )[0];
    
    const configAccount = await program.account.config.fetch(configPda);
    console.log("Current game round:", configAccount.gameRound.toString());

    const tx = await createGameTx(
        payer.publicKey,
        feePayerWalletKeypair,
        roundTime,
        minDepositAmount,
        maxJoinerCount,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
    console.log("Game created successfully");
};

export const setWinner = async (roundNum: number) => {
    if (!program || !payer || !solConnection) {
        throw new Error("Cluster configuration not initialized. Run setClusterConfig first.");
    }

    const tx = await setWinnerTx(
        payer.publicKey,
        roundNum,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);

    const [gameGroundPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(GAME_GROUND), new BN(roundNum).toArrayLike(Buffer, "le", 8)],
        program.programId
    );
    console.log("Game Ground PDA:", gameGroundPda.toBase58());

    const gameGroundAccount = await program.account.gameGround.fetch(gameGroundPda);
    console.log("Winner:", gameGroundAccount.winner.toBase58());
    console.log("Total deposit:", gameGroundAccount.totalDeposit.toString());
    console.log("Random number:", gameGroundAccount.rand.toString());
};

export const claimReward = async (roundNum: number) => {
    if (!program || !payer || !solConnection || !feePayerWalletKeypair) {
        throw new Error("Cluster configuration not initialized. Run setClusterConfig first.");
    }

    const tx = await claimRewardTx(
        payer.publicKey,
        feePayerWalletKeypair,
        roundNum,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
    console.log("Reward claimed successfully for round", roundNum);
};

export const joinGame = async (roundNum: number, amount: number) => {
    if (!program || !payer || !solConnection || !feePayerWalletKeypair || !teamWallet) {
        throw new Error("Cluster configuration not initialized. Run setClusterConfig first.");
    }

    const tx = await joinGameTx(
        payer.publicKey,
        feePayerWalletKeypair,
        teamWallet,
        roundNum,
        amount,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
    console.log(`Successfully joined game round ${roundNum} with ${amount} lamports`);
}; 