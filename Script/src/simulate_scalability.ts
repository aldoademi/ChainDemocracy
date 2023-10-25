import * as web3 from '@solana/web3.js';
import * as borsh from '@project-serum/borsh';

async function simulateScalability() {
  const connection = new web3.Connection('https://api.devnet.solana.com');

  // Genera un account chiave casuale per ogni transazione
  const keypairs = Array.from({ length: 100 }, () => web3.Keypair.generate());

  async function airdropSolIfNeeded(signer: web3.Keypair) {
    const balance = await connection.getBalance(signer.publicKey);
    console.log('Current balance is', balance);
    if (balance < web3.LAMPORTS_PER_SOL) {
      console.log('Airdropping 1 SOL...');
      await connection.requestAirdrop(signer.publicKey, web3.LAMPORTS_PER_SOL);
    }
  }

  // Chiamare la funzione airdrop per ogni account chiave
  await Promise.all(keypairs.map(airdropSolIfNeeded));

  // Crea e invia le transazioni simultanee
  const transactions = keypairs.map((keypair, index) => {
    const transaction = new web3.Transaction();
    const buffer = Buffer.alloc(1000);
    const first_name = `User_${index}`;
    const last_name = 'LastName';
    const election_name = 'Election1';
    const seed = 'candidate-list';

    electionInstructionLayout.encode(
      {
        variant: 2,
        first_name: first_name,
        last_name: last_name,
        election_name: election_name,
        seed: seed,
      },
      buffer
    );

    const instruction = new web3.TransactionInstruction({
      programId: chainDemocracyProgramId,
      data: buffer,
      keys: [
        {
          pubkey: keypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: pda_candidate_list,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
    });

    transaction.add(instruction);
    return { transaction, keypair };
  });

  // Invia le transazioni simultanee
  const promises = transactions.map(async (txData, index) => {
    try {
      const signature = await web3.sendAndConfirmTransaction(connection, txData.transaction, [txData.keypair]);
      console.log(`Transaction ${index} submitted with signature: ${signature}`);
    } catch (error) {
      console.error(`Transaction ${index} failed: ${error}`);
    }
  });

  await Promise.all(promises);
}

const chainDemocracyProgramId = new web3.PublicKey('9a9etVfmxwiSjat1QZV2EZZyfqggpSNogh5yhYTqnnqE');
const electionInstructionLayout = borsh.struct([
  borsh.u8('variant'),
  borsh.str('first_name'),
  borsh.str('last_name'),
  borsh.str('election_name'),
  borsh.str('seed'),
]);

simulateScalability().catch((error) => {
  console.error('Error:', error);
});
