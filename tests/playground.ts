import { AnchorProvider, setProvider, workspace, Program, web3 } from "@coral-xyz/anchor";
import { Playground } from "../target/types/playground";
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { expect } from "chai";

describe("playground", () => {
    const provider = AnchorProvider.env();
    setProvider(provider);

    const program = workspace.Playground as Program<Playground>;

    before(async () => {

    })

})