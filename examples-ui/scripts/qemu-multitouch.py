#!/usr/bin/env python3
"""Inject physical touch input into a QEMU virtio-multitouch device."""

from __future__ import annotations

import argparse
import json
import socket
import time
from pathlib import Path
from typing import Any


ABS_MAX = 0x7FFF


class QmpClient:
    def __init__(self, socket_path: Path) -> None:
        self._socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self._socket.settimeout(10)
        self._socket.connect(str(socket_path))
        self._reader = self._socket.makefile("r", encoding="utf-8")
        self._next_id = 1
        self._read_message()
        self.execute("qmp_capabilities")

    def close(self) -> None:
        self._reader.close()
        self._socket.close()

    def _read_message(self) -> dict[str, Any]:
        line = self._reader.readline()
        if not line:
            raise RuntimeError("QMP connection closed before a response arrived")
        return json.loads(line)

    def execute(self, command: str, arguments: dict[str, Any] | None = None) -> None:
        request_id = self._next_id
        self._next_id += 1
        request: dict[str, Any] = {"execute": command, "id": request_id}
        if arguments is not None:
            request["arguments"] = arguments
        payload = json.dumps(request, separators=(",", ":")) + "\n"
        self._socket.sendall(payload.encode("utf-8"))
        while True:
            response = self._read_message()
            if response.get("id") != request_id:
                continue
            if "error" in response:
                raise RuntimeError(f"QMP {command} failed: {response['error']}")
            return

    def input(self, events: list[dict[str, Any]]) -> None:
        self.execute("input-send-event", {"events": events})


def scaled(value: int, maximum: int) -> int:
    if maximum <= 0:
        raise ValueError("display dimensions must be positive")
    return max(0, min(ABS_MAX, round(value * ABS_MAX / maximum)))


def multitouch_event(
    event_type: str,
    tracking_id: int,
    *,
    axis: str = "x",
    value: int = 0,
) -> dict[str, Any]:
    return {
        "type": "mtt",
        "data": {
            "type": event_type,
            "slot": 0,
            "tracking-id": tracking_id,
            "axis": axis,
            "value": value,
        },
    }


def touch_frame(event_type: str, x: int, y: int) -> list[dict[str, Any]]:
    return [
        multitouch_event(event_type, 0),
        {"type": "btn", "data": {"button": "touch", "down": True}},
        multitouch_event("data", 0, axis="x", value=x),
        multitouch_event("data", 0, axis="y", value=y),
    ]


def touch_end() -> list[dict[str, Any]]:
    # QEMU's console touch helper releases a slot by publishing -1 as its
    # tracking ID. Keeping the previous ID leaves the guest contact pressed.
    return [multitouch_event("end", -1)]


def tap(client: QmpClient, x: int, y: int) -> None:
    client.input(touch_frame("begin", x, y))
    time.sleep(0.05)
    client.input(touch_end())


def double_tap(client: QmpClient, x: int, y: int) -> None:
    tap(client, x, y)
    time.sleep(0.1)
    tap(client, x, y)


def swipe(
    client: QmpClient,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    duration_ms: int,
) -> None:
    # Accessibility's gesture recognizer samples only moves farther apart than
    # its DPI-derived threshold. Three large moves are deterministic on the
    # public 800x500 QEMU image and remain below its 300 ms swipe timeout.
    steps = 3
    client.input(touch_frame("begin", x1, y1))
    for step in range(1, steps + 1):
        fraction = step / steps
        x = round(x1 + (x2 - x1) * fraction)
        y = round(y1 + (y2 - y1) * fraction)
        time.sleep(duration_ms / steps / 1000)
        client.input(touch_frame("update", x, y))
    client.input(touch_end())


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--socket", required=True, type=Path)
    parser.add_argument("--width", required=True, type=int)
    parser.add_argument("--height", required=True, type=int)
    subparsers = parser.add_subparsers(dest="command", required=True)

    tap_parser = subparsers.add_parser("tap")
    tap_parser.add_argument("x", type=int)
    tap_parser.add_argument("y", type=int)

    double_tap_parser = subparsers.add_parser("double-tap")
    double_tap_parser.add_argument("x", type=int)
    double_tap_parser.add_argument("y", type=int)

    swipe_parser = subparsers.add_parser("swipe")
    swipe_parser.add_argument("x1", type=int)
    swipe_parser.add_argument("y1", type=int)
    swipe_parser.add_argument("x2", type=int)
    swipe_parser.add_argument("y2", type=int)
    swipe_parser.add_argument("--duration-ms", type=int, default=240)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if not args.socket.is_socket():
        raise RuntimeError(f"QMP socket is unavailable: {args.socket}")
    client = QmpClient(args.socket)
    try:
        if args.command in ("tap", "double-tap"):
            operation = tap if args.command == "tap" else double_tap
            operation(
                client,
                scaled(args.x, args.width),
                scaled(args.y, args.height),
            )
        else:
            swipe(
                client,
                scaled(args.x1, args.width),
                scaled(args.y1, args.height),
                scaled(args.x2, args.width),
                scaled(args.y2, args.height),
                args.duration_ms,
            )
    finally:
        client.close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
