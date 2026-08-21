import { describe, expect, it } from "vitest";
import { dueInMinutes, formatBytes, shortPath } from "./utils";

describe("formatBytes", () => {
  it("formats common sizes", () => {
    expect(formatBytes(512)).toBe("512 B");
    expect(formatBytes(1024)).toBe("1.00 KB");
  });
});

describe("shortPath", () => {
  it("leaves short values untouched", () => expect(shortPath("/tmp/a")).toBe("/tmp/a"));
  it("truncates long values", () => expect(shortPath("a".repeat(100), 20)).toContain("…"));
});

describe("dueInMinutes", () => {
  it("is due when never run", () => expect(dueInMinutes(null, 60)).toBe(true));
});
