# Aether SDK — Python

```bash
pip install -e ".[dev]"
pytest
```

```python
from aether_sdk import AetherClient, Transform

client = AetherClient.connect("aether://localhost:9000")
obj = client.create_object(Transform(position=(0, 0, 1)))
client.subscribe("*", lambda oid, t: print(oid, t))
```
