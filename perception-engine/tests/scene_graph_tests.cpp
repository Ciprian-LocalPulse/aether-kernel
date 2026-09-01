// Minimal, dependency-free assertion-based tests (no gtest dependency,
// to keep this scaffold buildable offline). Swap for GoogleTest/Catch2
// when the project grows.
#include "aether_perception/scene_graph.hpp"
#include "aether_perception/slam.hpp"

#include <cassert>
#include <iostream>

using namespace aether::perception;

static void test_get_or_create_is_idempotent() {
    SceneGraph graph;
    auto a = graph.get_or_create(ObjectId{1, 1}, "chair");
    auto b = graph.get_or_create(ObjectId{1, 1}, "chair-again");
    assert(a.get() == b.get());
    assert(graph.node_count() == 2); // root + one object
}

static void test_centroid_slam_stub_tracks_centroid() {
    CentroidSlamStub slam;
    std::vector<MapPoint> points = {{{0, 0, 0}}, {{2, 0, 0}}};
    Transform pose = slam.track_frame(points);
    assert(pose.position[0] == 1.0);
}

int main() {
    test_get_or_create_is_idempotent();
    test_centroid_slam_stub_tracks_centroid();
    std::cout << "All perception-engine tests passed.\n";
    return 0;
}
