// Aether SDK — C++ (header-only)
// Blueprint reference: §9.1. Intended for integration with game/robotics
// engines (Unreal Engine, custom robotics stacks) per the blueprint.
//
// Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0
#pragma once

#include <array>
#include <cstdint>
#include <functional>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

namespace aether::sdk {

struct Transform {
    std::array<double, 3> position{0.0, 0.0, 0.0};
    std::array<double, 4> orientation{0.0, 0.0, 0.0, 1.0};
};

struct ObjectId {
    uint64_t value;
    bool operator==(const ObjectId& o) const { return value == o.value; }
};

class SdkError : public std::runtime_error {
public:
    explicit SdkError(const std::string& msg) : std::runtime_error(msg) {}
};

using SubscriberCallback = std::function<void(ObjectId, const Transform&)>;

/// A connection to an Aether node. Header-only, in-memory scaffold —
/// mirrors the Rust/Python/JS SDKs' behavior and API shape so engine
/// integrators (e.g. an Unreal Engine plugin) have a consistent surface
/// to bind against. A production version would hold a QUIC/WebTransport
/// session under the hood.
class AetherClient {
public:
    static AetherClient connect(const std::string& endpoint) {
        return AetherClient(endpoint);
    }

    ObjectId create_object(const Transform& transform = Transform{}) {
        require_connected();
        ObjectId id{next_id_++};
        objects_[id.value] = transform;
        return id;
    }

    void update_object(ObjectId id, const Transform& transform) {
        require_connected();
        auto it = objects_.find(id.value);
        if (it == objects_.end()) {
            throw SdkError("unknown object");
        }
        it->second = transform;
        for (auto& cb : subscribers_["*"]) {
            cb(id, transform);
        }
    }

    void subscribe(const std::string& cell_id, SubscriberCallback callback) {
        require_connected();
        subscribers_[cell_id].push_back(std::move(callback));
    }

    // TODO(roadmap Stage 5): bridge to the Intent Router over HTTP/gRPC.
    void send_intent(const std::string& text) {
        require_connected();
        (void)text;
    }

    void disconnect() { connected_ = false; }

private:
    explicit AetherClient(std::string endpoint) : endpoint_(std::move(endpoint)) {}

    void require_connected() const {
        if (!connected_) throw SdkError("not connected");
    }

    std::string endpoint_;
    bool connected_ = true;
    uint64_t next_id_ = 1;
    std::unordered_map<uint64_t, Transform> objects_;
    std::unordered_map<std::string, std::vector<SubscriberCallback>> subscribers_;
};

} // namespace aether::sdk
