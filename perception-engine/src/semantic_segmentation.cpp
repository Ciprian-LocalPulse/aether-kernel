#include "aether_perception/semantic_segmentation.hpp"

namespace aether::perception {

void merge_detections_into_graph(
    SceneGraph& graph,
    const std::vector<Detection>& detections
) {
    uint64_t synthetic_id = 1;
    for (const auto& det : detections) {
        // NOTE: production code must resolve detections to *persistent*
        // object IDs (via re-identification / tracking across frames),
        // not mint a fresh ID every call as this stub does.
        auto node = graph.get_or_create(ObjectId{0, synthetic_id++}, det.label);
        node->set_transform(det.pose);

        SemanticProperties props;
        props.label = det.label;
        props.confidence = det.confidence;
        node->set_semantics(std::move(props));
    }
}

} // namespace aether::perception
