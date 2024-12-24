const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
// const relayerUtils = require("@zk-email/relayer-utils");
import * as relayerUtils from "@zk-email/relayer-utils";
import { genEmailCircuitInput } from "../helpers/email_auth";
import { readFileSync } from "fs";
import { init } from "./wasm_init";

const option = {
    include: path.join(__dirname, "../../../node_modules"),
    output: path.join(__dirname, "../build"),
    recompile: true,
};
const shaPrecomputeSelector = '\\\[zkemail-begin\\\]';

jest.setTimeout(1440000);
describe("Email Auth Production", () => {
    let circuit;
    beforeAll(async () => {
        circuit = await wasm_tester(
            path.join(
                __dirname,
                "../src/email_auth.circom"
            ),
            option
        );
        await init();
    });

    // it("Verify a production email for recovery sent from mobile with the English setting", async () => {
    //     const emailFilePath = path.join(
    //         __dirname,
    //         "./emails/recovery_gmail_english_mobile.eml"
    //     );

    //     const emailRaw = readFileSync(emailFilePath, "utf8");
    //     const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    //     const accountCode =
    //         "0x276695464b95b7401535c10a28f804e9c158732fb9e5d974325b498e05cafe7e";

    //     const circuitInputs =
    //         await genEmailCircuitInput(emailFilePath, accountCode, {
    //             maxHeaderLength: 1024,
    //             maxBodyLength: 1024,
    //             ignoreBodyHashCheck: false,
    //             shaPrecomputeSelector
    //         });
    //     console.log(circuitInputs);
    //     const witness = await circuit.calculateWitness(circuitInputs);
    //     await circuit.checkConstraints(witness);

    //     const domainName = "gmail.com";
    //     const paddedDomain = relayerUtils.padString(domainName, 255);
    //     const domainFields = await relayerUtils.bytesToFields(paddedDomain);
    //     for (let idx = 0; idx < domainFields.length; ++idx) {
    //         expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
    //     }

    //     const expectedPubKeyHash = await relayerUtils.publicKeyHash(
    //         parsedEmail.publicKey
    //     );
    //     expect(BigInt(expectedPubKeyHash)).toEqual(
    //         witness[1 + domainFields.length]
    //     );

    //     const expectedEmailNullifier = await relayerUtils.emailNullifier(
    //         parsedEmail.signature
    //     );
    //     expect(BigInt(expectedEmailNullifier)).toEqual(
    //         witness[1 + domainFields.length + 1]
    //     );

    //     const timestamp = BigInt(1733834024);
    //     expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);

    //     const maskedCommand = "Accept guardian request for 0xAc09b60d0BD4370545F1f56dB21cd33606c1F201";
    //     const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    //     const maskedCommandFields =
    //         await relayerUtils.bytesToFields(paddedMaskedCommand);
    //     for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
    //         expect(BigInt(maskedCommandFields[idx])).toEqual(
    //             witness[1 + domainFields.length + 3 + idx]
    //         );
    //     }

    //     const fromAddr = "clavertest1@gmail.com";
    //     const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
    //     expect(BigInt(accountSalt)).toEqual(
    //         witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    //     );

    //     expect(BigInt(1)).toEqual(
    //         witness[
    //         1 + domainFields.length + 3 + maskedCommandFields.length + 1
    //         ]
    //     );
    // });


    // it("Verify a production email for recovery sent from PC with the English setting", async () => {
    //     const emailFilePath = path.join(
    //         __dirname,
    //         "./emails/recovery_gmail_english_pc.eml"
    //     );

    //     const emailRaw = readFileSync(emailFilePath, "utf8");
    //     const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    //     const accountCode =
    //         "0x2cf32a52285eb1f62fabeeb143e91314d1d2f8df39e572e26f19d6c9fd301a28";

    //     const circuitInputs =
    //         await genEmailCircuitInput(emailFilePath, accountCode, {
    //             maxHeaderLength: 1024,
    //             maxBodyLength: 1024,
    //             ignoreBodyHashCheck: false,
    //             shaPrecomputeSelector
    //         });
    //     console.log(circuitInputs);
    //     const witness = await circuit.calculateWitness(circuitInputs);
    //     await circuit.checkConstraints(witness);

    //     const domainName = "gmail.com";
    //     const paddedDomain = relayerUtils.padString(domainName, 255);
    //     const domainFields = await relayerUtils.bytesToFields(paddedDomain);
    //     for (let idx = 0; idx < domainFields.length; ++idx) {
    //         expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
    //     }

    //     const expectedPubKeyHash = await relayerUtils.publicKeyHash(
    //         parsedEmail.publicKey
    //     );
    //     expect(BigInt(expectedPubKeyHash)).toEqual(
    //         witness[1 + domainFields.length]
    //     );

    //     const expectedEmailNullifier = await relayerUtils.emailNullifier(
    //         parsedEmail.signature
    //     );
    //     expect(BigInt(expectedEmailNullifier)).toEqual(
    //         witness[1 + domainFields.length + 1]
    //     );

    //     const timestamp = BigInt(1733832799);
    //     expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);

    //     const maskedCommand = "Accept guardian request for 0xAc09b60d0BD4370545F1f56dB21cd33606c1F201";
    //     const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    //     const maskedCommandFields =
    //         await relayerUtils.bytesToFields(paddedMaskedCommand);
    //     for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
    //         expect(BigInt(maskedCommandFields[idx])).toEqual(
    //             witness[1 + domainFields.length + 3 + idx]
    //         );
    //     }

    //     const fromAddr = "clavertest1@gmail.com";
    //     const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
    //     expect(BigInt(accountSalt)).toEqual(
    //         witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    //     );

    //     expect(BigInt(1)).toEqual(
    //         witness[
    //         1 + domainFields.length + 3 + maskedCommandFields.length + 1
    //         ]
    //     );
    // });

    // it("Verify a production email for recovery sent from mobile with the Turkish setting", async () => {
    //     const emailFilePath = path.join(
    //         __dirname,
    //         "./emails/recovery_gmail_turkish_mobile.eml"
    //     );

    //     const emailRaw = readFileSync(emailFilePath, "utf8");
    //     const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    //     const accountCode =
    //         "0x207c337c09f4196dac142567a9e0ddc955b89b42ce30aae71806a6a5c8626570";

    //     const circuitInputs =
    //         await genEmailCircuitInput(emailFilePath, accountCode, {
    //             maxHeaderLength: 1024,
    //             maxBodyLength: 1024,
    //             ignoreBodyHashCheck: false,
    //             shaPrecomputeSelector
    //         });
    //     console.log(circuitInputs);
    //     const witness = await circuit.calculateWitness(circuitInputs);
    //     await circuit.checkConstraints(witness);

    //     const domainName = "gmail.com";
    //     const paddedDomain = relayerUtils.padString(domainName, 255);
    //     const domainFields = await relayerUtils.bytesToFields(paddedDomain);
    //     for (let idx = 0; idx < domainFields.length; ++idx) {
    //         expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
    //     }

    //     const expectedPubKeyHash = await relayerUtils.publicKeyHash(
    //         parsedEmail.publicKey
    //     );
    //     expect(BigInt(expectedPubKeyHash)).toEqual(
    //         witness[1 + domainFields.length]
    //     );

    //     const expectedEmailNullifier = await relayerUtils.emailNullifier(
    //         parsedEmail.signature
    //     );
    //     expect(BigInt(expectedEmailNullifier)).toEqual(
    //         witness[1 + domainFields.length + 1]
    //     );

    //     const timestamp = BigInt(1733834333);
    //     expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);

    //     const maskedCommand = "Accept guardian request for 0xAc09b60d0BD4370545F1f56dB21cd33606c1F201";
    //     const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    //     const maskedCommandFields =
    //         await relayerUtils.bytesToFields(paddedMaskedCommand);
    //     for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
    //         expect(BigInt(maskedCommandFields[idx])).toEqual(
    //             witness[1 + domainFields.length + 3 + idx]
    //         );
    //     }

    //     const fromAddr = "clavertest1@gmail.com";
    //     const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
    //     expect(BigInt(accountSalt)).toEqual(
    //         witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    //     );

    //     expect(BigInt(1)).toEqual(
    //         witness[
    //         1 + domainFields.length + 3 + maskedCommandFields.length + 1
    //         ]
    //     );
    // });

    // it("Verify a production email for recovery sent from PC with the Turkish setting", async () => {
    //     const emailFilePath = path.join(
    //         __dirname,
    //         "./emails/recovery_gmail_turkish_pc.eml"
    //     );

    //     const emailRaw = readFileSync(emailFilePath, "utf8");
    //     const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    //     const accountCode =
    //         "0x2ea2aea6391028aa5e1c5a8b1de277ba6b2d04b1a2da0bef66d3311ab2679347";

    //     const circuitInputs =
    //         await genEmailCircuitInput(emailFilePath, accountCode, {
    //             maxHeaderLength: 1024,
    //             maxBodyLength: 1024,
    //             ignoreBodyHashCheck: false,
    //             shaPrecomputeSelector
    //         });
    //     console.log(circuitInputs);
    //     const witness = await circuit.calculateWitness(circuitInputs);
    //     await circuit.checkConstraints(witness);

    //     const domainName = "gmail.com";
    //     const paddedDomain = relayerUtils.padString(domainName, 255);
    //     const domainFields = await relayerUtils.bytesToFields(paddedDomain);
    //     for (let idx = 0; idx < domainFields.length; ++idx) {
    //         expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
    //     }

    //     const expectedPubKeyHash = await relayerUtils.publicKeyHash(
    //         parsedEmail.publicKey
    //     );
    //     expect(BigInt(expectedPubKeyHash)).toEqual(
    //         witness[1 + domainFields.length]
    //     );

    //     const expectedEmailNullifier = await relayerUtils.emailNullifier(
    //         parsedEmail.signature
    //     );
    //     expect(BigInt(expectedEmailNullifier)).toEqual(
    //         witness[1 + domainFields.length + 1]
    //     );

    //     const timestamp = BigInt(1733834168);
    //     expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);

    //     const maskedCommand = "Accept guardian request for 0xAc09b60d0BD4370545F1f56dB21cd33606c1F201";
    //     const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    //     const maskedCommandFields =
    //         await relayerUtils.bytesToFields(paddedMaskedCommand);
    //     for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
    //         expect(BigInt(maskedCommandFields[idx])).toEqual(
    //             witness[1 + domainFields.length + 3 + idx]
    //         );
    //     }

    //     const fromAddr = "clavertest1@gmail.com";
    //     const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
    //     expect(BigInt(accountSalt)).toEqual(
    //         witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    //     );

    //     expect(BigInt(1)).toEqual(
    //         witness[
    //         1 + domainFields.length + 3 + maskedCommandFields.length + 1
    //         ]
    //     );
    // });

    it("Verify a production email for recovery sent from icloud pc", async () => {
        const emailFilePath = path.join(
            __dirname,
            "./emails/recovery_icloud_pc.eml"
        );

        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await relayerUtils.parseEmail(emailRaw);
        console.log(parsedEmail);
        const accountCode =
            "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";

        const circuitInputs =
            await genEmailCircuitInput(emailFilePath, accountCode, {
                maxHeaderLength: 1024,
                maxBodyLength: 1024,
                ignoreBodyHashCheck: false,
                shaPrecomputeSelector,
            });
        console.log(circuitInputs);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        console.log("checkConstraints done");

        const domainName = "icloud.com";
        const paddedDomain = relayerUtils.padString(domainName, 255);
        const domainFields = await relayerUtils.bytesToFields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        console.log("domainFields done");

        const expectedPubKeyHash = await relayerUtils.publicKeyHash(
            parsedEmail.publicKey
        );
        expect(BigInt(expectedPubKeyHash)).toEqual(
            witness[1 + domainFields.length]
        );
        console.log("expectedPubKeyHash done");

        const expectedEmailNullifier = await relayerUtils.emailNullifier(
            parsedEmail.signature
        );
        expect(BigInt(expectedEmailNullifier)).toEqual(
            witness[1 + domainFields.length + 1]
        );
        console.log("expectedEmailNullifier done");

        const timestamp = BigInt(1734913850);
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        console.log("timestamp done");

        const maskedCommand = "Accept guardian request for 0x04884491560f38342C56E26BDD0fEAbb68E2d2FC";
        const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
        const maskedCommandFields =
            await relayerUtils.bytesToFields(paddedMaskedCommand);
        for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
            expect(BigInt(maskedCommandFields[idx])).toEqual(
                witness[1 + domainFields.length + 3 + idx]
            );
        }

        const fromAddr = "suegamisora@icloud.com";
        const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
        expect(BigInt(accountSalt)).toEqual(
            witness[1 + domainFields.length + 3 + maskedCommandFields.length]
        );

        expect(BigInt(1)).toEqual(
            witness[
            1 + domainFields.length + 3 + maskedCommandFields.length + 1
            ]
        );
    });

    /// The outlook email fails at EmailAuth_723 line: 118. The circuit needs to put a dummy 1 in similar to `replaced_code_regex_reveal`. 
    it("Verify a production email for recovery sent from outlook pc", async () => {
        const emailFilePath = path.join(
            __dirname,
            "./emails/recovery_outlook_pc.eml"
        );

        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await relayerUtils.parseEmail(emailRaw);
        console.log(parsedEmail);
        const accountCode =
            "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";

        const circuitInputs =
            await genEmailCircuitInput(emailFilePath, accountCode, {
                maxHeaderLength: 1024,
                maxBodyLength: 1024,
                ignoreBodyHashCheck: false,
                shaPrecomputeSelector,
            });
        console.log(circuitInputs);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        console.log("checkConstraints done");

        const domainName = "outlook.com";
        const paddedDomain = relayerUtils.padString(domainName, 255);
        const domainFields = await relayerUtils.bytesToFields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        console.log("domainFields done");

        const expectedPubKeyHash = await relayerUtils.publicKeyHash(
            parsedEmail.publicKey
        );
        expect(BigInt(expectedPubKeyHash)).toEqual(
            witness[1 + domainFields.length]
        );
        console.log("expectedPubKeyHash done");

        const expectedEmailNullifier = await relayerUtils.emailNullifier(
            parsedEmail.signature
        );
        expect(BigInt(expectedEmailNullifier)).toEqual(
            witness[1 + domainFields.length + 1]
        );
        console.log("expectedEmailNullifier done");

        const timestamp = BigInt(0);
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        console.log("timestamp done");

        const maskedCommand = "Accept guardian request for 0x04884491560f38342C56E26BDD0fEAbb68E2d2FC";
        const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
        const maskedCommandFields =
            await relayerUtils.bytesToFields(paddedMaskedCommand);
        for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
            expect(BigInt(maskedCommandFields[idx])).toEqual(
                witness[1 + domainFields.length + 3 + idx]
            );
        }

        const fromAddr = "suegamisora@outlook.com";
        const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
        expect(BigInt(accountSalt)).toEqual(
            witness[1 + domainFields.length + 3 + maskedCommandFields.length]
        );

        expect(BigInt(1)).toEqual(
            witness[
            1 + domainFields.length + 3 + maskedCommandFields.length + 1
            ]
        );
    });


});


