import {
    Connection
} from '@solana/web3.js';

import {getRpcUrl} from './utils';

let connection: Connection;

export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}
