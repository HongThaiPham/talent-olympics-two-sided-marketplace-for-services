import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsTwoSidedMarketplaceForServices } from "../target/types/talent_olympics_two_sided_marketplace_for_services";

describe("talent-olympics-two-sided-marketplace-for-services", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TalentOlympicsTwoSidedMarketplaceForServices as Program<TalentOlympicsTwoSidedMarketplaceForServices>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
