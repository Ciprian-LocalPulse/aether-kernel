#include "aether_perception/slam.hpp"

namespace aether::perception {

Transform CentroidSlamStub::track_frame(const std::vector<MapPoint>& points) {
    if (!points.empty()) {
        std::array<double, 3> centroid{0.0, 0.0, 0.0};
        for (const auto& p : points) {
            centroid[0] += p.position[0];
            centroid[1] += p.position[1];
            centroid[2] += p.position[2];
        }
        const double n = static_cast<double>(points.size());
        centroid[0] /= n;
        centroid[1] /= n;
        centroid[2] /= n;
        last_pose_.position = centroid;

        map_.insert(map_.end(), points.begin(), points.end());
    }
    return last_pose_;
}

} // namespace aether::perception
