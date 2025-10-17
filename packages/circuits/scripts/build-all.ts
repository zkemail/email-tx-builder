import { Command } from "commander";
import { mkdir } from "fs/promises";
import { readdir } from "fs/promises";
import path from "path";
import { spawn } from "child_process";

type BuildOptions = {
  only?: string[];
  outDir: string;
  libDir: string;
  sequential: boolean;
};

function run(command: string, args: string[], cwd: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd, stdio: "inherit", shell: false });
    child.on("close", (code) => {
      if (code === 0) return resolve();
      reject(
        new Error(`${command} ${args.join(" ")} exited with code ${code}`)
      );
    });
    child.on("error", reject);
  });
}

async function listTopLevelCircuits(srcDir: string): Promise<string[]> {
  const entries = await readdir(srcDir, { withFileTypes: true });
  return (
    entries
      .filter((d) => d.isFile() && d.name.endsWith(".circom"))
      .map((d) => d.name)
      // exclude templates by convention and anything not intended to be built directly
      .filter((name) => !name.endsWith("_template.circom"))
  );
}

async function buildCircuits(
  projectRoot: string,
  opts: BuildOptions
): Promise<void> {
  const srcDir = path.join(projectRoot, "src");
  const outDir = path.join(projectRoot, opts.outDir);

  await mkdir(outDir, { recursive: true });

  const allCircuits = await listTopLevelCircuits(srcDir);
  const selected =
    opts.only && opts.only.length > 0
      ? allCircuits.filter((n) =>
          opts.only!.includes(n.replace(/\.circom$/, ""))
        )
      : allCircuits;

  if (selected.length === 0) {
    const hint =
      allCircuits.length > 0
        ? `Available: ${allCircuits.map((n) => n.replace(/\.circom$/, "")).join(", ")}`
        : "No circuits found";
    throw new Error(`No circuits selected to build. ${hint}`);
  }

  const buildOne = (fileName: string) => {
    const srcPath = path.join("src", fileName);
    const args = [
      srcPath,
      "--r1cs",
      "--wasm",
      "--sym",
      "--c",
      "-l",
      opts.libDir,
      "-o",
      opts.outDir,
    ];
    return run("circom", args, projectRoot);
  };

  if (opts.sequential) {
    for (const f of selected) {
      console.log(`Building ${f}...`);
      await buildOne(f);
    }
    return;
  }

  await Promise.all(
    selected.map(async (f) => {
      console.log(`Building ${f}...`);
      await buildOne(f);
    })
  );
}

async function main() {
  const program = new Command();
  program
    .option("-o, --out-dir <dir>", "output directory", "./build")
    .option(
      "-l, --lib-dir <dir>",
      "circom -l include directory",
      "../../node_modules"
    )
    .option(
      "-c, --circuit <names>",
      "comma-separated circuit base names (without .circom)"
    )
    .option("--sequential", "build sequentially to reduce peak memory", false)
    .option("--list", "list available circuit base names and exit", false);

  program.parse(process.argv);
  const flags = program.opts();

  const opts: BuildOptions = {
    outDir: flags.outDir,
    libDir: flags.libDir,
    sequential: Boolean(flags.sequential),
    only:
      (flags.circuit ?? flags.only)
        ? String(flags.circuit ?? flags.only)
            .split(",")
            .map((s: string) => s.trim())
            .filter(Boolean)
        : undefined,
  };

  const projectRoot = path.resolve(__dirname, "..");

  try {
    if (flags.list) {
      const names = await listTopLevelCircuits(path.join(projectRoot, "src"));
      if (names.length === 0) {
        console.log("No circuits found.");
        return;
      }
      console.log("Available circuits:");
      for (const n of names) {
        console.log("-", n.replace(/\.circom$/, ""));
      }
      return;
    }
    await buildCircuits(projectRoot, opts);
    console.log("All selected circuits built successfully.");
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
}

main();
