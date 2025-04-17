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
const shaPrecomputeSelector = "<br/>\\[Reply Needed\\]";

jest.setTimeout(1440000);
describe("Email Auth Production - Phone Number", () => {
    let circuit: any;
    beforeAll(async () => {
        circuit = await wasm_tester(
            path.join(
                __dirname,
                "../src/email_auth_clicksend.circom"
            ),
            option
        );
        await init();
    });


    it("should verify a phone number email", async () => {
        const emailFilePath = path.join(
            __dirname,
            "./emails/email_auth_clicksend.eml"
        );

        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await relayerUtils.parseEmail(emailRaw);
        const accountCode =
            "0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7";

        const circuitInputs =
            await genEmailCircuitInput(emailFilePath, accountCode, {
                maxHeaderLength: 1024,
                maxBodyLength: 1536,
                ignoreBodyHashCheck: false,
                shaPrecomputeSelector
            });

        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        console.log("checkConstraints done");

        const domainName = "sms.clicksend.com";
        const paddedDomain = relayerUtils.padString(domainName, 255);
        const domainFields = await relayerUtils.bytesToFields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        console.log("domainFields done");

        console.log("Public Key to check:", parsedEmail.publicKey);

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

        const timestamp = BigInt(1744880540n);
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        console.log("timestamp done");

        const maskedCommand = "signHash 3422187537932466966024063035555602007796308039392799897784341339633298874640";
        const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
        const maskedCommandFields =
            await relayerUtils.bytesToFields(paddedMaskedCommand);
        for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
            expect(BigInt(maskedCommandFields[idx])).toEqual(
                witness[1 + domainFields.length + 3 + idx]
            );
        }

        const fromAddr = "+4917663326278@sms.clicksend.com";
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
