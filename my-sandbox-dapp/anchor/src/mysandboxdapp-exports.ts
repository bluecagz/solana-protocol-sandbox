// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import MysandboxdappIDL from '../target/idl/mysandboxdapp.json'
import type { Mysandboxdapp } from '../target/types/mysandboxdapp'

// Re-export the generated IDL and type
export { Mysandboxdapp, MysandboxdappIDL }

// The programId is imported from the program IDL.
export const MYSANDBOXDAPP_PROGRAM_ID = new PublicKey(MysandboxdappIDL.address)

// This is a helper function to get the Mysandboxdapp Anchor program.
export function getMysandboxdappProgram(provider: AnchorProvider) {
  return new Program(MysandboxdappIDL as Mysandboxdapp, provider)
}

// This is a helper function to get the program ID for the Mysandboxdapp program depending on the cluster.
export function getMysandboxdappProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Mysandboxdapp program on devnet and testnet.
      return new PublicKey('CounNZdmsQmWh7uVngV9FXW2dZ6zAgbJyYsvBpqbykg')
    case 'mainnet-beta':
    default:
      return MYSANDBOXDAPP_PROGRAM_ID
  }
}
