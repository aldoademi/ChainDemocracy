import * as web3 from '@solana/web3.js'
import * as borsh from '@project-serum/borsh'
import * as fs from 'fs'
import dotenv from 'dotenv'
dotenv.config()

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

async function airdropSolIfNeeded(signer: web3.Keypair, connection: web3.Connection) {
    const balance = await connection.getBalance(signer.publicKey)
    console.log('Current balance is', balance)
    if (balance < web3.LAMPORTS_PER_SOL) {
        console.log('Airdropping 1 SOL...')
        await connection.requestAirdrop(signer.publicKey, web3.LAMPORTS_PER_SOL*3)
    }
}

const electionInstructionLayout = borsh.struct([
    borsh.u8('variant'),
    borsh.str('election_name'),
    
    
])

async function sendTestElection(signer: web3.Keypair, programId: web3.PublicKey, connection: web3.Connection) {
    let buffer = Buffer.alloc(1000)
    const election_name = 'Elettorale1'
    const candidate_list_seed = 'candidate-list'
    const result_seed = 'result'
    electionInstructionLayout.encode(
        {
            variant: 3,
            election_name:election_name,
            
        },
        buffer
    )

    buffer = buffer.slice(0, electionInstructionLayout.getSpan(buffer))


    const [election_pda] = await web3.PublicKey.findProgramAddress(
        [programId.toBuffer(), Buffer.from(election_name)],
        programId
    )

    const[pda_candidate_list] = await web3.PublicKey.findProgramAddress(
        [programId.toBuffer(), Buffer.from(election_name),Buffer.from(candidate_list_seed)],
        programId
    )

    const[result_pda] = await web3.PublicKey.findProgramAddress(
        [programId.toBuffer(), Buffer.from(election_name), Buffer.from(result_seed)],
        programId
    )

    console.log("PDA is:", result_pda.toBase58())




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
                pubkey: election_pda,
                isSigner: false,
                isWritable: true
            },
            {
                pubkey: pda_candidate_list,
                isSigner: false,
                isWritable: true
            },
            {
                pubkey: result_pda,
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

function pausaPerSecondi(secondi: number): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(resolve, secondi * 1000);
    });
}

async function main() {    
    const connection = new web3.Connection("http://127.0.0.1:8899")

    const signer =  web3.Keypair.generate()
    await airdropSolIfNeeded(signer, connection)

    await pausaPerSecondi(20)
    
    const chainDemocracyProgramId = new web3.PublicKey('9UWSBaRmDNnaFwKADFVpZMJMstoAYWZPFHA6ej93dYKm')
    await sendTestElection(signer, chainDemocracyProgramId, connection)
}

main().then(() => {
    console.log('Finished successfully')
    process.exit(0)
}).catch(error => {
    console.log(error)
    process.exit(1)
})