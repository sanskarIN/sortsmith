import { readFileSync } from "node:fs";

function fail(message) {
  console.error(`Release version check failed: ${message}`);
  process.exit(1);
}

function normalizedTag(raw) {
  const value = raw?.trim();
  if (!value) return null;
  const version = value.startsWith("v") ? value.slice(1) : value;
  if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(version)) {
    fail(`'${value}' is not a supported release tag`);
  }
  return version;
}

function workspaceVersion(cargoToml) {
  const section = cargoToml.match(/\[workspace\.package\]([\s\S]*?)(?:\n\[|$)/)?.[1];
  const version = section?.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  if (!version) fail("could not read [workspace.package] version from Cargo.toml");
  return version;
}

const frontend = JSON.parse(readFileSync(new URL("../apps/desktop/package.json", import.meta.url), "utf8"));
const tauri = JSON.parse(readFileSync(new URL("../apps/desktop/src-tauri/tauri.conf.json", import.meta.url), "utf8"));
const cargo = readFileSync(new URL("../Cargo.toml", import.meta.url), "utf8");
const cargoVersion = workspaceVersion(cargo);
const requested = normalizedTag(process.argv[2] ?? (process.env.GITHUB_REF_TYPE === "tag" ? process.env.GITHUB_REF_NAME : undefined));
const expected = requested ?? cargoVersion;

const versions = new Map([
  ["workspace Cargo.toml", cargoVersion],
  ["desktop package.json", frontend.version],
  ["Tauri configuration", tauri.version],
]);

const mismatches = [...versions].filter(([, version]) => version !== expected);
if (mismatches.length > 0) {
  const prefix = requested ? `tag v${expected}` : `workspace version ${expected}`;
  fail(`${prefix} does not match ${mismatches.map(([name, version]) => `${name} (${version ?? "missing"})`).join(", ")}`);
}

console.log(`Release version ${expected} is consistent across Cargo, frontend, and Tauri metadata${requested ? ` for tag v${expected}` : ""}.`);
