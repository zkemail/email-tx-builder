const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
import * as relayerUtils from "@zk-email/relayer-utils";
import { genEmailCircuitInput } from "../helpers/email_auth";
import { readFileSync } from "fs";

const option = {
  include: path.join(__dirname, "../../../node_modules"),
  output: path.join(__dirname, "../build"),
  recompile: false,
};

jest.setTimeout(1440000);
describe("Email Auth Public Key", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(
      path.join(__dirname, "../src/email_auth_publickey.circom"),
      option
    );
  });

  it("Extract public key from email", async () => {
    const emailName = "x_used";
    const emailFilePath = path.join(__dirname, `./emails/${emailName}.eml`);
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
        shaPrecomputeSelector: "</html>",
      }
    );

    // Write circuit inputs to a JSON file for debugging
    const outputPath = path.join(
      __dirname,
      `./emails/${emailName}_circuit_inputs_debug.json`
    );
    const circuitInputsJson = JSON.stringify(
      circuitInputs,
      (key, value) => {
        // Convert BigInt values to strings for JSON serialization
        if (typeof value === "bigint") {
          return value.toString();
        }
        return value;
      },
      2
    );
    require("fs").writeFileSync(outputPath, circuitInputsJson);
    console.log(`Circuit inputs written to: ${outputPath}`);

    const witness = await circuit.calculateWitness(circuitInputs);
    // Write witness to a JSON file for debugging
    const witnessPath = path.join(
      __dirname,
      `./emails/${emailName}_witness_debug.json`
    );
    const witnessJson = JSON.stringify(
      witness,
      (key, value) => {
        // Convert BigInt values to strings for JSON serialization
        if (typeof value === "bigint") {
          return value.toString();
        }
        return value;
      },
      2
    );
    require("fs").writeFileSync(witnessPath, witnessJson);
    console.log(`Witness written to: ${witnessPath}`);
    await circuit.checkConstraints(witness);
  });
});
