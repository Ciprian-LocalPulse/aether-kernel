// Aether Perception Engine — Semantic Segmentation interface
// Blueprint reference: §5.2 ("Segmentare semantică") — production
// targets YOLOv8 / Mask2Former / CLIP per the blueprint.
#pragma once

#include "aether_perception/scene_graph.hpp"
#include "aether_perception/slam.hpp"
#include <vector>

namespace aether::perception {

/// A detected object candidate, before it's merged into the scene graph.
struct Detection {
    std::string label;
    float confidence;
    Transform pose;
};

/// Interface for a semantic segmentation / object detection backend.
class SegmentationBackend {
public:
    virtual ~SegmentationBackend() = default;

    /// Run detection over the current map snapshot / sensor frame and
    /// return candidate objects. A real backend would take image/point-
    /// cloud tensors; this interface intentionally stays high-level so
    /// it can front either a classical CV pipeline or a neural model.
    virtual std::vector<Detection> detect(const std::vector<MapPoint>& points) = 0;
};

/// Merge a batch of detections into a scene graph, creating or updating
/// nodes as needed. This is the glue between the segmentation backend
/// and the shared, persistent Scene Graph.
void merge_detections_into_graph(
    SceneGraph& graph,
    const std::vector<Detection>& detections
);

} // namespace aether::perception
