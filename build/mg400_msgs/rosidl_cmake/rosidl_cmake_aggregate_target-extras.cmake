# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target mg400_msgs::mg400_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${mg400_msgs_TARGETS}.
if(mg400_msgs_TARGETS AND NOT TARGET mg400_msgs::mg400_msgs)
  add_library(mg400_msgs::mg400_msgs INTERFACE IMPORTED)
  set_target_properties(mg400_msgs::mg400_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${mg400_msgs_TARGETS}")
endif()
