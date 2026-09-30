import { existsSync, readFileSync } from "node:fs";

function fail(message) {
  console.error(`Release lockfile check failed: ${message}`);
  process.exit(1);
}

const cargoLock = new URL("../Cargo.lock", import.meta.url);
const npmLock = new URL("../apps/desktop/package-lock.json", import.meta.url);
const packageJson = new URL("../apps/desktop/package.json", import.meta.url);

if (!existsSync(cargoLock)) fail("Cargo.lock is missing; generate and commit it from a trusted Rust environment before tagging");
if (!existsSync(npmLock)) fail("apps/desktop/package-lock.json is missing; generate and commit it with the supported npm version before tagging");

const frontend = JSON.parse(readFileSync(packageJson, "utf8"));
const npm = JSON.parse(readFileSync(npmLock, "utf8"));
const rootPackage = npm.packages?.[""];

if (!Number.isInteger(npm.lockfileVersion) || npm.lockfileVersion < 3) {
  fail(`package-lock.json uses unsupported lockfileVersion ${String(npm.lockfileVersion)}`);
}
if (rootPackage?.version !== frontend.version) {
  fail(`package-lock.json root version (${rootPackage?.version ?? "missing"}) does not match package.json (${frontend.version})`);
}

const cargo = readFileSync(cargoLock, "utf8");
for (const packageName of ["sortsmith", "sortsmith-core"]) {
  const escapedName = packageName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const entry = new RegExp(`\\[\\[package\\]\\][\\s\\S]*?name = "${escapedName}"[\\s\\S]*?version = "([^"]+)"`).exec(cargo);
  if (!entry) fail(`Cargo.lock does not contain the workspace package ${packageName}`);
  if (entry[1] !== frontend.version) fail(`Cargo.lock ${packageName} version (${entry[1]}) does not match package.json (${frontend.version})`);
}

console.log(`Release lockfiles are present and aligned with version ${frontend.version}.`);
