import pytest

from aether_sdk import AetherClient, SdkError, Transform


def test_connect_and_create_object():
    client = AetherClient.connect("aether://localhost:9000")
    obj = client.create_object()
    assert obj.value == 1


def test_update_object_notifies_subscribers():
    client = AetherClient.connect("aether://localhost:9000")
    obj = client.create_object()

    received = []
    client.subscribe("*", lambda oid, t: received.append((oid, t)))

    new_transform = Transform(position=(1.0, 2.0, 3.0))
    client.update_object(obj, new_transform)

    assert len(received) == 1
    assert received[0][1].position == (1.0, 2.0, 3.0)


def test_operations_fail_after_disconnect():
    client = AetherClient.connect("aether://localhost:9000")
    client.disconnect()
    with pytest.raises(SdkError):
        client.create_object()
