#include "aether_perception/sensor_fusion.hpp"
#include <cstring>

namespace aether::perception {

void NaiveLatestSampleFusion::ingest(const SensorSample& sample) {
    if (sample.kind == SensorKind::Gps && sample.payload.size() >= sizeof(double) * 3) {
        std::memcpy(state_.position.data(), sample.payload.data(), sizeof(double) * 3);
        // A real GPS fix narrows uncertainty; here we just mark it "known".
        state_.position_variance = {1.0, 1.0, 1.0};
    }
    // IMU/LiDAR/camera fusion (velocity estimation, EKF predict/update
    // steps) is intentionally left as a TODO — see roadmap Stage 1.
}

} // namespace aether::perception
