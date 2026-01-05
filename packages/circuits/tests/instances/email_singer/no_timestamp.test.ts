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
describe("Email Auth Production - No Timestamp", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(
      circuitsPath("src", "instances", "email_signer", "no_timestamp.circom"),
      option
    );
  });

  it("Verify a production email for recovery sent from outlook pc with the English setting", async () => {
    const emailFilePath = emailFixture("recovery_outlook_english_pc.eml");

    const emailRaw = readFileSync(emailFilePath, "utf8");
    const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    const accountCode =
      "0x2caf991aa705cbc3fcdba1e42353c8245464394706444c57bf221eab44864ab2";

    const circuitInputs = await genEmailCircuitInput(
      emailFilePath,
      accountCode,
      {
        maxHeaderLength: 1024,
        maxBodyLength: 1024,
        ignoreBodyHashCheck: false,
        shaPrecomputeSelector,
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
      "Accept guardian request for 0xEE8Df3BA718C0cC420698259Ee58A83295C353DC";
    const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    const maskedCommandFields =
      await relayerUtils.bytesToFields(paddedMaskedCommand);
    for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
      expect(BigInt(maskedCommandFields[idx])).toEqual(
        witness[1 + domainFields.length + 3 + idx]
      );
    }

    const fromAddr = "thezdev1@outlook.com";
    const accountSalt = await relayerUtils.generateAccountSalt(
      fromAddr,
      accountCode
    );
    expect(BigInt(accountSalt)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    );

    expect(BigInt(1)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length + 1]
    );
  });

  it("Should verify emails with timestamp but output timestamp to 0", async () => {
    const emailFilePath = emailFixture("recovery_gmail_from_apple_mail.eml");

    const emailRaw = readFileSync(emailFilePath, "utf8");
    const parsedEmail = await relayerUtils.parseEmail(emailRaw);
    const accountCode =
      "0x1162ebff40918afe5305e68396f0283eb675901d0387f97d21928d423aaa0b20";

    const circuitInputs = await genEmailCircuitInput(
      emailFilePath,
      accountCode,
      {
        maxHeaderLength: 1024,
        maxBodyLength: 1024,
        ignoreBodyHashCheck: false,
        shaPrecomputeSelector,
      }
    );

    const witness = await circuit.calculateWitness(circuitInputs);
    await circuit.checkConstraints(witness);

    const domainName = "gmail.com";
    const paddedDomain = relayerUtils.padString(domainName, 255);
    const domainFields = await relayerUtils.bytesToFields(paddedDomain);
    for (let idx = 0; idx < domainFields.length; ++idx) {
      expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
    }

    const expectedPubKeyHash = await relayerUtils.publicKeyHash(
      parsedEmail.publicKey
    );
    expect(BigInt(expectedPubKeyHash)).toEqual(
      witness[1 + domainFields.length]
    );

    const expectedEmailNullifier = await relayerUtils.emailNullifier(
      parsedEmail.signature
    );
    expect(BigInt(expectedEmailNullifier)).toEqual(
      witness[1 + domainFields.length + 1]
    );

    const timestamp = BigInt(0);
    expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);

    const maskedCommand =
      "Accept guardian request for 0x952541bDfe8aae3805D5b9A37D5Ae5e1EE68346f";
    const paddedMaskedCommand = relayerUtils.padString(maskedCommand, 605);
    const maskedCommandFields =
      await relayerUtils.bytesToFields(paddedMaskedCommand);
    for (let idx = 0; idx < maskedCommandFields.length; ++idx) {
      expect(BigInt(maskedCommandFields[idx])).toEqual(
        witness[1 + domainFields.length + 3 + idx]
      );
    }

    const fromAddr = "suegamisora@gmail.com";
    const accountSalt = await relayerUtils.generateAccountSalt(
      fromAddr,
      accountCode
    );
    expect(BigInt(accountSalt)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length]
    );

    expect(BigInt(1)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length + 1]
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

    expect(BigInt(1)).toEqual(
      witness[1 + domainFields.length + 3 + maskedCommandFields.length + 1]
    );
  });
});
