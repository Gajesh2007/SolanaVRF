import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import "mocha";
import { Vrf } from "../target/types/vrf";

describe("VRF", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vrf as Program<Vrf>;


  it("Is initialized!", async () => {
    let btc = new anchor.web3.PublicKey("9AR8aftLBcspD1CKgzYBqtKQUTifYsYaA5xRSary1ntM");
    let eth = new anchor.web3.PublicKey("H4ZLU7F3QMLqTkn9CM4dtTkcEtfjncvK2hacjDwmrCXv");
    let sol = new anchor.web3.PublicKey("7AFybWd6zMQnkUsvpxc8CnSFUFWzVfGq9tMVxUVue1bk");

    const tx = await program.rpc.readResult(
      {},
      {
        accounts: {
          btcAggregator: btc,
          ethAggregator: eth,
          solAggregator: sol,
        },
      }
    );
    console.log("Read result signature", tx);
  });
});
