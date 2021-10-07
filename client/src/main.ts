 import {
    establishConnection,
  } from './lib';
  
  async function main() {
      // Establish connection to the cluster
    await establishConnection();
  
    console.log('Success');
  }
  
  main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );
  