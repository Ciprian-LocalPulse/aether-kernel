// Minimal end-to-end demo: synthetic point cloud -> SLAM stub ->
// segmentation stub -> scene graph, printed to stdout. Exercises the
// full perception pipeline shape described in blueprint §5.2.
#include "aether_perception/scene_graph.hpp"
#include "aether_perception/sensor_fusion.hpp"
#include "aether_perception/semantic_segmentation.hpp"
#include "aether_perception/slam.hpp"

#include <iostream>

namespace ap = aether::perception;

namespace {

class StubSegmentation : public ap::SegmentationBackend {
public:
    std::vector<ap::Detection> detect(const std::vector<ap::MapPoint>&) override {
        return {
            ap::Detection{"chair", 0.91f, ap::Transform{{1.0, 0.0, 0.0}, {0, 0, 0, 1}}},
            ap::Detection{"table", 0.87f, ap::Transform{{2.0, 0.0, 0.0}, {0, 0, 0, 1}}},
        };
    }
};

} // namespace

int main() {
    std::vector<ap::MapPoint> synthetic_cloud = {
        {{0.1, 0.0, 1.0}}, {{0.2, 0.1, 1.1}}, {{-0.1, 0.0, 0.9}},
    };

    ap::CentroidSlamStub slam;
    auto pose = slam.track_frame(synthetic_cloud);
    std::cout << "[perception] estimated pose: ("
              << pose.position[0] << ", " << pose.position[1] << ", " << pose.position[2]
              << ")\n";

    StubSegmentation segmentation;
    auto detections = segmentation.detect(slam.map_snapshot());

    ap::SceneGraph graph;
    ap::merge_detections_into_graph(graph, detections);

    std::cout << "[perception] scene graph node count: " << graph.node_count() << "\n";
    for (const auto& child : graph.root()->children()) {
        std::cout << "  - " << child->name();
        if (child->semantics()) {
            std::cout << " (confidence=" << child->semantics()->confidence << ")";
        }
        std::cout << "\n";
    }
    return 0;
}
