import { connect } from "get-starknet";
import { Contract, RpcProvider, Account } from "starknet";
import { createSignal } from "solid-js";
import type { Component } from 'solid-js';

const App: Component = () => {
  const contractAddress = `0x4d692f05e51b706a278e2007ecfdbb6641274000b37ceede8f972306af88277`;
  const [provider0, setProvider0] = createSignal(new RpcProvider({ nodeUrl: "http://127.0.0.1:5050" }));
  const [amount, setAmount] = createSignal(1);
  const [address, setAddress] = createSignal("");
  const [isConnected, setIsConnected] = createSignal(false);

  const updateValue = async () => {
    try {
      if (!isConnected()) {
        throw new Error("Wallet not connected.");
      }

      const myNodeUrl = "http://127.0.0.1:5050";
      const provider = new RpcProvider({ nodeUrl: myNodeUrl });

      const { abi: testAbi } = await provider.getClassAt(contractAddress);
      if (testAbi === undefined) {
        throw new Error("No ABI found.");
      }
      const myTestContract = new Contract(testAbi, contractAddress, provider);

      const bal0 = await myTestContract.get_balance();
      console.log("Initial balance =", bal0);

      const account0Address = `0x38ccd8a44af159fcdedc2d70cda091e5abaeebcbaba509d35159714ab0755b5`;
      const privateKey0 = `0x00000000000000000000000000000000e22675dbe199a5a577a369f045da6b26`;
      const account0 = new Account(provider, account0Address, privateKey0);

      myTestContract.connect(account0);

      // Populate the transaction
      const tx = myTestContract.populate("increase_balance", [10]);

      // Estimate fee with simulation flags
      const feeEstimate = await account0.estimateInvokeFee(tx);
      console.log("Estimated fee:", feeEstimate.suggestedMaxFee);

      // Execute the transaction with the estimated fee
      const res0 = await account0.execute(tx, undefined, {
        maxFee: feeEstimate.suggestedMaxFee,
      });
      await provider.waitForTransaction(res0.transaction_hash);

      console.log("Transaction completed");

      alert("You successfully incremented the counter!");
    } catch (err) {
      console.error(err);
      alert(err.message);
    }
  };

  const connectWallet = async () => {
    try {
      const starknet = await connect();
      if (!starknet) throw new Error("Failed to connect to wallet.");
      await starknet.enable({ starknetVersion: "v5" });
      setProvider(starknet.account);
      setAddress(starknet.selectedAddress || "");
      setIsConnected(true);
    } catch (error) {
      console.error(error);
      alert(error.message);
    }
  };

  return (
    <div>
      <div>
        <p>Latest block hash --</p>
        {isConnected() ? (
          <div>
            Wallet  Connected

            <p>
              Amount to increament
              <input
                type="number"
                value={amount()}
                onChange={(e) => setAmount(Number(e.target.value))}
              />
              <button onClick={updateValue}>increment</button>
            </p>
          </div>
        ) : (
          <div>
            <span>Choose a wallet:</span>
            <p>
              <button onClick={connectWallet}>Connect a Wallet</button>
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default App;