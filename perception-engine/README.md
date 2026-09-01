# `perception-engine/` — Perception Engine (C++)

Sensor fusion → SLAM → semantic segmentation → shared Scene Graph, as
specified in
[docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §5](../docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md#5-perception-engine).

## Pipeline

```
Sensors → Preprocessing → Sensor Fusion → SLAM & Mapping → Semantic Segmentation → Scene Graph
```

## Modules

| File | Responsibility |
|---|---|
| `include/aether_perception/scene_graph.hpp` + `src/scene_graph.cpp` | Hierarchical scene graph (space → frame → object → parts) with stable UUIDs |
| `include/aether_perception/sensor_fusion.hpp` + `src/sensor_fusion.cpp` | Multi-sensor fusion scaffold (Extended Kalman Filter interface) |
| `include/aether_perception/slam.hpp` + `src/slam.cpp` | SLAM interface (pose graph, mapping) |
| `include/aether_perception/semantic_segmentation.hpp` + `src/semantic_segmentation.cpp` | Object detection/labeling interface |
| `src/main.cpp` | Minimal demo wiring the pipeline together with synthetic data |

## Production dependencies (not vendored)

Per blueprint §5.2: NVIDIA Isaac ROS, OpenCV, PCL, DeepStream for real
sensor fusion/SLAM; YOLOv8 / Mask2Former / CLIP for segmentation; USD /
glTF for scene serialization. This scaffold defines clean interfaces so
those libraries can be dropped in without changing the public API — see
the commented-out `find_package` lines in `CMakeLists.txt`.

## Build & test

```bash
cmake -B build
cmake --build build
ctest --test-dir build
```
