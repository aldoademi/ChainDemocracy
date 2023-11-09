import * as web3 from '@solana/web3.js';
import * as borsh from '@project-serum/borsh';
import * as dotenv from 'dotenv';
import * as fs from 'fs'

dotenv.config();

function initializeSignerKeypair(): web3.Keypair {
    if (!process.env.PRIVATE_KEY) {
        console.log('Creating .env file')
        const signer = web3.Keypair.generate()
        fs.writeFileSync('.env', `PRIVATE_KEY=[${signer.secretKey.toString()}]`)
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
  

const TOTAL_TRANSACTIONS = 5000; // Numero totale di transazioni da inviare

const electionInstructionLayout = borsh.struct([
    borsh.u8('variant'),
    borsh.str('electoral_card_number'),
    borsh.str('first_name'),
    borsh.str('last_name'),
    borsh.str('election_name'),
    borsh.str('seed'),
]);

const names = [
    { firstName: "Matteo", lastName: "Salvini" },
    { firstName: "Giuseppe", lastName: "Conte" },
    { firstName: "Luigi", lastName: "Di Maio" },
    { firstName: "Giorgia", lastName: "Meloni" },
    { firstName: "Silvio", lastName: "Berlusconi" },
    { firstName: "Matteo", lastName: "Renzi" }
];

function selectRandomName() {
    const randomIndex = Math.floor(Math.random() * names.length);
    return names[randomIndex];
}

async function sendSingleVote(
    signer: web3.Keypair, 
    programId: web3.PublicKey, 
    connection: web3.Connection, 
    index: number,
    pda_candidate_list: web3.PublicKey,
    pda_election: web3.PublicKey,
    firstName: string,
    lastName: string
    ) {

    let buffer = Buffer.alloc(1000);
    const electoral_card_number = `FF${index}`;
    const first_name = firstName;
    const last_name = lastName;
    const election_name = 'Elettorale1';
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
    console.log(`Transaction ${index} submitted with signature: ${tx} - https://explorer.solana.com/tx/${tx}?cluster=custom `);
}

async function sendVoteConcurrent(signer: web3.Keypair, programId: web3.PublicKey, connection: web3.Connection, pda_candidate_list: web3.PublicKey, pda_election: web3.PublicKey) {
    const promises = [];
    for (let i = 0; i < TOTAL_TRANSACTIONS; i++) {
        const randomName = selectRandomName();

        promises.push(sendSingleVote(signer, programId, connection, i, pda_candidate_list, pda_election, randomName.firstName, randomName.lastName));
    }
    await Promise.all(promises);
}

async function main() {
    const connection = new web3.Connection('http://127.0.0.1:8899');
    const signer = initializeSignerKeypair();
    const chainDemocracyProgramId = new web3.PublicKey('DEVqjbNXCGwT2rjLCVk6qUtVVtyCn2yLE88ChNkRLiWZ');

    const election_name = 'Elettorale1';
    const seed = 'candidate-list';

    const[pda_candidate_list] = await web3.PublicKey.findProgramAddress(
        [chainDemocracyProgramId.toBuffer(), Buffer.from(election_name),Buffer.from(seed)],
        chainDemocracyProgramId
    )

    const[pda_election] = await web3.PublicKey.findProgramAddress(
        [chainDemocracyProgramId.toBuffer(), Buffer.from(election_name)],
        chainDemocracyProgramId
    )
    
    await sendVoteConcurrent(signer, chainDemocracyProgramId, connection, pda_candidate_list, pda_election);
}

main().then(() => {
    console.log('Finished successfully');
    process.exit(0);
}).catch(error => {
    console.log(error);
    process.exit(1);
});