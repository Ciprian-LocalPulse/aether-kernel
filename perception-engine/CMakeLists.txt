cmake_minimum_required(VERSION 3.16)
project(aether_perception_engine LANGUAGES CXX VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

add_compile_options(-Wall -Wextra -Wpedantic)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

add_library(aether_perception
  src/scene_graph.cpp
  src/sensor_fusion.cpp
  src/slam.cpp
  src/semantic_segmentation.cpp
)
target_include_directories(aether_perception PUBLIC include)

add_executable(perception_demo src/main.cpp)
target_link_libraries(perception_demo PRIVATE aether_perception)

enable_testing()
add_executable(perception_tests tests/scene_graph_tests.cpp)
target_link_libraries(perception_tests PRIVATE aether_perception)
add_test(NAME perception_tests COMMAND perception_tests)

# --- External dependencies (production) ---
# The blueprint (§5.2) specifies NVIDIA Isaac ROS, OpenCV, PCL, and
# DeepStream for the real sensor-fusion/SLAM pipeline. They are NOT
# vendored here (heavy, platform-specific). Uncomment once available:
#
# find_package(OpenCV REQUIRED)
# target_link_libraries(aether_perception PUBLIC ${OpenCV_LIBS})
