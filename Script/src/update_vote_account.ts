import * as web3 from '@solana/web3.js'
import * as borsh from '@project-serum/borsh'
import * as fs from 'fs'
import dotenv from 'dotenv'
dotenv.config()

async function airdropSolIfNeeded(signer: web3.Keypair, connection: web3.Connection) {
    const balance = await connection.getBalance(signer.publicKey)
    console.log('Current balance is', balance)
    if (balance < web3.LAMPORTS_PER_SOL) {
        console.log('Airdropping 1 SOL...')
        await connection.requestAirdrop(signer.publicKey, web3.LAMPORTS_PER_SOL)
    }
}

const updateVoteAccountLayout = borsh.struct([
    borsh.u8('variant'),
    borsh.str('name'),
])

async function sendTestElection(signer: web3.Keypair, programId: web3.PublicKey, connection: web3.Connection) {
    let buffer = Buffer.alloc(1000)
    const voteAccountName = 'Nazionali2023'
    updateVoteAccountLayout.encode(
        {
            variant: 3,
            name: voteAccountName,
        },
        buffer
    )

    buffer = buffer.slice(0, updateVoteAccountLayout.getSpan(buffer))


    const [pda] = await web3.PublicKey.findProgramAddress(
        [programId.toBuffer(), Buffer.from(voteAccountName)],
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
    const signer =  web3.Keypair.generate()
    
    const connection = new web3.Connection("http://127.0.0.1:8899")
    await airdropSolIfNeeded(signer, connection)

    await pausaPerSecondi(15)
    
    // const chainDemocracyProgramId = new web3.PublicKey('9a9etVfmxwiSjat1QZV2EZZyfqggpSNogh5yhYTqnnqE')       // DAVIDE
    // const chainDemocracyProgramId = new web3.PublicKey('Hr7MuMT6ZmEVQtewmHnAbe3mAQ6j42toicBe7bU6rJX')        // DAVIDE
    const chainDemocracyProgramId = new web3.PublicKey('8Y4BdSZcoQGp7U1UKqpxjZh8ar7DN2xsMehETFKUvej8')          // ALDO
    await sendTestElection(signer, chainDemocracyProgramId, connection)
}

main().then(() => {
    console.log('Finished successfully')
    process.exit(0)
}).catch(error => {
    console.log(error)
    process.exit(1)
})