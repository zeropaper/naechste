// VIOLATION: Test should be .test.db.ts based on imports
import { describe, it, expect } from "vitest";
import { ensureTestUser } from "@/tests/utils/test-helpers";

describe("Auth DB tests", () => {
  it("should create user", async () => {
    const user = await ensureTestUser("test@example.com");
    expect(user).toBeDefined();
  });
});
