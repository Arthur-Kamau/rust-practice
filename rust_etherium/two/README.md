
## rust interaction with a smart contract

### deps
1. rust(cargo, rust up ....)
2. node and npm
3. npm install -g truffle
4. npm install -g ganache

### description
1. in contract is a truffle project using `truffle init` then ` truffle create contract counter`
2. `truffle compile` to compile smart contract
3. `truffle migrate --network development` to deploy smart contract

## interact using truffle console
1. `truffle console` (in contract folder)
2. `let instance = await StoreValue.deployed()` 
3. `instance.set(42)`
4. `instance.get()`