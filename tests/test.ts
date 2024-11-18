import * as anchor from '@coral-xyz/anchor';
import * as solanaWeb3 from "@solana/web3.js";
import base58 from "bs58";

import {
    getOrCreateAssociatedTokenAccount,
    transfer,
    mintTo,
    TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    createTransferInstruction,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddress,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { token } from '@coral-xyz/anchor/dist/cjs/utils';

const BN = require('bn.js');

const curveSeed = "CurveConfiguration"
const POOL_SEED_PREFIX = "liquidity_pool"
const SOL_VAULT_PREFIX = "liquidity_sol_vault"
const POOL_SEED_PREFIX_ESCROW = "escrow_pool"
const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

async function example() {

    const private_key = "62ZpFMJ6fRqjmxkp4ibSEqhq11YL123Ba3FvYqVeiMXHr7pqKRwsVA34BesGiaomoF5cj6j9i7XsY3WxBWudW4Zv";
    const admin_keypair = base58.decode(private_key);
    const my_account = anchor.web3.Keypair.fromSecretKey(admin_keypair);
    const my_wallet = new anchor.Wallet(my_account);

    // Connect to the network
    const connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl('devnet'), 'confirmed');
    // const connection = new solanaWeb3.Connection('http://127.0.0.1:8899/', 'confirmed');
    const provider = new anchor.AnchorProvider(connection, my_wallet, anchor.AnchorProvider.defaultOptions());
    anchor.setProvider(provider);

    console.log("heloo")
    const idl = require('./idl_fun.json');
    console.log("heloo")
    const program: any = new anchor.Program(idl, provider);
    console.log("heloo")

    console.log("create initialize");
    // try {
    //         const [curveConfig] = solanaWeb3.PublicKey.findProgramAddressSync(
    //             [Buffer.from(curveSeed)],
    //             program.programId
    //         )
    //         console.log("curveConfig: ",curveConfig)

    //         const [poolSolCurves, bump] = solanaWeb3.PublicKey.findProgramAddressSync(
    //             [Buffer.from(SOL_VAULT_PREFIX), curveConfig.toBuffer()],
    //             program.programId
    //         )
    //         console.log("poolSolCurves",poolSolCurves);            
    //         const tx = new solanaWeb3.Transaction()
    //             .add(
    //                 await program.methods
    //                     .initialize(10,my_wallet.publicKey)
    //                     .accounts({
    //                         dexConfigurationAccount: curveConfig,
    //                         poolSolCurves: poolSolCurves,
    //                         admin: my_wallet.publicKey,
    //                         rent: solanaWeb3.SYSVAR_RENT_PUBKEY,
    //                         systemProgram: anchor.web3.SystemProgram.programId
    //                     })
    //                     .instruction()
    //             )
    //         tx.feePayer = my_wallet.publicKey
    //         tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //         console.log(await connection.simulateTransaction(tx))
    //         const sig = await solanaWeb3.sendAndConfirmTransaction(connection, tx, [my_account], { skipPreflight: true })
    //         console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)

    //         let pool = await program.account.curveConfiguration.fetch(curveConfig)
    //         console.log("Pool State : ", pool)

    //     } catch (error) {
    //         console.log("Error in initialization :", error)
    // }
    
    console.log("create token 20");
    // try {
    //     const mintAccountToken = anchor.web3.Keypair.generate();
    //     console.log("mintAccountToken: ",mintAccountToken.publicKey)
        
    //     const [curveConfig] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(curveSeed)],
    //         program.programId
    //     )
    //     console.log("curveConfig: ",curveConfig)

    //     const [poolSolCurves, bump] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), curveConfig.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolCurves",poolSolCurves);

    //     const [poolPda] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(POOL_SEED_PREFIX), mintAccountToken.publicKey.toBuffer()],
    //         program.programId
    //     )

    //     console.log("poolPda: ",poolPda)

    //     const [poolSolVault] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), mintAccountToken.publicKey.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolVault",poolSolVault)

    //     const poolToken = await getAssociatedTokenAddress(
    //         mintAccountToken.publicKey, poolPda, true
    //     )
    //     console.log("poolToken",poolToken);
       
    //     const metadata = {
    //         name: 'Me',
    //         symbol: 'Me',
    //         uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
    //     };
    
    //     const tx = new solanaWeb3.Transaction()
    //         .add(
    //             await program.methods
    //                 .createPoolToken20(metadata.name, metadata.symbol, metadata.uri)
    //                 .accounts({
    //                     dexConfigurationAccount: curveConfig,
    //                     poolSolCurves: poolSolCurves,
    //                     pool: poolPda,
    //                     poolSolVault: poolSolVault,
    //                     poolTokenAccount: poolToken,
    //                     payer: my_wallet.publicKey,
    //                     tokenMint: mintAccountToken.publicKey,
    //                     // tokenProgram: TOKEN_PROGRAM_ID,
    //                     // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //                     // systemProgram: solanaWeb3.SystemProgram.programId,
    //                     // rent: solanaWeb3.SYSVAR_RENT_PUBKEY,
    //                 })
    //                 .instruction()
    //         )
    //     tx.feePayer = my_wallet.publicKey
    //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //     // console.log(await connection.simulateTransaction(tx))
    //     const sig = await solanaWeb3.sendAndConfirmTransaction(connection, tx, [my_account,mintAccountToken], { skipPreflight: true })
    //     console.log("Successfully created pool : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
    
    // } catch (error) {
    //     console.log("Error in creating pool", error)
    // }

    console.log("create token 404");
    // try {
    //     const mintAccountToken = anchor.web3.Keypair.generate();
    //     // const mintAccountToken = new anchor.web3.PublicKey("A32Ekf4ucjuAWAh4ETcvXrAkQL4B5TwuH4amyZBxat87")
    //     const MintTokenPublicKey = mintAccountToken.publicKey;
    //     console.log("mintAccountToken: ",MintTokenPublicKey)

    //     const collectionMintToken = anchor.web3.Keypair.generate();
    //     const collectionPublicKey = collectionMintToken.publicKey;
    //     console.log("collectionMintToken: ",collectionMintToken.publicKey)
        
    //     const [curveConfig] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(curveSeed)],
    //         program.programId
    //     )
    //     console.log("curveConfig: ",curveConfig)

    //     const [poolSolCurves, bump] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), curveConfig.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolCurves",poolSolCurves);
        
    //     // collection 
    //     const [poolEscrow] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [
    //             Buffer.from(POOL_SEED_PREFIX_ESCROW),
    //             MintTokenPublicKey.toBuffer(),
    //             collectionMintToken.publicKey.toBuffer()
    //         ],
    //         program.programId
    //     );
    //     console.log("poolEscrow",poolEscrow);

    //     const [mintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [
    //             Buffer.from('authority'),
    //             collectionMintToken.publicKey.toBuffer()
    //         ], 
    //         program.programId
    //     );  
    //     console.log("mintAuthority",mintAuthority);
    //     const [collectionMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), collectionPublicKey.toBuffer()],
    //         TOKEN_METADATA_PROGRAM_ID,
    //     );
    //     console.log("collectionMetadata",collectionMetadata)
    //     const [collectionMasterEdition]  = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), collectionPublicKey.toBuffer(), Buffer.from('edition')],
    //         TOKEN_METADATA_PROGRAM_ID,
    //     );
    //     console.log("collectionMasterEdition",collectionMasterEdition)

    //     // token
    //     const [poolPda] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(POOL_SEED_PREFIX), MintTokenPublicKey.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolPda: ",poolPda)

    //     // const collectionTokenAccount = getAssociatedTokenAddressSync(collectionMintToken.publicKey, poolPda);
    //     const collectionTokenAccount = await getAssociatedTokenAddress(
    //         collectionMintToken.publicKey, poolPda, true
    //     )
    //     console.log("destination: ",collectionTokenAccount);

    //     const [pool_sol_vault] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), MintTokenPublicKey.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolVault",pool_sol_vault)

    //     const poolToken = await getAssociatedTokenAddress(
    //         MintTokenPublicKey, poolPda, true
    //     )
    //     console.log("poolToken",poolToken);
       
    //     const metadata = {
    //         tokenName: "My Erc404",
    //         tokenSymbol: "My404",
    //         tokenUri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    //         baseUri: "https://ipfs.ventory.gg/ventory/ino/starknet/ventorians/json/",
    //         uriSize: 5,
    //         path: Buffer.from([200,200,200,200,200])
    //     };
    //     let transaction = new solanaWeb3.Transaction();
        
    //     transaction.add(
    //             await program.methods
    //                 .createPoolToken404(metadata.tokenName, metadata.tokenSymbol, metadata.tokenUri)
    //                 .accounts({
    //                     dexConfigurationAccount: curveConfig,
    //                     poolSolCurves: poolSolCurves,
    //                     pool: poolPda,
    //                     poolSolVault: pool_sol_vault,
    //                     poolTokenAccount: poolToken,
    //                     payer: my_wallet.publicKey,
    //                     tokenMint: mintAccountToken.publicKey,
    //                     // tokenProgram: TOKEN_PROGRAM_ID,
    //                     // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //                     // systemProgram: solanaWeb3.SystemProgram.programId,
    //                     // rent: solanaWeb3.SYSVAR_RENT_PUBKEY,
    //                 })
    //                 .instruction()
    //     )
        
    //     transaction.feePayer = my_wallet.publicKey
    //     transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //     // console.log(await connection.simulateTransaction(tx))
    //     const sig = await solanaWeb3.sendAndConfirmTransaction(connection, transaction, [my_account,mintAccountToken], { skipPreflight: true })
    //     console.log("Successfully created pool 404: ", `https://solscan.io/tx/${sig}?cluster=devnet`)


    //     let transaction2 = new solanaWeb3.Transaction();
    //     transaction2.add(
    //         await program.methods
    //             .initPoolToken404(
    //                 metadata.tokenName, 
    //                 metadata.tokenSymbol, 
    //                 metadata.tokenUri,
    //                 metadata.baseUri, 
    //                 metadata.uriSize, 
    //                 metadata.path
    //             )
    //             .accounts({
    //                 collectionMint: collectionMintToken.publicKey,
    //                 poolEscrow: poolEscrow,
    //                 mint_authority: mintAuthority,
    //                 metadata: collectionMetadata,
    //                 masterEdition: collectionMasterEdition,
    //                 destination: collectionTokenAccount,
    //                 pool: poolPda,
    //                 payer: my_wallet.publicKey,
    //                 // tokenProgram: TOKEN_PROGRAM_ID,
    //                 // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //                 // systemProgram: solanaWeb3.SystemProgram.programId,
    //                 // rent: solanaWeb3.SYSVAR_RENT_PUBKEY,
    //             })
    //             .instruction()
    //     )

    //     transaction2.feePayer = my_wallet.publicKey
    //     transaction2.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //     // console.log(await connection.simulateTransaction(tx))
    //     const sig2 = await solanaWeb3.sendAndConfirmTransaction(connection, transaction2, [my_account,collectionMintToken], { skipPreflight: true })
    //     console.log("Successfully init pool 404: ", `https://solscan.io/tx/${sig2}?cluster=devnet`)
    // } catch (error) {
    //     console.log("Error in creating pool 404 ", error)
    // }

    console.log("swap mint nft");
    try {
        const MintTokenPublicKey = new anchor.web3.PublicKey("4PHqLMqDvynexiym5u7Z5bnM16o7WbJYRGxRKCpa2KC4")
        console.log("mintAccountToken: ",MintTokenPublicKey)

        const collectionPublicKey = new anchor.web3.PublicKey("DhU6TBAuhwg7x9jxcvyXRgoearhx26423etsqb1PRbfQ")
        console.log("collectionMintToken: ",collectionPublicKey)
        // collection 
        const [poolEscrow] = solanaWeb3.PublicKey.findProgramAddressSync(
            [
                Buffer.from(POOL_SEED_PREFIX_ESCROW),
                MintTokenPublicKey.toBuffer(),
                collectionPublicKey.toBuffer()
            ],
            program.programId
        );
        console.log("poolEscrow",poolEscrow);

        const [mintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('authority'),
                collectionPublicKey.toBuffer()
            ], 
            program.programId
        );  
        console.log("mintAuthority",mintAuthority);

        const [collectionMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), collectionPublicKey.toBuffer()],
            TOKEN_METADATA_PROGRAM_ID,
        );
        console.log("collectionMetadata",collectionMetadata)

        const [collectionMasterEdition]  = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), collectionPublicKey.toBuffer(), Buffer.from('edition')],
            TOKEN_METADATA_PROGRAM_ID,
        );

        console.log("collectionMasterEdition",collectionMasterEdition)

        // const collectionTokenAccount = getAssociatedTokenAddressSync(collectionMintToken.publicKey, poolPda);
        const pool_escrow_token_account = await getAssociatedTokenAddress(
            MintTokenPublicKey, poolEscrow, true
        )
        console.log("pool_escrow_token_account: ",pool_escrow_token_account);

        const user_token_account = await getAssociatedTokenAddress(
            MintTokenPublicKey, my_wallet.publicKey, true
        )
        console.log("user_token_account: ",user_token_account);

        ///
        /// NFT
        ///  
        const nftMint = anchor.web3.Keypair.generate();;
        console.log("mintNFTKeypair",nftMint.publicKey);

        const [nftMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(),nftMint.publicKey.toBuffer()],
            TOKEN_METADATA_PROGRAM_ID,
        );
        console.log("nftMetadata",nftMetadata)
        
        const [nftMasterEdition]  = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(),nftMint.publicKey.toBuffer(), Buffer.from('edition')],
            TOKEN_METADATA_PROGRAM_ID,
        );
        console.log("nftMasterEdition",nftMasterEdition)

        const destination_nft = await getAssociatedTokenAddressSync(
            nftMint.publicKey, my_wallet.publicKey
        )
        console.log("destination_nft",destination_nft)
        let transaction = new solanaWeb3.Transaction();
        transaction.add(
           
                await program.methods
                    .swapNft()
                    .accounts({
                        owner: my_wallet.publicKey,
                        mintNft: nftMint.publicKey,
                        destination: destination_nft,
                        metadata: nftMetadata,
                        masterEdition: nftMasterEdition,
                        mintAuthority: mintAuthority,
                        // collectionMint: collectionPublicKey,
                        // collectionMetadata: collectionMetadata,
                        // collectionMasterEdition: collectionMasterEdition,
                        poolEscrow: poolEscrow,
                        tokenMint: MintTokenPublicKey,
                        poolEscrowTokenAccount: pool_escrow_token_account,
                        userTokenAccount: user_token_account,
                        // tokenProgram: TOKEN_PROGRAM_ID,
                        // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                        // systemProgram: solanaWeb3.SystemProgram.programId,
                        // rent: solanaWeb3.SYSVAR_RENT_PUBKEY,
                    })
                    .instruction()
        )
        
        transaction.feePayer = my_wallet.publicKey
        transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
        // console.log(await connection.simulateTransaction(tx))
        const sig = await solanaWeb3.sendAndConfirmTransaction(connection, transaction, [my_account,nftMint], { skipPreflight: true })
        console.log("Successfully mint nft 404: ", `https://solscan.io/tx/${sig}?cluster=devnet`)

    } catch (error) {
        console.log("Error in creating pool 404 ", error)
    }
    const mint1 = new anchor.web3.PublicKey("4PHqLMqDvynexiym5u7Z5bnM16o7WbJYRGxRKCpa2KC4");
    console.log("buy")

    // try {
    //     const [curveConfig] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(curveSeed)],
    //         program.programId
    //     )
    //     console.log("curveConfig",curveConfig)
    //     const [poolSolCurves, bumpcurves] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), curveConfig.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolCurves",poolSolCurves);

    //     const [poolPda] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(POOL_SEED_PREFIX), mint1.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolPda",poolPda)

    //     const poolToken = await getAssociatedTokenAddress(
    //         mint1, poolPda, true
    //     )
    //     console.log("poolToken",poolToken)

    //     const userAccountToken = await getAssociatedTokenAddressSync(
    //         mint1, my_wallet.publicKey
    //     )
    //     console.log("userAccountToken",userAccountToken)

    //     const recipientTokenAccount = await connection.getAccountInfo(userAccountToken);

    //     const [poolSolVault] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), mint1.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolVault",poolSolVault)
    //     let transaction = new solanaWeb3.Transaction();
    //     if (!recipientTokenAccount) {
    //         // If the recipient doesn't have a token account, create one
    //         console.log("Creating associated token account for recipient");
            
    //         transaction.add(
    //             createAssociatedTokenAccountInstruction(
    //                 my_wallet.publicKey, // payer
    //                 userAccountToken, // associated token account address
    //                 my_wallet.publicKey, // owner
    //                 mint1, // mint
    //                 TOKEN_PROGRAM_ID,
    //                 ASSOCIATED_TOKEN_PROGRAM_ID
    //             )
    //         );
    //     }
    //     const tx = transaction.add(
    //             await program.methods
    //                 .buy(new anchor.BN(Number(1000000 * 10**9).toString())) // 1 sol
    //                 .accounts({
    //                     pool: poolPda,
    //                     tokenMint: mint1,
    //                     poolSolVault: poolSolVault,
    //                     poolSolCurves: poolSolCurves,
    //                     poolTokenAccount: poolToken,
    //                     userTokenAccount: userAccountToken,
    //                     dexConfigurationAccount: curveConfig,
    //                     user: my_wallet.publicKey,
    //                     // tokenProgram: TOKEN_PROGRAM_ID,
    //                     // associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
    //                     // rent: SYSVAR_RENT_PUBKEY,
    //                     // systemProgram: SystemProgram.programId
    //                 })
    //                 .instruction()
    //         )
    //     tx.feePayer = my_wallet.publicKey
    //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //     // console.log(await connection.simulateTransaction(tx))
    //     const sig = await solanaWeb3.sendAndConfirmTransaction(connection, tx, [my_account], { skipPreflight: true })
    //     console.log("Successfully bought : ", `https://solscan.io/tx/${sig}?cluster=devnet`)

    //     const userBalance = (await connection.getTokenAccountBalance(userAccountToken)).value.uiAmount
    //     const poolBalance = (await connection.getTokenAccountBalance(poolToken)).value.uiAmount

    //     console.log("after creating pool => userBalance:", userBalance)
    //     console.log("after creating pool => poolBalance:", poolBalance)

    // } catch (error) {
    //     console.log("Error in buy transaction", error)
    // }

    console.log("sell")
    // try {
    //     const [curveConfig] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(curveSeed)],
    //         program.programId
    //     )
    //     const [poolSolCurves, bumpcurves] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), curveConfig.toBuffer()],
    //         program.programId
    //     )
    //     console.log("poolSolCurves",poolSolCurves);


    //     const [poolPda] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(POOL_SEED_PREFIX), mint1.toBuffer()],
    //         program.programId
    //     )
        
    //     const poolToken = await getAssociatedTokenAddress(
    //         mint1, poolPda, true
    //     )
    //     const userAccountToken = await getAssociatedTokenAddress(
    //         mint1, my_wallet.publicKey
    //     )
    //     const [poolSolVault, bump] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SOL_VAULT_PREFIX), mint1.toBuffer()],
    //         program.programId
    //     )
    //     const tx = new solanaWeb3.Transaction()
    //         .add(
    //             // solanaWeb3.ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 }),
    //             // solanaWeb3.ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 200_000 }),
    //             await program.methods
    //                 .sell(new anchor.BN(Number(800000000 * 10**9).toString()), bump)
    //                 .accounts({
    //                     pool: poolPda,
    //                     tokenMint: mint1,
    //                     poolSolVault,
    //                     poolTokenAccount: poolToken,
    //                     userTokenAccount: userAccountToken,
    //                     dexConfigurationAccount: curveConfig,
    //                     poolSolCurves: poolSolCurves,
    //                     user: my_wallet.publicKey,
    //                     // tokenProgram: TOKEN_PROGRAM_ID,
    //                     // associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
    //                     // rent: SYSVAR_RENT_PUBKEY,
    //                     // systemProgram: SystemProgram.programId
    //                 })
    //                 .instruction()
    //         )
    //     tx.feePayer = my_wallet.publicKey
    //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //     // console.log(await connection.simulateTransaction(tx)) 
    //     const sig = await solanaWeb3.sendAndConfirmTransaction(connection, tx, [my_account], { skipPreflight: true })
    //     console.log("Successfully Sold : ", `https://solscan.io/tx/${sig}?cluster=devnet`)

    //     const userBalance = (await connection.getTokenAccountBalance(userAccountToken)).value.uiAmount
    //     const poolBalance = (await connection.getTokenAccountBalance(poolToken)).value.uiAmount

    //     console.log("after creating pool => userBalance:", userBalance)
    //     console.log("after creating pool => poolBalance:", poolBalance)

    // } catch (error) {
    //     console.log("Error in sell transaction", error)
    // }

    console.log("claim")
    // try {
    //     const [curveConfig] = solanaWeb3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(curveSeed)],
    //         program.programId
    //     )
    //     console.log("curveConfig",curveConfig);
    //     const [poolSolCurves, bump] = solanaWeb3.PublicKey.findProgramAddressSync(
    //                     [Buffer.from(SOL_VAULT_PREFIX), curveConfig.toBuffer()],
    //                     program.programId
    //             )
    //     console.log("poolSolCurves")
    //     const tx = new solanaWeb3.Transaction()
    //         .add(
            
               
    //             await program.methods
    //                 .claim(bump)
    //                 .accounts({
    //                     dexConfigurationAccount: curveConfig,
    //                     poolSolCurves: poolSolCurves,
    //                     user: my_wallet.publicKey
    //                     // rent: solanaWeb3.SYSVAR_RENT_PUBKEY,
    //                     // systemProgram: anchor.web3.SystemProgram.programId
    //                     // tokenProgram: TOKEN_PROGRAM_ID,
    //                     // associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
    //                     // systemProgram: SystemProgram.programId
    //                 })
    //                 .instruction()
    //         )
    //     tx.feePayer = my_wallet.publicKey
    //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    //     // console.log(await connection.simulateTransaction(tx)) 
    //     const sig = await solanaWeb3.sendAndConfirmTransaction(connection, tx, [my_account], { skipPreflight: true })
    //     console.log("Successfully Sold : ", `https://solscan.io/tx/${sig}?cluster=devnet`)

    // //     const userBalance = (await connection.getTokenAccountBalance(userAta1)).value.uiAmount
    // //     const poolBalance = (await connection.getTokenAccountBalance(poolToken)).value.uiAmount

    // //     console.log("after creating pool => userBalance:", userBalance)
    // //     console.log("after creating pool => poolBalance:", poolBalance)

    // } catch (error) {
    //     console.log("Error in sell transaction", error)
    // }
}

example()