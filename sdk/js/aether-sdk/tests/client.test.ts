import assert from "node:assert/strict";
import test from "node:test";
import { AetherClient, SdkError } from "../src/index.ts";

test("connect and create object", () => {
  const client = AetherClient.connect("aether://localhost:9000");
  const obj = client.createObject();
  assert.equal(obj.value, 1);
});

test("update object notifies subscribers", () => {
  const client = AetherClient.connect("aether://localhost:9000");
  const obj = client.createObject();

  const received: unknown[] = [];
  client.subscribe("*", (id, t) => received.push([id, t]));

  client.updateObject(obj, { position: [1, 2, 3], orientation: [0, 0, 0, 1] });
  assert.equal(received.length, 1);
});

test("operations fail after disconnect", () => {
  const client = AetherClient.connect("aether://localhost:9000");
  client.disconnect();
  assert.throws(() => client.createObject(), SdkError);
});
