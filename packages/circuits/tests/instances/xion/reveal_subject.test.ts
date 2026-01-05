const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import path from "path";
import relayerUtils from "@zk-email/relayer-utils";
import { genEmailCircuitInput } from "../../../helpers/email_auth";
import { readFileSync } from "fs";

const circuitsRoot = path.resolve(__dirname, "..", "..", ".."); // packages/circuits
const repoRoot = path.resolve(circuitsRoot, "..", ".."); // repo root
const circuitsPath = (...segments: string[]) =>
  path.join(circuitsRoot, ...segments);
const repoPath = (...segments: string[]) => path.join(repoRoot, ...segments);
const emailFixture = (file: string) =>
  circuitsPath("tests", "fixtures", "emails", file);

const option = {
  include: repoPath("node_modules"),
  output: circuitsPath("build"),
  recompile: true,
};
const shaPrecomputeSelector = '(<div id=3D\"[^\"]*zkemail[^\"]*\"[^>]*>)';

jest.setTimeout(1440000);
describe("Email Auth Production - Reveal Subject", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(
      circuitsPath("src", "instances", "xion", "reveal_subject.circom"),
      option
    );
  });

  it("Verify a production email for xion transaction from outlook", async () => {
    const emailFilePath = emailFixture("xion_outlook.eml");

    const emailRaw = readFileSync(emailFilePath, "utf8");
    const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    const accountCode =
      "0x0a19e429c12bd391a80c7307954ca7789164d790ba23f8466c39fd209aaa0444";

    const circuitInputs = await genEmailCircuitInput(
      emailFilePath,
      accountCode,
      {
        maxHeaderLength: 1024,
        maxBodyLength: 1024,
        ignoreBodyHashCheck: false,
        shaPrecomputeSelector,
        revealSubject: true,
      }
    );

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

    const timestamp = BigInt(0);
    expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
    console.log("timestamp done");

    const maskedCommand =
      "CqIBCp8BChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEn8KP3hpb24xNG43OWVocGZ3aGRoNHN6dWRhZ2Q0bm14N2NsajU3bHk1dTBsenhzNm1nZjVxeTU1a3k5c21zenM0OBIreGlvbjFxYWYyeGZseDVqM2FndGx2cWs1dmhqcGV1aGw2ZzQ1aHhzaHdxahoPCgV1eGlvbhIGMTAwMDAwEmYKTQpDCh0vYWJzdHJhY3RhY2NvdW50LnYxLk5pbFB1YktleRIiCiCs_FzcKXXbesBcb1Daz2b2Pyp75Kcf8Roa2hNAEpSxCxIECgIIARgBEhUKDgoFdXhpb24SBTYwMDAwEICHpw4aBnhpb24tMSAM\r\n";
    const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    const maskedCommandFields =
      await relayerUtils.bytesToFields(paddedMaskedCommand);
    for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
      expect(BigInt(maskedCommandFields[idx])).toEqual(
        witness[1 + domainFields.length + 3 + idx]
      );
    }

    const fromAddr = "kushal.007dum@outlook.com";
    const accountSalt = await relayerUtils.generateAccountSalt(
      fromAddr,
      accountCode
    );
    expect(BigInt(accountSalt)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    );
    console.log("accountSalt done");

    expect(BigInt(1)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length + 1]
    );
    console.log("is code exists done");

    const subject =
      "Re: [Reply Needed] Command Confirmation Required\
 [91s_ixk_t46tng0fedtvdg]";
    const paddedSubject = relayerUtils.padString(subject, 256);
    const subjectFields = await relayerUtils.bytesToFields(paddedSubject);
    for (let idx = 0; idx < subjectFields.length; ++idx) {
      expect(BigInt(subjectFields[idx])).toEqual(
        witness[
          1 + domainFields.length + 3 + maskedCommandFields.length + 2 + idx
        ]
      );
    }
    console.log("subject done");
  });
});
