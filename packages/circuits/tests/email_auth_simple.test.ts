const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";

const option = {
  include: path.join(__dirname, "../../../node_modules"),
  output: path.join(__dirname, "../build"),
  recompile: true,
};

jest.setTimeout(1440000);
describe("Email Auth Simple", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(
      path.join(__dirname, "../src/email_auth_simple.circom"),
      option
    );
  });

  it("Simple test", async () => {
    const input = 1n;
    const witness = await circuit.calculateWitness({ i: input });
    await circuit.checkConstraints(witness);
    expect(witness[0]).toEqual(input);
  });
});
