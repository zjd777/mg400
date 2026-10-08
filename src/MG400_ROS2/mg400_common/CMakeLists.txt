cmake_minimum_required(VERSION 3.8)
project(mg400_common)

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 17)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_CXX_EXTENSIONS OFF)
endif()

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake_auto REQUIRED)
ament_auto_find_build_dependencies()

file(GLOB src_files RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} src/*.cpp)
set(TARGET ${PROJECT_NAME})
ament_auto_add_library(${TARGET} SHARED ${src_files})

if(BUILD_TESTING)
  find_package(ament_lint_auto REQUIRED)
  find_package(ament_cmake_gtest REQUIRED)
  set(ament_cmake_cpplint_FOUND TRUE)
  ament_lint_auto_find_test_dependencies()

  set(TARGET test_mg400_kinematics)
  ament_add_gtest(${TARGET} test/${TARGET}.cpp)
  if(TARGET ${TARGET})
    target_include_directories(${TARGET} PRIVATE include)
    target_link_libraries(${TARGET} ${PROJECT_NAME})
  endif()
endif()

ament_auto_package()
