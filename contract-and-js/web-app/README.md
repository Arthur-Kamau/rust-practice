## pre requisites
1. scarb.
2. vite, node and npm
3. starknet-devnet

## steps
1. run `starknet-devnet` in  your os
2. run scarb build in `contract-and-js` folder.
3. use `https://stark-deployer.vercel.app/` to deploy the smart contract and get the smart contract account address.
4. update the const `contractAddress`  in `app.tsx`
5. update the `account0Address` and `privateKey0` in `app.tsx`
6. npm run dev in `web-app`
