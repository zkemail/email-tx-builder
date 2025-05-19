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
    recompile: true,
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

        // Define all witness constants
        const domainName = "outlook.com";
        const fromAddr = "thezdev1@outlook.com";
        const maskedCommand = "Accept guardian request for 0xEE8Df3BA718C0cC420698259Ee58A83295C353DC";
        
        // Process domain, masked command, and from address fields
        const paddedDomain = relayerUtils.padString(domainName, 255);
        const domainFields = await relayerUtils.bytesToFields(paddedDomain);
        const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
        const maskedCommandFields = await relayerUtils.bytesToFields(paddedMaskedCommand);
        const expectedFromAddrFields = await relayerUtils.bytesToFields(Buffer.from(fromAddr, 'utf-8'));
        
        // Define all witness index constants
        const WITNESS_OFFSET = 1; // First output element is reserved
        const DOMAIN_FIELDS_COUNT = domainFields.length;
        const MASKED_COMMAND_FIELDS_COUNT = maskedCommandFields.length;
        const PUBLIC_KEY_FIELDS_COUNT = 17;
        const FROM_ADDRESS_FIELDS_COUNT = 9;

        // Define all witness offsets
        const PUBKEY_HASH_OFFSET = WITNESS_OFFSET + DOMAIN_FIELDS_COUNT;
        const EMAIL_NULLIFIER_OFFSET = PUBKEY_HASH_OFFSET + 1;
        const TIMESTAMP_OFFSET = EMAIL_NULLIFIER_OFFSET + 1;
        const MASKED_COMMAND_OFFSET = TIMESTAMP_OFFSET + 1;
        const ACCOUNT_SALT_OFFSET = MASKED_COMMAND_OFFSET + MASKED_COMMAND_FIELDS_COUNT;
        const IS_CODE_EXISTS_OFFSET = ACCOUNT_SALT_OFFSET + 1;
        const PUBLIC_KEY_REVEAL_OFFSET = IS_CODE_EXISTS_OFFSET + 1;
        const FROM_ADDRESS_REVEAL_OFFSET = PUBLIC_KEY_REVEAL_OFFSET + PUBLIC_KEY_FIELDS_COUNT;

        // check domain name
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[WITNESS_OFFSET + idx]);
        }

        // check public key hash
        const expectedPubKeyHash = await relayerUtils.publicKeyHash(
            parsedEmail.publicKey
        );
        expect(BigInt(expectedPubKeyHash)).toEqual(
            witness[PUBKEY_HASH_OFFSET]
        );

        // check email nullifier
        const expectedEmailNullifier = await relayerUtils.emailNullifier(
            parsedEmail.signature
        );
        expect(BigInt(expectedEmailNullifier)).toEqual(
            witness[EMAIL_NULLIFIER_OFFSET]
        );

        // check timestamp - it is 0 since we don't have a signed timestamp in the email
        const timestamp = BigInt(0);
        expect(timestamp).toEqual(witness[TIMESTAMP_OFFSET]);

        // check masked command
        for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
            expect(BigInt(maskedCommandFields[idx])).toEqual(
                witness[MASKED_COMMAND_OFFSET + idx]
            );
        }

        // check account salt
        const accountSalt = await relayerUtils.generateAccountSalt(fromAddr, accountCode);
        expect(BigInt(accountSalt)).toEqual(
            witness[ACCOUNT_SALT_OFFSET]
        );

        // check is_code_exists
        expect(BigInt(1)).toEqual(
            witness[IS_CODE_EXISTS_OFFSET]
        );

        // check public key reveal - next 17 fields are the public key
        for (let idx = 0; idx < PUBLIC_KEY_FIELDS_COUNT; ++idx) {
            expect(BigInt(circuitInputs.public_key[idx])).toEqual(
                witness[PUBLIC_KEY_REVEAL_OFFSET + idx]
            );
        }

        // check from address reveal - next 9 fields are the from address
        for (let idx = 0; idx < FROM_ADDRESS_FIELDS_COUNT; ++idx) {
            const expectedValue = idx < expectedFromAddrFields.length ? BigInt(expectedFromAddrFields[idx]) : BigInt(0);
            expect(expectedValue).toEqual(
                witness[FROM_ADDRESS_REVEAL_OFFSET + idx]
            );
        }
    });
});
