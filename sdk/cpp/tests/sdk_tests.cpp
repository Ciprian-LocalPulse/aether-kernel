#include "aether/sdk.hpp"
#include <cassert>
#include <iostream>

using namespace aether::sdk;

int main() {
    auto client = AetherClient::connect("aether://localhost:9000");
    auto obj = client.create_object();
    assert(obj.value == 1);

    bool notified = false;
    client.subscribe("*", [&](ObjectId, const Transform&) { notified = true; });
    client.update_object(obj, Transform{{1, 2, 3}, {0, 0, 0, 1}});
    assert(notified);

    client.disconnect();
    bool threw = false;
    try {
        client.create_object();
    } catch (const SdkError&) {
        threw = true;
    }
    assert(threw);

    std::cout << "All C++ SDK tests passed.\n";
    return 0;
}
