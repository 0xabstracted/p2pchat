import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {PublicKey} from "@solana/web3.js";
import { PToPChat } from "../target/types/p_to_p_chat";
import {expect} from "chai";

describe("p_to_p_chat", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PToPChat as Program<PToPChat>;
  const receiverAddress = anchor.web3.Keypair.generate();
  const vistorAddress = anchor.web3.Keypair.generate();
  
  it("creates User Profile", async () => {
    const [userProfilePDA, userProfileBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("user_profile"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const cpp ={
      userName: "abstracted",
      userLocation: "Hyderabad"
    }
    const tx = await program.methods
                    .createProfile(cpp)
                    .accounts({
                      userProfile: userProfilePDA,                               
                      user: provider.wallet.publicKey,
    }).rpc();
    expect((await program.account.userProfile.fetch(userProfilePDA)).userName).to.equal("abstracted");
    const userProfiles = await program.account.userProfile.all();
    console.log("create User Profile");
    console.log(userProfiles);
    const messages = await program.account.message.all();
    console.log(messages);
   
    console.log("1. Your transaction signature", tx);
  });
  it("updates user Profile", async () => {
    const [userProfilePDA, userProfileBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("user_profile"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const upp ={
      userName: "bhargav",
      userLocation: "India"
    }
    const tx = await program.methods
                    .updateProfile(upp)
                    .accounts({
                      user: provider.wallet.publicKey,
                      userProfile: userProfilePDA,                  
    }).rpc();
    const userProfiles = await program.account.userProfile.all();
    console.log(userProfiles);
    const messages = await program.account.message.all();
    console.log(messages);
    expect((await program.account.userProfile.fetch(userProfilePDA)).userLocation).to.equal("India");
    console.log("2. Your transaction signature", tx);
  });
  it("likes user Profile", async () => {
    const userProfiles = await program.account.userProfile.all();
    console.log(userProfiles);
    const tx = await program.methods
                    .likeProfile()
                    .accounts({
                      visitor: vistorAddress.publicKey,
                      userProfile: userProfiles[0].publicKey,                  
    }).rpc();
    expect((await program.account.userProfile.fetch(userProfiles[0].publicKey)).likes).to.equal(1);
    console.log("3. Your transaction signature", tx);
  });
  it("send a message from the user", async () => {
    const [messagePDA, messageBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("message"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const upp ={
      receiverAddress:   receiverAddress.publicKey,
      message: "Excited to talk to Metaplex!"
    }
    
    const tx = await program.methods
                    .sendMessage(upp)
                    .accounts({
      message: messagePDA,
      sender: provider.wallet.publicKey,
    }).rpc();
    console.log("4. Your transaction signature", tx);
    
    expect((await program.account.message.fetch(messagePDA)).message).to.equal("Excited to talk to Metaplex!");
    });
    it("read receipt", async () => {
      const [messagePDA, messageBump] = await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("message"),
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId
      );
      const messages = await program.account.message.all();
      
      const tx = await program.methods
                      .readMessage()
                      .accounts({
        message: messages[0].publicKey,
        receiver: provider.wallet.publicKey,
      }).rpc();
      console.log("5. Your transaction signature", tx);
      
      expect((await program.account.message.fetch(messages[0].publicKey)).readReceipt).to.equal(true);
      });
    
});
