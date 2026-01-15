import {
    Transaction,
    Connection,
} from "@solana/web3.js";

import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

/**
 * Execute a transaction with proper error handling and confirmation
 * @param transaction - The transaction to execute
 * @param connection - Solana connection
 * @param payer - Wallet to sign the transaction
 * @param commitment - Transaction commitment level
 */
export const execTx = async (
    transaction: Transaction,
    connection: Connection,
    payer: NodeWallet,
    commitment: "confirmed" | "finalized" = 'confirmed'
): Promise<string> => {
    try {
        // Sign the transaction with payer wallet
        const signedTx = await payer.signTransaction(transaction);

        // Simulate transaction to check for errors
        const simulation = await connection.simulateTransaction(signedTx);
        if (simulation.value.err) {
            throw new Error(`Transaction simulation failed: ${JSON.stringify(simulation.value.err)}`);
        }
        console.log("Transaction simulation successful. Estimated fee:", simulation.value.fee);

        // Serialize and send the transaction
        const rawTransaction = signedTx.serialize();
        const txid = await connection.sendRawTransaction(rawTransaction, {
            skipPreflight: true,
            maxRetries: 3,
            preflightCommitment: "processed"
        });
        
        console.log(`Transaction sent: https://solscan.io/tx/${txid}?cluster=custom&customUrl=${connection.rpcEndpoint}`);

        // Confirm the transaction
        const confirmed = await connection.confirmTransaction(txid, commitment);
        
        if (confirmed.value.err) {
            throw new Error(`Transaction failed: ${JSON.stringify(confirmed.value.err)}`);
        }

        console.log("Transaction confirmed successfully");
        return txid;
    } catch (error) {
        console.error("Transaction execution error:", error);
        throw error;
    }
};
