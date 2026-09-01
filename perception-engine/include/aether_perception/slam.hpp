// Aether Perception Engine — SLAM & Mapping interface
// Blueprint reference: §5.2 ("SLAM & Mapping") — production targets
// ORB-SLAM3 / LIO-SAM / DROID-SLAM per the blueprint's references.
#pragma once

#include "aether_perception/scene_graph.hpp"
#include <vector>

namespace aether::perception {

/// A single 3D point in the reconstructed map, e.g. from a LiDAR sweep
/// or a stereo/RGB-D depth frame.
struct MapPoint {
    std::array<double, 3> position;
};

/// Interface for a SLAM backend. Swap in a real ORB-SLAM3/LIO-SAM
/// binding by implementing this interface — the rest of the perception
/// pipeline only depends on `SlamBackend`, not on any specific library.
class SlamBackend {
public:
    virtual ~SlamBackend() = default;

    /// Process one frame of point-cloud data and update the internal map
    /// and pose estimate. Returns the current estimated camera/sensor pose.
    virtual Transform track_frame(const std::vector<MapPoint>& points) = 0;

    /// A snapshot of the currently reconstructed map (for visualization
    /// or handing off to the semantic segmentation stage).
    virtual std::vector<MapPoint> map_snapshot() const = 0;
};

/// A trivial placeholder SLAM backend: "tracks" pose as the centroid of
/// the incoming points and accumulates them into the map without any
/// actual loop closure or optimization. Exists purely so the pipeline
/// has something to run end-to-end before a real SLAM library is
/// integrated (roadmap Stage 1).
class CentroidSlamStub : public SlamBackend {
public:
    Transform track_frame(const std::vector<MapPoint>& points) override;
    std::vector<MapPoint> map_snapshot() const override { return map_; }

private:
    std::vector<MapPoint> map_;
    Transform last_pose_{};
};

} // namespace aether::perception
