import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { OracleSwap } from "../target/types/oracle_swap";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("oracle_swap", () => {
    // 设置 provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // 加载程序
    const program = anchor.workspace.OracleSwap as Program<OracleSwap>;


    // 临时创建一个 Vault 账户
    let solVault: PublicKey;
    let vaultKeypair = anchor.web3.Keypair.generate();
    let rentValue: number;

    before(async () => {
        solVault = vaultKeypair.publicKey;

        rentValue = await provider.connection.getMinimumBalanceForRentExemption(0);
        // 初始化 Vault 账户并分配 SOL
        const tx = new anchor.web3.Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: provider.wallet.publicKey,
                newAccountPubkey: solVault,
                lamports: rentValue,
                space: 0,
                programId: SystemProgram.programId,
            })
        );
        await provider.sendAndConfirm(tx, [vaultKeypair]);
    });

    it("Initialize the program", async () => {
        const tx = await program.methods
            .initialize()
            .accounts({
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        console.log("Transaction signature for initialize:", tx);
    });

    it("Admin deposits SOL", async () => {
        // 定义要存入的 SOL 数量
        const depositAmount = new BN(1_000_000_000); // 1 SOL

        const tx = await program.methods
            .deposit(depositAmount)
            .accounts({
                admin: provider.wallet.publicKey,
                solVault: solVault,
            })
            .rpc();

        console.log("Transaction signature for deposit:", tx);

        // 检查 Vault 的余额以验证存款
        const vaultBalance: number =  await provider.connection.getBalance(solVault);
        expect(vaultBalance).equal(depositAmount.toNumber() + rentValue);
    });
});