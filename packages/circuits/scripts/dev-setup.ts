/**
 *
 * This script is for generating zKey and Verification Key for the circuit.
 * Running this will download the phase1 file (if not already present),
 * generate the zKey, and also export solidity and json verification keys.
 *
 * Running this will overwrite any existing zKey and verification key files.
 *
 */

// @ts-ignore
import { zKey } from "snarkjs";
import https from "https";
import fs from "fs";
import path from "path";
import { program } from "commander";
import crypto from "crypto";

program
  .requiredOption(
    "--output <string>",
    "Path to the directory storing output files"
  )
  .option("--silent", "No console logs")
  .option("--legacy", "Use a legacy circuit")
  .option("--entropy <hex>", "Entropy for contribution (hex or string)")
  .option("--beacon <hex>", "Beacon for finalization (hex string)")
  .option("--name <string>", "Contributor name for ceremony")
  .option(
    "-c, --circuit <names>",
    "comma-separated circuit base names to generate keys for (without .circom)"
  );

program.parse();
const args = program.opts();

function log(...message: any) {
  if (!args.silent) {
    console.log(...message);
  }
}

let { ZKEY_ENTROPY, ZKEY_BEACON } = process.env;
if (args.entropy) {
  ZKEY_ENTROPY = String(args.entropy);
}
if (ZKEY_ENTROPY == null) {
  ZKEY_ENTROPY = "dev";
}
function deriveBeaconHex(input: string): string {
  let raw = input.trim();
  if (raw.startsWith("0x")) raw = raw.slice(2);
  if (/^[0-9a-fA-F]+$/.test(raw)) {
    return (raw.length % 2 === 0 ? raw : "0" + raw).toLowerCase();
  }
  return crypto.createHash("sha256").update(raw, "utf8").digest("hex");
}

if (args.beacon) {
  ZKEY_BEACON = deriveBeaconHex(String(args.beacon));
}
if (ZKEY_BEACON == null) {
  ZKEY_BEACON =
    "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
}

let phase1Url =
  "https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_23.ptau";
if (args.legacy) {
  phase1Url =
    "https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_22.ptau";
}
// const buildDir = path.join(__dirname, "../build");
// const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_21.ptau");
// const r1cPath = path.join(buildDir, "wallet.r1cs");
const solidityTemplate = path.join(
  require.resolve("snarkjs"),
  "../../templates/verifier_groth16.sol.ejs"
);

// Output paths
// const zKeyPath = path.join(buildDir, "wallet.zkey");
// const vKeyPath = path.join(buildDir, "vkey.json");
// const solidityVerifierPath = path.join(buildDir, "verifier.sol");

// async function askBeacon() {
//   if (!ZKEY_BEACON) {
//     ZKEY_BEACON = await new Promise((resolve) => {
//       const readline = require("readline").createInterface({
//         input: process.stdin,
//         output: process.stdout,
//       });
//       readline.question(
//         "Enter Beacon (hex string) to apply: ",
//         (entropy: string) => {
//           readline.close();
//           resolve(entropy);
//         }
//       );
//     });
//   }
// }

async function downloadPhase1(phase1Path: string) {
  if (!fs.existsSync(phase1Path)) {
    log(`✘ Phase 1 not found at ${phase1Path}`);
    log(`䷢ Downloading Phase 1`);

    const phase1File = fs.createWriteStream(phase1Path);

    return new Promise((resolve, reject) => {
      https
        .get(phase1Url, (response) => {
          response.pipe(phase1File);
          phase1File.on("finish", () => {
            phase1File.close();
            resolve(true);
          });
        })
        .on("error", (err) => {
          fs.unlink(phase1Path, () => {});
          reject(err);
        });
    });
  }
}

async function generateKeys(
  phase1Path: string,
  r1cPath: string,
  zKeyPath: string,
  vKeyPath: string,
  solidityVerifierPath: string
) {
  log(`✓ Generating ZKey for ${r1cPath}`);
  await zKey.newZKey(r1cPath, phase1Path, zKeyPath + ".step1", console);
  log("✓ Partial ZKey generated");

  await zKey.contribute(
    zKeyPath + ".step1",
    zKeyPath + ".step2",
    args.name ? String(args.name) : "Contributor 1",
    ZKEY_ENTROPY,
    console
  );
  log("✓ First contribution completed");

  // await askBeacon();
  await zKey.beacon(
    zKeyPath + ".step2",
    zKeyPath,
    "Final Beacon",
    ZKEY_BEACON,
    10,
    console
  );
  log("✓ Beacon applied");

  await zKey.verifyFromR1cs(r1cPath, phase1Path, zKeyPath, console);
  log(`✓ Final ZKey verified - ${zKeyPath}`);

  const vKey = await zKey.exportVerificationKey(zKeyPath, console);
  fs.writeFileSync(vKeyPath, JSON.stringify(vKey, null, 2));
  log(`✓ Verification key exported - ${vKeyPath}`);

  const templates = {
    groth16: fs.readFileSync(solidityTemplate, "utf8"),
  };
  const code = await zKey.exportSolidityVerifier(zKeyPath, templates, console);
  fs.writeFileSync(solidityVerifierPath, code);
  log(`✓ Solidity verifier exported - ${solidityVerifierPath}`);
  fs.rmSync(zKeyPath + ".step1");
  fs.rmSync(zKeyPath + ".step2");
}

async function exec() {
  const buildDir = args.output;

  // if specific circuits requested, handle them first and exit
  const requestedCircuits: string[] | undefined = args.circuit
    ? String(args.circuit)
        .split(",")
        .map((s: string) => s.trim())
        .filter(Boolean)
    : undefined;

  const generateForCircuit = async (baseName: string) => {
    const phase1Path = path.join(
      buildDir,
      args.legacy
        ? "powersOfTau28_hez_final_22.ptau"
        : "powersOfTau28_hez_final_23.ptau"
    );
    await downloadPhase1(phase1Path);
    log("✓ Phase 1:", phase1Path);

    const r1csPath = path.join(buildDir, `${baseName}.r1cs`);
    if (!fs.existsSync(r1csPath)) {
      throw new Error(`${r1csPath} does not exist.`);
    }
    const zkeyPath = path.join(buildDir, `${baseName}.zkey`);
    const vkeyPath = path.join(buildDir, `${baseName}.vkey`);
    const verifierPath = path.join(
      buildDir,
      args.legacy
        ? `Groth16LegacyVerifier_${baseName}.sol`
        : `Groth16Verifier_${baseName}.sol`
    );
    await generateKeys(phase1Path, r1csPath, zkeyPath, vkeyPath, verifierPath);
    log(`✓ Keys generated for ${baseName}`);
  };

  if (requestedCircuits && requestedCircuits.length > 0) {
    for (const baseName of requestedCircuits) {
      await generateForCircuit(baseName);
    }
    return;
  }

  if (args.legacy) {
    const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_22.ptau");

    await downloadPhase1(phase1Path);
    log("✓ Phase 1:", phase1Path);

    const emailAuthR1csPath = path.join(buildDir, "email_auth_legacy.r1cs");
    if (!fs.existsSync(emailAuthR1csPath)) {
      throw new Error(`${emailAuthR1csPath} does not exist.`);
    }
    await generateKeys(
      phase1Path,
      emailAuthR1csPath,
      path.join(buildDir, "email_auth_legacy.zkey"),
      path.join(buildDir, "email_auth_legacy.vkey"),
      path.join(buildDir, "Groth16LegacyVerifier.sol")
    );
    log("✓ Keys for email auth legacy circuit generated");
  } else {
    const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_23.ptau");

    await downloadPhase1(phase1Path);
    log("✓ Phase 1:", phase1Path);

    const emailAuthR1csPath = path.join(buildDir, "email_auth.r1cs");
    if (!fs.existsSync(emailAuthR1csPath)) {
      throw new Error(`${emailAuthR1csPath} does not exist.`);
    }
    await generateKeys(
      phase1Path,
      emailAuthR1csPath,
      path.join(buildDir, "email_auth.zkey"),
      path.join(buildDir, "email_auth.vkey"),
      path.join(buildDir, "Groth16Verifier.sol")
    );
    log("✓ Keys for email auth circuit generated");
  }
}

exec()
  .then(() => {
    process.exit(0);
  })
  .catch((err) => {
    console.log("Error: ", err);
    process.exit(1);
  });
