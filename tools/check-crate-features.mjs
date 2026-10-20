#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";

const { values } = parseArgs({
  options: {
    target: { type: "string" },
    package: { type: "string", multiple: true, short: "p" },
    "all-api-levels": { type: "boolean", default: false },
    lint: { type: "boolean", default: false },
    offline: { type: "boolean", default: false },
    "verify-forwarding-only": { type: "boolean", default: false },
  },
});
const root = fileURLToPath(new URL("../", import.meta.url));
const cargo = process.platform === "win32" ? "cargo.exe" : "cargo";
const commonArgs = values.offline ? ["--offline"] : [];

function runCargo(args, rustcArgs = []) {
  const result = spawnSync(cargo, [...args, ...commonArgs, ...rustcArgs], {
    cwd: root,
    // Every API level has a distinct compilation unit. Incremental/debug
    // artifacts add substantial disk usage without helping this sweep.
    env: {
      ...process.env,
      CARGO_INCREMENTAL: "0",
      CARGO_PROFILE_DEV_DEBUG: "0",
      CARGO_PROFILE_TEST_DEBUG: "0",
    },
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error || result.signal) {
    throw result.error ?? new Error(`cargo terminated by ${result.signal}`);
  }
  return result;
}

const metadata = runCargo(["metadata", "--format-version", "1", "--no-deps"]);
if (metadata.status !== 0) {
  process.stderr.write(metadata.stderr);
  process.exit(metadata.status ?? 1);
}
const workspace = JSON.parse(metadata.stdout);
function packageDirectory(pkg) {
  return path.relative(root, pkg.manifest_path).split(path.sep)[0];
}
const nativePackages = workspace.packages
  .filter(
    (pkg) =>
      workspace.workspace_members.includes(pkg.id) &&
      ["crates", "sys"].includes(packageDirectory(pkg)),
  )
  .sort((a, b) => a.name.localeCompare(b.name, "en"));

// Check declared edges, not only a unified build: a sibling dependency can
// otherwise hide a missing API forward by enabling the feature transitively.
const packagesByName = new Map(nativePackages.map((pkg) => [pkg.name, pkg]));
const forwardingErrors = [];
let forwardedEdges = 0;
for (const pkg of nativePackages) {
  for (const [feature, enabled] of Object.entries(pkg.features)) {
    const api = /^api-(\d+)$/.exec(feature);
    if (!api) continue;
    const previous = `api-${Number(api[1]) - 1}`;
    if (Number(api[1]) > 13 && !enabled.includes(previous)) {
      forwardingErrors.push(`${pkg.name}/${feature} must inherit ${previous}`);
    }
    for (const dependency of pkg.dependencies) {
      if (dependency.kind !== null) continue;
      const target = packagesByName.get(dependency.name);
      // Forward only levels actually exposed by this dependency. Higher parent
      // levels inherit the child's last supported level through the API chain.
      if (!target || !Object.hasOwn(target.features, feature)) continue;
      const name = dependency.rename ?? dependency.name;
      const strong = `${name}/${feature}`;
      const expected = `${name}${dependency.optional ? "?" : ""}/${feature}`;
      if (!enabled.includes(expected)) {
        forwardingErrors.push(`${pkg.name}/${feature} must forward ${expected}`);
      } else {
        forwardedEdges++;
      }
      if (dependency.optional && enabled.includes(strong)) {
        forwardingErrors.push(
          `${pkg.name}/${feature} must use ${expected}; activate capabilities explicitly with dep:${name}`,
        );
      }
    }
  }
}
if (forwardingErrors.length) {
  process.stderr.write(`${forwardingErrors.join("\n")}\n`);
  process.exit(1);
}
console.log(`API forwarding: ${nativePackages.length} packages, ${forwardedEdges} dependency edges checked`);
if (values["verify-forwarding-only"]) process.exit(0);

const packages = nativePackages.filter((pkg) => pkg.publish === null || pkg.publish.length > 0);
for (const name of values.package ?? []) {
  if (!packages.some((pkg) => pkg.name === name)) {
    throw new Error(`Unknown publishable workspace package: ${name}`);
  }
}

// Some non-API features have an API floor (for example net-stack's http
// enables api-20). Do not let them silently raise a lower-level check.
function minimumApi(pkg, feature, seen = new Set()) {
  if (seen.has(feature)) return 12;
  seen.add(feature);
  const api = /(?:^|\/)api-(\d+)$/.exec(feature);
  if (api) return Number(api[1]);
  return Math.max(
    12,
    ...(pkg.features[feature] ?? []).map((dependency) => minimumApi(pkg, dependency, seen)),
  );
}

let checks = 0;
const failures = [];
const plannedChecks = [];
for (const pkg of packages) {
  if (values.package && !values.package.includes(pkg.name)) continue;
  const configurations = [["default", []]];
  if (pkg.features.default?.length) {
    configurations.push(["no-default-features", ["--no-default-features"]]);
  }
  if (values["all-api-levels"]) {
    const capabilities = Object.keys(pkg.features)
      .filter((feature) => feature !== "default" && !feature.startsWith("api-"))
      .sort();
    const baseline = capabilities.filter((feature) => minimumApi(pkg, feature) <= 12);
    if (baseline.length) {
      configurations.push([
        "capabilities",
        ["--no-default-features", "--features", baseline.join(",")],
      ]);
    }
    const apiFeatures = Object.keys(pkg.features)
      .filter((feature) => /^api-\d+$/.test(feature))
      .sort((a, b) => Number(a.slice(4)) - Number(b.slice(4)));
    for (const feature of apiFeatures) {
      configurations.push([feature, ["--no-default-features", "--features", feature]]);
      const enabled = capabilities.filter(
        (capability) => minimumApi(pkg, capability) <= Number(feature.slice(4)),
      );
      if (enabled.length) {
        configurations.push([
          `${feature} + capabilities`,
          ["--no-default-features", "--features", [feature, ...enabled].join(",")],
        ]);
      }
    }
  }

  for (const [label, featureArgs] of configurations) {
    plannedChecks.push({ pkg, label, featureArgs });
  }
}

// Keep example/test lint coverage with the API requirements declared by each
// example, without unifying their features with other packages in the sweep.
if (values.lint && !values.package) {
  for (const pkg of workspace.packages
    .filter(
      (pkg) => workspace.workspace_members.includes(pkg.id) && packageDirectory(pkg) === "examples",
    )
    .sort((a, b) => a.name.localeCompare(b.name, "en"))) {
    plannedChecks.push({ pkg, label: "example defaults", featureArgs: [] });
  }
}

// Finish each API level across the crates before moving to the next level.
function apiLevel(label) {
  return Number(/^api-(\d+)/.exec(label)?.[1] ?? 0);
}
plannedChecks.sort((a, b) => apiLevel(a.label) - apiLevel(b.label));
for (const { pkg, label, featureArgs } of plannedChecks) {
  // Separate invocations prevent examples and other crates from enabling
  // dependency features that would hide a broken standalone package.
  const args = [
    values.lint ? "clippy" : "check",
    values.lint ? "--all-targets" : "--lib",
    "--package",
    pkg.name,
    ...featureArgs,
  ];
  if (values.target) args.push("--target", values.target);
  const result = runCargo(args, values.lint ? ["--", "-D", "warnings"] : []);
  checks += 1;
  if (result.status === 0) {
    console.log(`PASS ${pkg.name} (${label})`);
  } else {
    const failure = `${pkg.name} (${label})`;
    failures.push(failure);
    console.error(`FAIL ${failure}\n${result.stdout}${result.stderr}`);
  }
}

console.log(`${checks} checks, ${failures.length} failures`);
if (failures.length) {
  console.error(failures.join("\n"));
  process.exitCode = 1;
}
