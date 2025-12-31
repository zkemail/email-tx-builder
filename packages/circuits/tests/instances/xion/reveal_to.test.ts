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
  recompile: false,
};
const shaPrecomputeSelector = '(<div id=3D\"[^\"]*zkemail[^\"]*\"[^>]*>)';

jest.setTimeout(1440000);
describe("Email Auth Production - Reveal To Address", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(
      circuitsPath("src", "instances", "xion", "reveal_to.circom"),
      option
    );
  });

  it("Verify a production email for recovery sent from icloud pc with the English setting", async () => {
    const emailFilePath = emailFixture("recovery_icloud_english_pc.eml");

    const emailRaw = readFileSync(emailFilePath, "utf8");
    const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    const accountCode =
      "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";

    const circuitInputs = await genEmailCircuitInput(
      emailFilePath,
      accountCode,
      {
        maxHeaderLength: 1024,
        maxBodyLength: 1024,
        ignoreBodyHashCheck: false,
        shaPrecomputeSelector,
        revealToAddr: true,
      }
    );

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
      "Accept guardian request for 0x04884491560f38342C56E26BDD0fEAbb68E2d2FC";
    const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    const maskedCommandFields =
      await relayerUtils.bytesToFields(paddedMaskedCommand);
    for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
      expect(BigInt(maskedCommandFields[idx])).toEqual(
        witness[1 + domainFields.length + 3 + idx]
      );
    }

    const fromAddr = "suegamisora@icloud.com";
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

    const toAddr = "emaiwallet.alice@gmail.com";
    const paddedToAddr = relayerUtils.padString(toAddr, 255);
    const toAddrFields = await relayerUtils.bytesToFields(paddedToAddr);
    for (let idx = 0; idx < toAddrFields.length; ++idx) {
      expect(BigInt(toAddrFields[idx])).toEqual(
        witness[
          1 + domainFields.length + 3 + maskedCommandFields.length + 2 + idx
        ]
      );
    }
    console.log("toAddr done");
  });
});
