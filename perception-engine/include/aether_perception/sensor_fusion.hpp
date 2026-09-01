// Aether Perception Engine — Sensor Fusion
// Blueprint reference: §5.2 ("Fuziune senzorială")
#pragma once

#include <array>
#include <cstdint>
#include <vector>

namespace aether::perception {

enum class SensorKind { CameraRgbD, Lidar, Imu, Gps, Microphone, Biosensor };

/// A single timestamped sensor sample. Payload is intentionally opaque
/// (raw bytes) at this layer — decoding is sensor-kind-specific and
/// happens in the driver / HAL layer beneath the perception engine.
struct SensorSample {
    SensorKind kind;
    uint64_t timestamp_ns;
    std::vector<uint8_t> payload;
};

/// A fused state estimate: position, velocity, and a diagonal covariance
/// as a stand-in for a full covariance matrix. Real implementations
/// should use a proper Extended Kalman Filter or factor-graph backend
/// (GTSAM), as specified in the blueprint.
struct FusedState {
    std::array<double, 3> position{};
    std::array<double, 3> velocity{};
    std::array<double, 3> position_variance{1e6, 1e6, 1e6}; // large = unknown
};

/// Interface for a sensor fusion backend. `ekf_fusion.cpp` (to be added)
/// would implement this with a real Extended Kalman Filter; this
/// scaffold ships a simple placeholder that just tracks the latest
/// sample per sensor.
class SensorFusionEngine {
public:
    virtual ~SensorFusionEngine() = default;

    /// Ingest one sample and update the internal state estimate.
    virtual void ingest(const SensorSample& sample) = 0;

    /// Current best estimate of the fused state.
    virtual FusedState estimate() const = 0;
};

/// A minimal, dependency-free fusion engine: keeps the most recent GPS
/// reading (decoded as three little-endian doubles) as "position" and
/// reports maximal uncertainty otherwise. This exists so the pipeline
/// compiles and is testable end-to-end before a real EKF is wired in.
class NaiveLatestSampleFusion : public SensorFusionEngine {
public:
    void ingest(const SensorSample& sample) override;
    FusedState estimate() const override { return state_; }

private:
    FusedState state_{};
};

} // namespace aether::perception
