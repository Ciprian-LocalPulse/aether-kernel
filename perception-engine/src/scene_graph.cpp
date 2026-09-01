#include "aether_perception/scene_graph.hpp"

namespace aether::perception {

SceneNode::SceneNode(ObjectId id, std::string name)
    : id_(id), name_(std::move(name)) {}

void SceneNode::add_child(std::shared_ptr<SceneNode> child) {
    children_.push_back(std::move(child));
}

SceneGraph::SceneGraph() {
    root_ = std::make_shared<SceneNode>(ObjectId{0, 0}, "root");
    index_[root_->id()] = root_;
}

std::shared_ptr<SceneNode> SceneGraph::get_or_create(ObjectId id, const std::string& name) {
    auto it = index_.find(id);
    if (it != index_.end()) {
        return it->second;
    }
    auto node = std::make_shared<SceneNode>(id, name);
    index_[id] = node;
    root_->add_child(node);
    return node;
}

std::shared_ptr<SceneNode> SceneGraph::find(ObjectId id) const {
    auto it = index_.find(id);
    return it != index_.end() ? it->second : nullptr;
}

} // namespace aether::perception
