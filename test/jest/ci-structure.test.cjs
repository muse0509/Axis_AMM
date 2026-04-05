const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const repoRoot = path.resolve(__dirname, "..", "..");

function readFile(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function runBunScript(relativePath, extraEnv = {}) {
  return spawnSync("bun", [relativePath], {
    cwd: repoRoot,
    env: { ...process.env, ...extraEnv },
    stdio: "inherit",
  });
}

describe("CI Structure", () => {
  test("workflow delegates job logic to shell scripts", () => {
    const workflow = readFile(".github/workflows/ci.yml");

    expect(workflow).toContain("bash ci/job-rust-and-typescript.sh");
    expect(workflow).toContain("bash ci/job-local-e2e.sh");
  });

  test("required CI scripts are tracked", () => {
    const requiredScripts = [
      "ci/setup-rust.sh",
      "ci/install-solana.sh",
      "ci/rust-tests.sh",
      "ci/rust-build-sbf.sh",
      "ci/rust-lint.sh",
      "ci/ts-typecheck.sh",
      "ci/ts-lint.sh",
      "ci/jest.sh",
      "ci/job-rust-and-typescript.sh",
      "ci/job-local-e2e.sh",
    ];

    for (const script of requiredScripts) {
      expect(fs.existsSync(path.join(repoRoot, script))).toBe(true);
    }
  });

  test("bun runtime is available for test execution", () => {
    const result = spawnSync("bun", ["--version"], {
      cwd: repoRoot,
      encoding: "utf8",
    });

    expect(result.status).toBe(0);
    expect(result.stdout.trim().length).toBeGreaterThan(0);
  });

  const runLocal = process.env.RUN_SOLANA_LOCAL_TESTS === "1" ? test : test.skip;
  runLocal(
    "local suite can be orchestrated by jest through bun",
    () => {
      const result = runBunScript("test/e2e/pfda-amm-3/pfda-amm-3.local.e2e.ts", {
        RPC_URL: "http://localhost:8899",
        WINDOW_SLOTS: "10",
      });

      expect(result.status).toBe(0);
    },
    15 * 60 * 1000,
  );

  const runDevnet = process.env.RUN_SOLANA_DEVNET_TESTS === "1" ? test : test.skip;
  runDevnet(
    "devnet suite can be orchestrated by jest through bun",
    () => {
      const result = runBunScript("test/e2e/pfda-amm-3/pfda-amm-3.o1-proof.test.ts");
      expect(result.status).toBe(0);
    },
    20 * 60 * 1000,
  );
});
