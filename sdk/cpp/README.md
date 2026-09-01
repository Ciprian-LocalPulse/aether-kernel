# Aether SDK — C++ (header-only)

Single header, no build step required — `#include <aether/sdk.hpp>`.
Intended for integration with game/robotics engines (Unreal Engine, custom
robotics stacks) per blueprint §9.1.

```bash
g++ -std=c++20 -I include tests/sdk_tests.cpp -o /tmp/sdk_tests && /tmp/sdk_tests
```
