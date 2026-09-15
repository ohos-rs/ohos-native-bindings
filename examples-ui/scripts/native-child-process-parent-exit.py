#!/usr/bin/env python3
"""Terminate only the E2E parent, then observe cleanup of all four child APIs."""
import argparse
import json
import re
import subprocess
import time
from pathlib import Path


class ParentExitRunner:
    def __init__(self, options):
        self.options = options
        self.directory = Path(options.diagnostics_dir)
        self.directory.mkdir(parents=True, exist_ok=True)
        self.hdc = [options.hdc]
        if options.target:
            self.hdc += ["-t", options.target]

    def shell(self, command):
        return subprocess.run(self.hdc + ["shell", command], check=True,
                              capture_output=True, text=True, timeout=5).stdout

    def alive(self, pid):
        return "alive" in self.shell(f"if test -d /proc/{pid}; then echo alive; else echo gone; fi")

    def run(self):
        hilog_path = self.directory / "parent-exit-hilog.log"
        aa_path = self.directory / "parent-exit-aa.log"
        processes = []
        result = {"passed": False}
        with hilog_path.open("wb") as hilog, aa_path.open("wb") as aa:
            try:
                processes.append(subprocess.Popen(self.hdc + ["shell", "hilog -T JSAPP"],
                                                  stdout=hilog, stderr=subprocess.STDOUT))
                processes.append(subprocess.Popen(self.hdc + ["shell",
                    f"aa test -b {self.options.bundle} -m entry_test -s unittest OpenHarmonyTestRunner "
                    "-s class native_child_process_parent_exit -s timeout 30000"],
                    stdout=aa, stderr=subprocess.STDOUT))
                deadline = time.monotonic() + 10
                pattern = r"\[native_child_process_parent_exit\] parent_pid=(\d+); child_pids=(\d+),(\d+),(\d+),(\d+); apis=start,start_with_configs,create,create_with_configs; armed=true"
                pids = None
                while time.monotonic() < deadline:
                    matches = list(re.finditer(pattern, hilog_path.read_text(errors="replace")))
                    for match in reversed(matches):
                        candidate = list(map(int, match.groups()))
                        if len(set(candidate)) == 5 and all(self.alive(pid) for pid in candidate):
                            pids = candidate
                            break
                    if pids:
                        break
                    time.sleep(0.05)
                if not pids:
                    raise RuntimeError("parent-exit fixture did not report four live children")
                for pid in pids:
                    if self.options.bundle not in self.shell(f"cat /proc/{pid}/cmdline"):
                        raise RuntimeError(f"PID {pid} does not belong to the E2E application")
                parent, *children = pids
                result.update(parent_pid=parent, child_pids=children,
                              apis=["start", "start_with_configs", "create", "create_with_configs"])
                # Kill only the observed parent. Bundle force-stop could kill
                # children directly and would not prove parent-death cleanup.
                self.shell(f"kill -9 {parent}")
                killed_at = time.monotonic()
                deadline = killed_at + 5
                while time.monotonic() < deadline and any(self.alive(pid) for pid in pids):
                    time.sleep(0.05)
                remaining = [pid for pid in pids if self.alive(pid)]
                if remaining:
                    raise RuntimeError(f"processes survived parent termination: {remaining}")
                result.update(passed=True, parent_signal=9,
                              cleanup_ms=round((time.monotonic() - killed_at) * 1000))
                print("[native_child_process_parent_exit] " + json.dumps(result))
            except Exception as error:
                result["error"] = str(error)
                raise
            finally:
                if not result["passed"]:
                    try:
                        self.shell(f"aa force-stop {self.options.bundle}")
                    except Exception:
                        pass
                for process in reversed(processes):
                    if process.poll() is None:
                        process.terminate()
                        try:
                            process.wait(timeout=2)
                        except subprocess.TimeoutExpired:
                            process.kill()
                            process.wait(timeout=2)
                (self.directory / "parent-exit-result.json").write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--hdc", default="hdc")
    parser.add_argument("--target", default="")
    parser.add_argument("--bundle", default="com.richerfu.ohos_example")
    parser.add_argument("--diagnostics-dir", required=True)
    ParentExitRunner(parser.parse_args()).run()
