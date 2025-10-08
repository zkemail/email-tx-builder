/**
 * Local BLS12-381 dev setup with Groth16 using snarkjs CLI.
 * - Runs a local Powers of Tau ceremony (power default 23)
 * - Applies a single contribution, verification, beacon, and phase2 preparation
 * - Generates Groth16 proving key (zkey), verification key, and Solidity verifier
 * - Caches intermediate artifacts (ptau and zkey steps) to allow resuming
 *
 * Usage example:
 *   ts-node scripts/dev-setup-bls.ts \
 *     --output ./build \
 *     --circuit email_auth \
 *     --power 23 \
 *     --verbose
 */

import fs from "fs";
import path from "path";
import { spawn } from "child_process";
import crypto from "crypto";
import { program } from "commander";

program
  .requiredOption(
    "--output <string>",
    "Path to the directory storing output files"
  )
  .requiredOption(
    "--circuit <string>",
    "Name of the circuit (without extension)"
  )
  .option(
    "--power <number>",
    "Powers of Tau power (default 23)",
    (v) => parseInt(v, 10),
    23
  )
  .option(
    "--beacon <hex>",
    "32-byte hex for beacon",
    "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
  )
  .option(
    "--beacon-iterations <number>",
    "Beacon iterations power (2^n)",
    (v) => parseInt(v, 10),
    10
  )
  .option(
    "--ptau-dir <string>",
    "Directory to store ptau artifacts",
    "./build/ptau"
  )
  .option("--silent", "Suppress logs")
  .option("--verbose", "Verbose command logging");

program.parse();
const args = program.opts();

function log(...message: any[]) {
  if (!args.silent) console.log(...message);
}

const cwd = process.cwd();
const snarkjsBin = path.join(cwd, "node_modules", ".bin", "snarkjs");

function ensureFile(filePath: string) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`${filePath} does not exist.`);
  }
}

function ensureDir(dirPath: string) {
  if (!fs.existsSync(dirPath)) fs.mkdirSync(dirPath, { recursive: true });
}

function run(
  cmd: string,
  argv: string[],
  opts: { cwd?: string; verboseTag?: string } = {}
): Promise<void> {
  const tag = opts.verboseTag ? `[${opts.verboseTag}]` : "";
  return new Promise((resolve, reject) => {
    if (args.verbose) log(`${tag} $ ${cmd} ${argv.join(" ")}`);
    const child = spawn(cmd, argv, {
      stdio: "pipe",
      shell: false,
      cwd: opts.cwd ?? cwd,
    });
    child.stdout.on("data", (d) => {
      if (!args.silent) process.stdout.write(d);
    });
    child.stderr.on("data", (d) => {
      if (!args.silent) process.stderr.write(d);
    });
    child.on("error", reject);
    child.on("close", (code) => {
      if (code === 0) return resolve();
      reject(new Error(`${cmd} exited with code ${code}`));
    });
  });
}

function randomEntropyLabel(label: string): string {
  const rnd = crypto.randomBytes(16).toString("hex");
  return `${label}-${Date.now()}-${rnd}`;
}

async function runPowersOfTau(ptauDir: string, power: number) {
  ensureDir(ptauDir);
  const curve = "bls12-381";
  const base = `pot${power}`;

  const ptau_0000 = path.join(ptauDir, `${base}_0000.ptau`);
  const ptau_0001 = path.join(ptauDir, `${base}_0001.ptau`);
  const ptau_beacon = path.join(ptauDir, `${base}_beacon.ptau`);
  const ptau_final = path.join(ptauDir, `${base}_final.ptau`);

  // 1) new
  log("▶ Starting Powers of Tau (BLS12-381)");
  if (!fs.existsSync(ptau_0000)) {
    await run(
      snarkjsBin,
      ["powersoftau", "new", curve, String(power), ptau_0000, "-v"],
      { verboseTag: "ptau:new" }
    );
  } else {
    log(`↺ Skipping ptau new (exists): ${ptau_0000}`);
  }

  // 2) contribute #1
  log("▶ Contribution #1");
  if (!fs.existsSync(ptau_0001)) {
    await run(
      snarkjsBin,
      [
        "powersoftau",
        "contribute",
        ptau_0000,
        ptau_0001,
        `--name=${randomEntropyLabel("contrib1")}`,
        "-v",
        `-e=${randomEntropyLabel("entropy1")}`,
      ],
      { verboseTag: "ptau:contrib1" }
    );
  } else {
    log(`↺ Skipping ptau contribution (exists): ${ptau_0001}`);
  }

  // 5) verify so far
  log("▶ Verifying transcript so far");
  if (fs.existsSync(ptau_0001)) {
    await run(snarkjsBin, ["powersoftau", "verify", ptau_0001], {
      verboseTag: "ptau:verify1",
    });
  }

  // 6) beacon
  log("▶ Applying random beacon");
  if (!fs.existsSync(ptau_beacon)) {
    await run(
      snarkjsBin,
      [
        "powersoftau",
        "beacon",
        ptau_0001,
        ptau_beacon,
        args.beacon,
        String(args.beaconIterations),
        `-n=${randomEntropyLabel("FinalBeacon")}`,
      ],
      { verboseTag: "ptau:beacon" }
    );
  } else {
    log(`↺ Skipping ptau beacon (exists): ${ptau_beacon}`);
  }

  // 7) prepare phase2
  log("▶ Preparing phase2");
  if (!fs.existsSync(ptau_final)) {
    await run(
      snarkjsBin,
      ["powersoftau", "prepare", "phase2", ptau_beacon, ptau_final, "-v"],
      { verboseTag: "ptau:phase2" }
    );
  } else {
    log(`↺ Skipping phase2 prepare (exists): ${ptau_final}`);
  }

  // 8) verify final
  log("▶ Verifying final ptau");
  if (fs.existsSync(ptau_final)) {
    await run(snarkjsBin, ["powersoftau", "verify", ptau_final], {
      verboseTag: "ptau:verify2",
    });
  }

  return { ptauFinalPath: ptau_final };
}

async function runGroth16Setup(
  ptauFinalPath: string,
  outputDir: string,
  circuitName: string
) {
  const r1csPath = path.join(outputDir, `${circuitName}.r1cs`);
  ensureFile(r1csPath);

  const zkeyStep1 = path.join(outputDir, `${circuitName}.zkey.step1`);
  const zkeyStep2 = path.join(outputDir, `${circuitName}.zkey.step2`);
  const zkeyPath = path.join(outputDir, `${circuitName}.zkey`);
  const vkeyPath = path.join(outputDir, `${circuitName}.vkey`);

  log(`▶ Generating Groth16 zkey for ${circuitName}`);
  if (!fs.existsSync(zkeyStep1)) {
    await run(
      snarkjsBin,
      ["groth16", "setup", r1csPath, ptauFinalPath, zkeyStep1],
      { verboseTag: "groth16:setup" }
    );
  } else {
    log(`↺ Skipping groth16 setup (exists): ${zkeyStep1}`);
  }

  if (!fs.existsSync(zkeyStep2)) {
    log("▶ Contributing to zkey");
    await run(
      snarkjsBin,
      [
        "zkey",
        "contribute",
        zkeyStep1,
        zkeyStep2,
        `--name=${randomEntropyLabel("zkey-contrib1")}`,
        "-v",
        `-e=${randomEntropyLabel("zkey-entropy1")}`,
      ],
      { verboseTag: "groth16:contribute" }
    );
  } else {
    log(`↺ Skipping zkey contribution (exists): ${zkeyStep2}`);
  }

  if (!fs.existsSync(zkeyPath)) {
    log("▶ Applying zkey beacon");
    await run(
      snarkjsBin,
      [
        "zkey",
        "beacon",
        zkeyStep2,
        zkeyPath,
        args.beacon,
        String(args.beaconIterations),
      ],
      { verboseTag: "groth16:beacon" }
    );
  } else {
    log(`↺ Skipping zkey beacon (exists): ${zkeyPath}`);
  }

  log("▶ Verifying final zkey");
  await run(snarkjsBin, ["zkey", "verify", r1csPath, ptauFinalPath, zkeyPath], {
    verboseTag: "groth16:verify",
  });

  log("▶ Exporting verification key");
  await run(
    snarkjsBin,
    ["zkey", "export", "verificationkey", zkeyPath, vkeyPath],
    { verboseTag: "groth16:vkey" }
  );

  log(`✓ Groth16 setup completed (zkey + vkey): ${zkeyPath}`);
}

async function main() {
  const buildDir = args.output as string;
  const circuitName = args.circuit as string;
  const ptauDir = path.isAbsolute(args.ptauDir)
    ? args.ptauDir
    : path.join(cwd, args.ptauDir);

  ensureDir(buildDir);
  log(`• Output dir: ${buildDir}`);
  log(`• Circuit: ${circuitName}`);
  log(`• Curve: bls12-381`);
  log(`• Power: ${args.power}`);
  log(`• Ptau dir: ${ptauDir}`);

  const { ptauFinalPath } = await runPowersOfTau(ptauDir, args.power as number);
  await runGroth16Setup(ptauFinalPath, buildDir, circuitName);
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error("Error:", err);
    process.exit(1);
  });
