import * as web3 from '@solana/web3.js'
import * as borsh from '@project-serum/borsh'
import * as fs from 'fs'
import dotenv from 'dotenv'
dotenv.config()

async function airdropSolIfNeeded(signer: web3.Keypair, connection: web3.Connection) {
    const balance = await connection.getBalance(signer.publicKey)
    console.log('Current balance is', balance)
    if (balance < web3.LAMPORTS_PER_SOL) {
        console.log('Airdropping 2 SOL...')
        await connection.requestAirdrop(signer.publicKey, web3.LAMPORTS_PER_SOL * 1)
    }
}

const electionInstructionLayout = borsh.struct([
    borsh.u8('variant'),
    borsh.str('first_name'),
    borsh.str('last_name'),
    borsh.str('election_name'),
    borsh.str('seed')
    
])

async function createCandidate(signer: web3.Keypair, programId: web3.PublicKey, connection: web3.Connection, firstName: string, lastName: string ){
    let buffer = Buffer.alloc(1000)
    const first_name = firstName
    const last_name = lastName
    const election_name = 'Elettorale1'
    const seed = 'candidate-list'
    electionInstructionLayout.encode(
        {
            variant: 1,
            first_name: first_name,
            last_name: last_name,
            election_name:election_name,
            seed:seed
        },
        buffer
    )

    buffer = buffer.slice(0, electionInstructionLayout.getSpan(buffer))


    const [pda] = await web3.PublicKey.findProgramAddress(
        [signer.publicKey.toBuffer(), Buffer.from(election_name), Buffer.from(first_name),Buffer.from(last_name)],
        programId
    )

    const[pda_candidate_list] = await web3.PublicKey.findProgramAddress(
        [programId.toBuffer(), Buffer.from(election_name),Buffer.from(seed)],
        programId
    )

    const[election_pda_account] = await web3.PublicKey.findProgramAddress(
        [programId.toBuffer(), Buffer.from(election_name)],
        programId
    )

    console.log("PDA is:", pda.toBase58())

    const transaction = new web3.Transaction()
    
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
                pubkey: election_pda_account,
                isSigner: false,
                isWritable: true
            },
            {
                pubkey: web3.SystemProgram.programId,
                isSigner: false,
                isWritable: false
            }
        ]
    })

    transaction.add(instruction)
    const tx = await web3.sendAndConfirmTransaction(connection, transaction, [signer])
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom`)
}

function waitAirdropSol(secondi: number): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(resolve, secondi * 1000);
    });
}

async function main() {
    const signer =  web3.Keypair.generate()
    
    const connection = new web3.Connection("http://127.0.0.1:8899")
    await airdropSolIfNeeded(signer, connection)

    await waitAirdropSol(15)
    
    const chainDemocracyProgramId = new web3.PublicKey('9UWSBaRmDNnaFwKADFVpZMJMstoAYWZPFHA6ej93dYKm')          // ALDO
    await createCandidate(signer, chainDemocracyProgramId, connection, 'Matteo', 'Salvini')
    await createCandidate(signer, chainDemocracyProgramId, connection, 'Giuseppe', 'Conte')
    await createCandidate(signer, chainDemocracyProgramId, connection, 'Luigi', 'Di Maio')
    await createCandidate(signer, chainDemocracyProgramId, connection, 'Giorgia', 'Meloni')
    await createCandidate(signer, chainDemocracyProgramId, connection, 'Silvio', 'Berlusconi')
    await createCandidate(signer, chainDemocracyProgramId, connection, 'Matteo', 'Renzi')
}

main().then(() => {
    console.log('Finished successfully')
    process.exit(0)
}).catch(error => {
    console.log(error)
    process.exit(1)
})