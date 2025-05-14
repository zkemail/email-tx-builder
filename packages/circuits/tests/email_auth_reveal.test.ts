/// <reference types="node" />
// @ts-ignore
const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import path from 'path';
import relayerUtils from "@zk-email/relayer-utils";
import { genEmailCircuitInput } from "../helpers/email_auth";
import { readFileSync } from 'fs';

const option = {
    include: path.join(__dirname, "../../../node_modules"),
    output: path.join(__dirname, "../build"),
    recompile: false,
};
const shaPrecomputeSelector = '(<div id=3D\"[^\"]*zkemail[^\"]*\"[^>]*>)';

jest.setTimeout(1440000);
describe("Email Auth Production - Reveal Public Key and From Address", () => {
    let circuit: any;
    beforeAll(async () => {
        circuit = await wasm_tester(
            path.join(
                __dirname,
                "../src/email_auth_reveal.circom"
            ),
            option
        );
    });

    it("should verify and reveal the public key and from address", async () => {
        const emailFilePath = path.join(
            __dirname,
            "./emails/recovery_outlook_english_pc.eml"
        );

        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await relayerUtils.parseEmail(emailRaw);
        const accountCode =
            "0x2caf991aa705cbc3fcdba1e42353c8245464394706444c57bf221eab44864ab2";

        const circuitInputs =
            await genEmailCircuitInput(emailFilePath, accountCode, {
                maxHeaderLength: 1024,
                maxBodyLength: 1024,
                ignoreBodyHashCheck: false,
                shaPrecomputeSelector
            });

        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);

        // check domain name
        const domainName = "outlook.com";
        const paddedDomain = relayerUtils.padString(domainName, 255);
        const domainFields = await relayerUtils.bytesToFields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }

        // check public key hash
        const expectedPubKeyHash = await relayerUtils.publicKeyHash(
            parsedEmail.publicKey
        );
        expect(BigInt(expectedPubKeyHash)).toEqual(
            witness[1 + domainFields.length]
        );

        // check email nullifier
        const expectedEmailNullifier = await relayerUtils.emailNullifier(
            parsedEmail.signature
        );
        expect(BigInt(expectedEmailNullifier)).toEqual(
            witness[1 + domainFields.length + 1]
        );

        // check timestamp - it is 0 since we don't have a signed timestamp in the email
        const timestamp = BigInt(0);
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);

        // check masked command
        const maskedCommand = "Accept guardian request for 0xEE8Df3BA718C0cC420698259Ee58A83295C353DC";
        const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
        const maskedCommandFields =
            await relayerUtils.bytesToFields(paddedMaskedCommand);
        for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
            expect(BigInt(maskedCommandFields[idx])).toEqual(
                witness[1 + domainFields.length + 3 + idx]
            );
        }

        // check account salt
        const fromAddr = "thezdev1@outlook.com";
        const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
        expect(BigInt(accountSalt)).toEqual(
            witness[1 + domainFields.length + 3 + maskedCommandFields.length]
        );

        // check is_code_exists
        expect(BigInt(1)).toEqual(
            witness[
            1 + domainFields.length + 3 + maskedCommandFields.length + 1
            ]
        );

        // check public key reveal - next 17 fields are the public key
        for (let idx = 0; idx < 17; ++idx) {
            expect(BigInt(circuitInputs.public_key[idx])).toEqual(
                witness[1 + domainFields.length + 3 + maskedCommandFields.length + 2 + idx]
            );
        }

        // check from address reveal - next 9 fields are the from address
        const expectedFromAddrFields = await relayerUtils.bytesToFields(Buffer.from(fromAddr, 'utf-8'));
        for (let idx = 0; idx < 9; ++idx) {
            const expectedValue = idx < expectedFromAddrFields.length ? BigInt(expectedFromAddrFields[idx]) : BigInt(0);
            expect(expectedValue).toEqual(
                witness[1 + domainFields.length + 3 + maskedCommandFields.length + 2 + 17 + idx]
            );
        }
    });
});
