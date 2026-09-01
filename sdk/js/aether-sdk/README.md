# Aether SDK — JavaScript/TypeScript

For WebXR-based Aether apps running in the browser.

```bash
npm install
npm run build
npm test
```

```ts
import { AetherClient } from "@aether-kernel/sdk";

const client = AetherClient.connect("aether://localhost:9000");
const obj = client.createObject({ position: [0, 0, 1], orientation: [0, 0, 0, 1] });
client.subscribe("*", (id, t) => console.log(id, t));
```
