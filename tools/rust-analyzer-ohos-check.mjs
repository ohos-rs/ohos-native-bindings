#!/usr/bin/env node

import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const targets = {
  "aarch64-unknown-linux-ohos": {
    clangTarget: "aarch64-linux-ohos",
    linker: "aarch64-unknown-linux-ohos-clang",
    cxx: "aarch64-unknown-linux-ohos-clang++",
  },
  "armv7-unknown-linux-ohos": {
    clangTarget: "arm-linux-ohos",
    linker: "armv7-unknown-linux-ohos-clang",
    cxx: "armv7-unknown-linux-ohos-clang++",
    extraClangArgs: ["-march=armv7-a", "-mfloat-abi=softfp", "-mtune=generic-armv7-a", "-mthumb"],
  },
  "x86_64-unknown-linux-ohos": {
    clangTarget: "x86_64-linux-ohos",
    linker: "x86_64-unknown-linux-ohos-clang",
    cxx: "x86_64-unknown-linux-ohos-clang++",
  },
};

const target = process.env.OHOS_RA_TARGET || "aarch64-unknown-linux-ohos";
const targetInfo = targets[target];

if (!targetInfo) {
  fail(`Unsupported OHOS_RA_TARGET: ${target}`);
}

const sdkRoot = resolveSdkRoot();
const llvmBin = path.join(sdkRoot, "native", "llvm", "bin");
const sysroot = path.join(sdkRoot, "native", "sysroot");
const clang = findTool(llvmBin, "clang");
const clangxx = findTool(llvmBin, "clang++");
const linker = findTool(llvmBin, targetInfo.linker) || clang;
const cxx = findTool(llvmBin, targetInfo.cxx) || clangxx;
const ar = findTool(llvmBin, "llvm-ar");
const ranlib = findTool(llvmBin, "llvm-ranlib");
const libclang = path.join(sdkRoot, "native", "llvm", "lib");

if (!fs.existsSync(sysroot)) {
  fail(`OpenHarmony sysroot not found: ${sysroot}`);
}
if (!linker) {
  fail(`OpenHarmony linker not found under: ${llvmBin}`);
}

const env = { ...process.env };
const cargoTargetKey = target.toUpperCase().replaceAll("-", "_");
const ccTargetKey = target.replaceAll("-", "_");
const clangArgs = [
  `--target=${targetInfo.clangTarget}`,
  `--sysroot=${sysroot}`,
  "-D__MUSL__",
  ...(targetInfo.extraClangArgs || []),
];
const clangArgsEnv = clangArgs.join(" ");
env.OHOS_NDK_HOME = sdkRoot;
env.OHOS_SDK_HOME = env.OHOS_SDK_HOME || sdkRoot;
env.PATH = `${llvmBin}${path.delimiter}${env.PATH || ""}`;
env[`CARGO_TARGET_${cargoTargetKey}_LINKER`] = linker;
env[`CC_${ccTargetKey}`] = linker;
env[`CXX_${ccTargetKey}`] = cxx || linker;
env[`CFLAGS_${ccTargetKey}`] = appendEnv(env[`CFLAGS_${ccTargetKey}`], clangArgsEnv);
env[`CXXFLAGS_${ccTargetKey}`] = appendEnv(env[`CXXFLAGS_${ccTargetKey}`], clangArgsEnv);
env.TARGET_CC = linker;
env.TARGET_CXX = cxx || linker;
env.TARGET_CFLAGS = appendEnv(env.TARGET_CFLAGS, clangArgsEnv);
env.TARGET_CXXFLAGS = appendEnv(env.TARGET_CXXFLAGS, clangArgsEnv);
if (ar) {
  env[`AR_${ccTargetKey}`] = ar;
  env.TARGET_AR = ar;
}
if (ranlib) {
  env[`RANLIB_${ccTargetKey}`] = ranlib;
  env.TARGET_RANLIB = ranlib;
}
env.LIBCLANG_PATH = libclang;
env.CLANG_PATH = clang || linker;

const bindgenKey = `BINDGEN_EXTRA_CLANG_ARGS_${target.replaceAll("-", "_")}`;
env[bindgenKey] = appendEnv(env[bindgenKey], clangArgsEnv);
env[bindgenKey.toUpperCase()] = appendEnv(env[bindgenKey.toUpperCase()], clangArgsEnv);

if (!findTool(llvmBin, targetInfo.linker)) {
  const rustFlags = clangArgs.map((arg) => `-Clink-arg=${arg}`).join("\x1f");
  env.CARGO_ENCODED_RUSTFLAGS = appendEnv(env.CARGO_ENCODED_RUSTFLAGS, rustFlags, "\x1f");
}

const cargoArgs = process.argv.slice(2);
const args = cargoArgs.length
  ? cargoArgs
  : ["check", "--workspace", "--target", target, "--message-format=json", "--all-targets"];

const cargo = process.platform === "win32" ? "cargo.exe" : "cargo";
const child = spawn(cargo, args, { env, stdio: ["ignore", "pipe", "pipe"] });

let stdoutBuffer = "";
child.stdout.setEncoding("utf8");
child.stdout.on("data", (chunk) => {
  stdoutBuffer += chunk;
  const lines = stdoutBuffer.split(/\r?\n/);
  stdoutBuffer = lines.pop() || "";
  for (const line of lines) {
    writeJsonLine(line);
  }
});

child.stderr.pipe(process.stderr);

child.on("error", (error) => {
  fail(`Failed to run cargo: ${error.message}`);
});

child.on("close", (code, signal) => {
  if (stdoutBuffer) {
    writeJsonLine(stdoutBuffer);
  }
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 1);
});

function resolveSdkRoot() {
  const candidates = [process.env.OHOS_NDK_HOME, process.env.OHOS_SDK_HOME].filter(Boolean);
  for (const candidate of candidates) {
    if (fs.existsSync(path.join(candidate, "native", "llvm", "bin"))) {
      return path.resolve(candidate);
    }
  }
  fail("Set OHOS_NDK_HOME or OHOS_SDK_HOME to an OpenHarmony SDK root containing native/llvm/bin.");
}

function findTool(dir, baseName) {
  const suffixes = process.platform === "win32" ? [".cmd", ".bat", ".exe", ""] : ["", ".cmd", ".bat", ".exe"];
  for (const suffix of suffixes) {
    const candidate = path.join(dir, `${baseName}${suffix}`);
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }
  return undefined;
}

function appendEnv(current, value, sep = " ") {
  return current ? `${current}${sep}${value}` : value;
}

function writeJsonLine(line) {
  if (line.trimStart().startsWith("{")) {
    process.stdout.write(`${line}\n`);
  } else if (line.trim()) {
    process.stderr.write(`${line}\n`);
  }
}

function fail(message) {
  process.stderr.write(`${message}\n`);
  process.exit(1);
}
