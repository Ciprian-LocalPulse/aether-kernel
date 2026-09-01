// Aether Perception Engine — Scene Graph
// Blueprint reference: docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §5.2-5.4
//
// Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0
#pragma once

#include <array>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

namespace aether::perception {

/// A stable, persistent identifier for an object in the shared scene
/// graph. Persistence matters: a virtual screen anchored to a real wall
/// must resolve to the same object across sessions and across nodes.
struct ObjectId {
    uint64_t high;
    uint64_t low;

    bool operator==(const ObjectId& other) const {
        return high == other.high && low == other.low;
    }
};

struct ObjectIdHash {
    size_t operator()(const ObjectId& id) const noexcept {
        return std::hash<uint64_t>{}(id.high) ^ (std::hash<uint64_t>{}(id.low) << 1);
    }
};

/// Rigid-body transform: position (x,y,z) + orientation as a quaternion
/// (x,y,z,w). Kept POD-simple; a real implementation would likely use
/// Eigen or GLM.
struct Transform {
    std::array<double, 3> position{0.0, 0.0, 0.0};
    std::array<double, 4> orientation{0.0, 0.0, 0.0, 1.0}; // identity quaternion
};

/// Semantic metadata attached to an object: a label ("chair", "person"),
/// a confidence score from the detector, and free-form key/value
/// properties (material, function, ownership, ...).
struct SemanticProperties {
    std::string label;
    float confidence = 0.0f;
    std::unordered_map<std::string, std::string> properties;
};

/// A node in the scene graph: Space → Frame → Object → Parts hierarchy
/// (blueprint §5.2). Each node has a transform relative to its parent,
/// optional semantic properties, and children.
class SceneNode {
public:
    explicit SceneNode(ObjectId id, std::string name = "");

    ObjectId id() const { return id_; }
    const std::string& name() const { return name_; }

    void set_transform(const Transform& t) { transform_ = t; }
    const Transform& transform() const { return transform_; }

    void set_semantics(SemanticProperties props) { semantics_ = std::move(props); }
    const std::optional<SemanticProperties>& semantics() const { return semantics_; }

    void add_child(std::shared_ptr<SceneNode> child);
    const std::vector<std::shared_ptr<SceneNode>>& children() const { return children_; }

private:
    ObjectId id_;
    std::string name_;
    Transform transform_;
    std::optional<SemanticProperties> semantics_;
    std::vector<std::shared_ptr<SceneNode>> children_;
};

/// The scene graph itself: an index of all known nodes plus a root.
/// Serialization to USD/glTF (blueprint §5.3) is intentionally out of
/// scope for this scaffold — see the roadmap for that milestone.
class SceneGraph {
public:
    SceneGraph();

    std::shared_ptr<SceneNode> root() const { return root_; }

    /// Create (or fetch, if it already exists) a node by id and register
    /// it in the flat index used for O(1) lookups by ID.
    std::shared_ptr<SceneNode> get_or_create(ObjectId id, const std::string& name = "");

    std::shared_ptr<SceneNode> find(ObjectId id) const;

    size_t node_count() const { return index_.size(); }

private:
    std::shared_ptr<SceneNode> root_;
    std::unordered_map<ObjectId, std::shared_ptr<SceneNode>, ObjectIdHash> index_;
};

} // namespace aether::perception
