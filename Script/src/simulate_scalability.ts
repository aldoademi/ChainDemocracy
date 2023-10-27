import * as web3 from '@solana/web3.js';
import * as borsh from '@project-serum/borsh';
import * as dotenv from 'dotenv';
import * as fs from 'fs'

dotenv.config();

function initializeSignerKeypair(): web3.Keypair {
    if (!process.env.PRIVATE_KEY) {
        console.log('Creating .env file')
        const signer = web3.Keypair.generate()
        fs.writeFileSync('.env',`PRIVATE_KEY=[${signer.secretKey.toString()}]`)
        return signer
    }
    
    const secret = JSON.parse(process.env.PRIVATE_KEY ?? "") as number[]
    const secretKey = Uint8Array.from(secret)
    const keypairFromSecretKey = web3.Keypair.fromSecretKey(secretKey)
    console.log('Signer public key:', keypairFromSecretKey.publicKey.toBase58())
    return keypairFromSecretKey
}

const {
    Transaction,
    TransactionInstruction,
    SystemProgram,
    PublicKey,
  } = web3;
  

const TOTAL_TRANSACTIONS = 10; // Numero totale di transazioni da inviare

async function airdropSolIfNeeded(signer: web3.Keypair, connection: web3.Connection) {
    const balance = await connection.getBalance(signer.publicKey);
    console.log('Current balance is', balance);
    if (balance < web3.LAMPORTS_PER_SOL) {
        console.log('Airdropping 1 SOL...');
        await connection.requestAirdrop(signer.publicKey, web3.LAMPORTS_PER_SOL);
    }
}

const electionInstructionLayout = borsh.struct([
    borsh.u8('variant'),
    borsh.str('electoral_card_number'),
    borsh.str('first_name'),
    borsh.str('last_name'),
    borsh.str('election_name'),
    borsh.str('seed'),
]);

async function sendTestElection(signer: web3.Keypair, programId: web3.PublicKey, connection: web3.Connection) {

    for (let i = 0; i < TOTAL_TRANSACTIONS; i++) {
        let buffer = Buffer.alloc(1000);
        const electoral_card_number = `EC${i}`;
        // const first_name = `User_${i}`;
        const first_name = 'Aldo';
        const last_name = 'Ademi';
        const election_name = 'Test1';
        const seed = 'candidate-list';
        electionInstructionLayout.encode(
            {
                variant: 2,
                electoral_card_number: electoral_card_number,
                first_name: first_name,
                last_name: last_name,
                election_name: election_name,
                seed: seed
            },
            buffer
        );

        buffer = buffer.slice(0, electionInstructionLayout.getSpan(buffer));

        const [pda] = await web3.PublicKey.findProgramAddress(
            [programId.toBuffer(), Buffer.from(election_name), Buffer.from(electoral_card_number)],
            programId
        )

        const[pda_candidate_list] = await web3.PublicKey.findProgramAddress(
            [programId.toBuffer(), Buffer.from(election_name),Buffer.from(seed)],
            programId
        )
    
        const[pda_election] = await web3.PublicKey.findProgramAddress(
            [programId.toBuffer(), Buffer.from(election_name)],
            programId
        )
  
      console.log("PDA is:", pda.toBase58())


        const instruction = new web3.TransactionInstruction({
            programId: programId,
            data: buffer,
            keys: [
                {
                    pubkey: signer.publicKey,
                    isSigner: true,
                    isWritable: false
                },
                {
                    pubkey: pda,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: pda_candidate_list,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: pda_election,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: web3.SystemProgram.programId,
                    isSigner: false,
                    isWritable: false
                }
            ]
        });

        const transaction = new web3.Transaction();
        transaction.add(instruction);
        const tx = await web3.sendAndConfirmTransaction(connection, transaction, [signer]);
        console.log(`Transaction ${i} submitted with signature: ${tx} - https://explorer.solana.com/tx/${tx}?cluster=devnet `);
    }
}

function pausaPerSecondi(secondi: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, secondi * 1000);
  });
}

async function main() {
    // const signer = web3.Keypair.generate();
    // const connection = new web3.Connection('http://127.0.0.1:8899');
    // await airdropSolIfNeeded(signer, connection);
    // await pausaPerSecondi(15);
    const signer = initializeSignerKeypair();
    let connection = new web3.Connection(web3.clusterApiUrl("devnet"));
    const chainDemocracyProgramId = new web3.PublicKey('4ViuBVhMASkeaX8RHc3gDQsBEmFdDKcXbCPXKoeWRxAa');
    await sendTestElection(signer, chainDemocracyProgramId, connection);
}

main().then(() => {
    console.log('Finished successfully');
    process.exit(0);
}).catch(error => {
    console.log(error);
    process.exit(1);
});