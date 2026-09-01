/**
 * Aether SDK (JavaScript/TypeScript)
 *
 * Client API mirroring blueprint §9.1: connect, createObject,
 * updateObject, subscribe, sendIntent. Targets WebXR apps running in
 * the browser.
 *
 * Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0
 */

export interface Transform {
  position: [number, number, number];
  orientation: [number, number, number, number];
}

export class ObjectId {
  readonly value: number;
  constructor(value: number) {
    this.value = value;
  }
}

export class SdkError extends Error {}

type SubscriberCallback = (id: ObjectId, transform: Transform) => void;

const IDENTITY_TRANSFORM: Transform = {
  position: [0, 0, 0],
  orientation: [0, 0, 0, 1],
};

/**
 * A connection to an Aether node. This scaffold keeps object state
 * in-memory only — a production client would talk to the sync-engine
 * over WebTransport/QUIC. See `sync-engine/src/network.rs`.
 */
export class AetherClient {
  private connected = true;
  private nextId = 1;
  private objects = new Map<number, Transform>();
  private subscribers = new Map<string, SubscriberCallback[]>();
  readonly endpoint: string;

  private constructor(endpoint: string) {
    this.endpoint = endpoint;
  }

  static connect(endpoint: string): AetherClient {
    return new AetherClient(endpoint);
  }

  private requireConnected(): void {
    if (!this.connected) {
      throw new SdkError("not connected");
    }
  }

  createObject(transform: Transform = IDENTITY_TRANSFORM): ObjectId {
    this.requireConnected();
    const id = new ObjectId(this.nextId++);
    this.objects.set(id.value, transform);
    return id;
  }

  updateObject(id: ObjectId, transform: Transform): void {
    this.requireConnected();
    if (!this.objects.has(id.value)) {
      throw new SdkError(`unknown object: ${id.value}`);
    }
    this.objects.set(id.value, transform);
    for (const cb of this.subscribers.get("*") ?? []) {
      cb(id, transform);
    }
  }

  subscribe(cellId: string, callback: SubscriberCallback): void {
    this.requireConnected();
    const list = this.subscribers.get(cellId) ?? [];
    list.push(callback);
    this.subscribers.set(cellId, list);
  }

  /** Forward a natural-language intent to the Intent Router.
   * TODO(roadmap Stage 5): bridge to `intent-router` over HTTP/gRPC. */
  sendIntent(text: string, context?: Record<string, unknown>): void {
    this.requireConnected();
    void text;
    void context;
  }

  disconnect(): void {
    this.connected = false;
  }
}
