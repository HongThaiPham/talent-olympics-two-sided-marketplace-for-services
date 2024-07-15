import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsTwoSidedMarketplaceForServices } from "../target/types/talent_olympics_two_sided_marketplace_for_services";
import { assert } from "chai";
import { randomBytes } from "crypto";

describe("talent-olympics-two-sided-marketplace-for-services", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;
  provider.opts.commitment = "confirmed";
  anchor.setProvider(provider);

  const program = anchor.workspace
    .TalentOlympicsTwoSidedMarketplaceForServices as Program<TalentOlympicsTwoSidedMarketplaceForServices>;

  const FEE = new anchor.BN(1_00_000_000);
  const VENDOR_ID = new anchor.BN(randomBytes(8));

  const assetArgs = {
    id: new anchor.BN(randomBytes(8)),
    name: "Solana Talent Olympics NFT 2024",
    uri: "https://ipfs.io/ipfs/QmQQYq41wkaAu5ekxv3xeDbSKyribYvHP8Pz7kPddYvvwB",
    agreements: "Some agreements",
    price: new anchor.BN(1_000_000_000),
  };

  const [admin, user1, user2] = [
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
  ];

  const [vendorAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vendor"), VENDOR_ID.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  it("Init test successfully", async () => {
    const tx = await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx);

    const tx2 = await provider.connection.requestAirdrop(
      user1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx2);

    const tx3 = await provider.connection.requestAirdrop(
      user2.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx3);
  });

  it("Should init protocol successfully", async () => {
    const tx = await program.methods
      .initialize(FEE)
      .accounts({
        signer: admin.publicKey,
      })
      .signers([admin])
      .rpc();
    assert.ok(tx);
    console.log("Protocol initialized successfully at tx: ", tx);
  });

  it("Should update fee successfully", async () => {
    const tx = await program.methods
      .setFee(new anchor.BN(1_000_000_000))
      .accounts({
        signer: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    assert.ok(tx);

    console.log("Fee updated successfully at tx: ", tx);
  });

  it("Should create vendor successfully", async () => {
    const tx = await program.methods
      .createVendor(VENDOR_ID, "Vendor 1")
      .accountsPartial({
        signer: user1.publicKey,
        vendor: vendorAccount,
      })
      .signers([user1])
      .rpc();

    assert.ok(tx);

    console.log("Vendor created successfully at tx: ", tx);
  });

  it("Should create service successfully", async () => {
    const [serviceAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("service"),
        vendorAccount.toBuffer(),
        assetArgs.id.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const asset = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .createService(
        VENDOR_ID,
        assetArgs.id,
        assetArgs.price,
        assetArgs.name,
        assetArgs.uri,
        assetArgs.agreements,
        true,
        500
      )
      .accountsPartial({
        signer: user1.publicKey,
        vendor: vendorAccount,
        service: serviceAccount,
        logWrapper: null,
        asset: asset.publicKey,
      })
      .signers([user1, asset])
      .rpc();

    assert.ok(tx);

    console.log("Service created successfully at tx: ", tx);
  });
});
