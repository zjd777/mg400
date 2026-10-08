# generated from
# ament_cmake_core/cmake/symlink_install/ament_cmake_symlink_install.cmake.in

# create empty symlink install manifest before starting install step
file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/symlink_install_manifest.txt")

#
# Reimplement CMake install(DIRECTORY) command to use symlinks instead of
# copying resources.
#
# :param cmake_current_source_dir: The CMAKE_CURRENT_SOURCE_DIR when install
#   was invoked
# :type cmake_current_source_dir: string
# :param ARGN: the same arguments as the CMake install command.
# :type ARGN: various
#
function(ament_cmake_symlink_install_directory cmake_current_source_dir)
  cmake_parse_arguments(ARG "OPTIONAL" "DESTINATION" "DIRECTORY;PATTERN;PATTERN_EXCLUDE" ${ARGN})
  if(ARG_UNPARSED_ARGUMENTS)
    message(FATAL_ERROR "ament_cmake_symlink_install_directory() called with "
      "unused/unsupported arguments: ${ARG_UNPARSED_ARGUMENTS}")
  endif()

  # make destination absolute path and ensure that it exists
  if(NOT IS_ABSOLUTE "${ARG_DESTINATION}")
    set(ARG_DESTINATION "/root/ros2_ws/install/mg400_msgs/${ARG_DESTINATION}")
  endif()
  if(NOT EXISTS "${ARG_DESTINATION}")
    file(MAKE_DIRECTORY "${ARG_DESTINATION}")
  endif()

  # default pattern to include
  if(NOT ARG_PATTERN)
    set(ARG_PATTERN "*")
  endif()

  # iterate over directories
  foreach(dir ${ARG_DIRECTORY})
    # make dir an absolute path
    if(NOT IS_ABSOLUTE "${dir}")
      set(dir "${cmake_current_source_dir}/${dir}")
    endif()

    if(EXISTS "${dir}")
      # if directory has no trailing slash
      # append folder name to destination
      set(destination "${ARG_DESTINATION}")
      string(LENGTH "${dir}" length)
      math(EXPR offset "${length} - 1")
      string(SUBSTRING "${dir}" ${offset} 1 dir_last_char)
      if(NOT dir_last_char STREQUAL "/")
        get_filename_component(destination_name "${dir}" NAME)
        set(destination "${destination}/${destination_name}")
      else()
        # remove trailing slash
        string(SUBSTRING "${dir}" 0 ${offset} dir)
      endif()
      
      # Create destination directory.
      # This does *not* solve the problem of empty directories WITHIN the install tree,
      # but does make sure that the top-level directory specified by the caller gets created.
      file(MAKE_DIRECTORY "${destination}")

      # glob recursive files
      set(relative_files "")
      foreach(pattern ${ARG_PATTERN})
        file(
          GLOB_RECURSE
          include_files
          RELATIVE "${dir}"
          "${dir}/${pattern}"
        )
        if(NOT include_files STREQUAL "")
          list(APPEND relative_files ${include_files})
        endif()
      endforeach()
      foreach(pattern ${ARG_PATTERN_EXCLUDE})
        file(
          GLOB_RECURSE
          exclude_files
          RELATIVE "${dir}"
          "${dir}/${pattern}"
        )
        if(NOT exclude_files STREQUAL "")
          list(REMOVE_ITEM relative_files ${exclude_files})
        endif()
      endforeach()
      list(SORT relative_files)

      foreach(relative_file ${relative_files})
        set(absolute_file "${dir}/${relative_file}")
        # determine link name for file including destination path
        set(symlink "${destination}/${relative_file}")

        # ensure that destination exists
        get_filename_component(symlink_dir "${symlink}" PATH)
        if(NOT EXISTS "${symlink_dir}")
          file(MAKE_DIRECTORY "${symlink_dir}")
        endif()

        _ament_cmake_symlink_install_create_symlink("${absolute_file}" "${symlink}")
      endforeach()
    else()
      if(NOT ARG_OPTIONAL)
        message(FATAL_ERROR
          "ament_cmake_symlink_install_directory() can't find '${dir}'")
      endif()
    endif()
  endforeach()
endfunction()

#
# Reimplement CMake install(FILES) command to use symlinks instead of copying
# resources.
#
# :param cmake_current_source_dir: The CMAKE_CURRENT_SOURCE_DIR when install
#   was invoked
# :type cmake_current_source_dir: string
# :param ARGN: the same arguments as the CMake install command.
# :type ARGN: various
#
function(ament_cmake_symlink_install_files cmake_current_source_dir)
  cmake_parse_arguments(ARG "OPTIONAL" "DESTINATION;RENAME" "FILES" ${ARGN})
  if(ARG_UNPARSED_ARGUMENTS)
    message(FATAL_ERROR "ament_cmake_symlink_install_files() called with "
      "unused/unsupported arguments: ${ARG_UNPARSED_ARGUMENTS}")
  endif()

  # make destination an absolute path and ensure that it exists
  if(NOT IS_ABSOLUTE "${ARG_DESTINATION}")
    set(ARG_DESTINATION "/root/ros2_ws/install/mg400_msgs/${ARG_DESTINATION}")
  endif()
  if(NOT EXISTS "${ARG_DESTINATION}")
    file(MAKE_DIRECTORY "${ARG_DESTINATION}")
  endif()

  if(ARG_RENAME)
    list(LENGTH ARG_FILES file_count)
    if(NOT file_count EQUAL 1)
    message(FATAL_ERROR "ament_cmake_symlink_install_files() called with "
      "RENAME argument but not with a single file")
    endif()
  endif()

  # iterate over files
  foreach(file ${ARG_FILES})
    # make file an absolute path
    if(NOT IS_ABSOLUTE "${file}")
      set(file "${cmake_current_source_dir}/${file}")
    endif()

    if(EXISTS "${file}")
      # determine link name for file including destination path
      get_filename_component(filename "${file}" NAME)
      if(NOT ARG_RENAME)
        set(symlink "${ARG_DESTINATION}/${filename}")
      else()
        set(symlink "${ARG_DESTINATION}/${ARG_RENAME}")
      endif()
      _ament_cmake_symlink_install_create_symlink("${file}" "${symlink}")
    else()
      if(NOT ARG_OPTIONAL)
        message(FATAL_ERROR
          "ament_cmake_symlink_install_files() can't find '${file}'")
      endif()
    endif()
  endforeach()
endfunction()

#
# Reimplement CMake install(PROGRAMS) command to use symlinks instead of copying
# resources.
#
# :param cmake_current_source_dir: The CMAKE_CURRENT_SOURCE_DIR when install
#   was invoked
# :type cmake_current_source_dir: string
# :param ARGN: the same arguments as the CMake install command.
# :type ARGN: various
#
function(ament_cmake_symlink_install_programs cmake_current_source_dir)
  cmake_parse_arguments(ARG "OPTIONAL" "DESTINATION" "PROGRAMS" ${ARGN})
  if(ARG_UNPARSED_ARGUMENTS)
    message(FATAL_ERROR "ament_cmake_symlink_install_programs() called with "
      "unused/unsupported arguments: ${ARG_UNPARSED_ARGUMENTS}")
  endif()

  # make destination an absolute path and ensure that it exists
  if(NOT IS_ABSOLUTE "${ARG_DESTINATION}")
    set(ARG_DESTINATION "/root/ros2_ws/install/mg400_msgs/${ARG_DESTINATION}")
  endif()
  if(NOT EXISTS "${ARG_DESTINATION}")
    file(MAKE_DIRECTORY "${ARG_DESTINATION}")
  endif()

  # iterate over programs
  foreach(file ${ARG_PROGRAMS})
    # make file an absolute path
    if(NOT IS_ABSOLUTE "${file}")
      set(file "${cmake_current_source_dir}/${file}")
    endif()

    if(EXISTS "${file}")
      # determine link name for file including destination path
      get_filename_component(filename "${file}" NAME)
      set(symlink "${ARG_DESTINATION}/${filename}")
      _ament_cmake_symlink_install_create_symlink("${file}" "${symlink}")
    else()
      if(NOT ARG_OPTIONAL)
        message(FATAL_ERROR
          "ament_cmake_symlink_install_programs() can't find '${file}'")
      endif()
    endif()
  endforeach()
endfunction()

#
# Reimplement CMake install(TARGETS) command to use symlinks instead of copying
# resources.
#
# :param TARGET_FILES: the absolute files, replacing the name of targets passed
#   in as TARGETS
# :type TARGET_FILES: list of files
# :param ARGN: the same arguments as the CMake install command except that
#   keywords identifying the kind of type and the DESTINATION keyword must be
#   joined with an underscore, e.g. ARCHIVE_DESTINATION.
# :type ARGN: various
#
function(ament_cmake_symlink_install_targets)
  cmake_parse_arguments(ARG "OPTIONAL" "ARCHIVE_DESTINATION;DESTINATION;LIBRARY_DESTINATION;RUNTIME_DESTINATION"
    "TARGETS;TARGET_FILES" ${ARGN})
  if(ARG_UNPARSED_ARGUMENTS)
    message(FATAL_ERROR "ament_cmake_symlink_install_targets() called with "
      "unused/unsupported arguments: ${ARG_UNPARSED_ARGUMENTS}")
  endif()

  # iterate over target files
  foreach(file ${ARG_TARGET_FILES})
    if(NOT IS_ABSOLUTE "${file}")
      message(FATAL_ERROR "ament_cmake_symlink_install_targets() target file "
        "'${file}' must be an absolute path")
    endif()

    # determine destination of file based on extension
    set(destination "")
    get_filename_component(fileext "${file}" EXT)
    if(fileext STREQUAL ".a" OR fileext STREQUAL ".lib")
      set(destination "${ARG_ARCHIVE_DESTINATION}")
    elseif(fileext STREQUAL ".dylib" OR fileext MATCHES "\\.so(\\.[0-9]+)?(\\.[0-9]+)?(\\.[0-9]+)?$")
      set(destination "${ARG_LIBRARY_DESTINATION}")
    elseif(fileext STREQUAL "" OR fileext STREQUAL ".dll" OR fileext STREQUAL ".exe")
      set(destination "${ARG_RUNTIME_DESTINATION}")
    endif()
    if(destination STREQUAL "")
      set(destination "${ARG_DESTINATION}")
    endif()

    # make destination an absolute path and ensure that it exists
    if(NOT IS_ABSOLUTE "${destination}")
      set(destination "/root/ros2_ws/install/mg400_msgs/${destination}")
    endif()
    if(NOT EXISTS "${destination}")
      file(MAKE_DIRECTORY "${destination}")
    endif()

    if(EXISTS "${file}")
      # determine link name for file including destination path
      get_filename_component(filename "${file}" NAME)
      set(symlink "${destination}/${filename}")
      _ament_cmake_symlink_install_create_symlink("${file}" "${symlink}")
    else()
      if(NOT ARG_OPTIONAL)
        message(FATAL_ERROR
          "ament_cmake_symlink_install_targets() can't find '${file}'")
      endif()
    endif()
  endforeach()
endfunction()

function(_ament_cmake_symlink_install_create_symlink absolute_file symlink)
  # register symlink for being removed during install step
  file(APPEND "${CMAKE_CURRENT_BINARY_DIR}/symlink_install_manifest.txt"
    "${symlink}\n")

  # avoid any work if correct symlink is already in place
  if(EXISTS "${symlink}" AND IS_SYMLINK "${symlink}")
    get_filename_component(destination "${symlink}" REALPATH)
    get_filename_component(real_absolute_file "${absolute_file}" REALPATH)
    if(destination STREQUAL real_absolute_file)
      message(STATUS "Up-to-date symlink: ${symlink}")
      return()
    endif()
  endif()

  message(STATUS "Symlinking: ${symlink}")
  if(EXISTS "${symlink}" OR IS_SYMLINK "${symlink}")
    file(REMOVE "${symlink}")
  endif()

  execute_process(
    COMMAND "/usr/bin/cmake" "-E" "create_symlink"
      "${absolute_file}"
      "${symlink}"
  )
  # the CMake command does not provide a return code so check manually
  if(NOT EXISTS "${symlink}" OR NOT IS_SYMLINK "${symlink}")
    get_filename_component(destination "${symlink}" REALPATH)
    message(FATAL_ERROR
      "Could not create symlink '${symlink}' pointing to '${absolute_file}'")
  endif()
endfunction()

# end of template

message(STATUS "Execute custom install script")

# begin of custom install code

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/rosidl_interfaces/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/rosidl_interfaces")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/rosidl_interfaces/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/rosidl_interfaces")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_c/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.h")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_c/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.h")

# install(FILES "/opt/ros/humble/lib/python3.10/site-packages/ament_package/template/environment_hook/library_path.sh" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/opt/ros/humble/lib/python3.10/site-packages/ament_package/template/environment_hook/library_path.sh" "DESTINATION" "share/mg400_msgs/environment")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/library_path.dsv" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/library_path.dsv" "DESTINATION" "share/mg400_msgs/environment")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_fastrtps_c/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN_EXCLUDE" "*.cpp")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_fastrtps_c/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN_EXCLUDE" "*.cpp")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_cpp/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.hpp")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_cpp/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.hpp")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_fastrtps_cpp/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN_EXCLUDE" "*.cpp")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_fastrtps_cpp/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN_EXCLUDE" "*.cpp")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_introspection_c/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.h")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_introspection_c/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.h")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_introspection_cpp/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.hpp")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_typesupport_introspection_cpp/mg400_msgs/" "DESTINATION" "include/mg400_msgs/mg400_msgs" "PATTERN" "*.hpp")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/pythonpath.sh" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/pythonpath.sh" "DESTINATION" "share/mg400_msgs/environment")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/pythonpath.dsv" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/pythonpath.dsv" "DESTINATION" "share/mg400_msgs/environment")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/ament_cmake_python/mg400_msgs/mg400_msgs.egg-info/" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs-1.7.0-py3.10.egg-info")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/ament_cmake_python/mg400_msgs/mg400_msgs.egg-info/" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs-1.7.0-py3.10.egg-info")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_py/mg400_msgs/" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs" "PATTERN_EXCLUDE" "*.pyc" "PATTERN_EXCLUDE" "__pycache__")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_py/mg400_msgs/" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs" "PATTERN_EXCLUDE" "*.pyc" "PATTERN_EXCLUDE" "__pycache__")

# install("TARGETS" "mg400_msgs__rosidl_typesupport_fastrtps_c__pyext" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs")
include("/root/ros2_ws/build/mg400_msgs/ament_cmake_symlink_install_targets_0_${CMAKE_INSTALL_CONFIG_NAME}.cmake")

# install("TARGETS" "mg400_msgs__rosidl_typesupport_introspection_c__pyext" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs")
include("/root/ros2_ws/build/mg400_msgs/ament_cmake_symlink_install_targets_1_${CMAKE_INSTALL_CONFIG_NAME}.cmake")

# install("TARGETS" "mg400_msgs__rosidl_typesupport_c__pyext" "DESTINATION" "local/lib/python3.10/dist-packages/mg400_msgs")
include("/root/ros2_ws/build/mg400_msgs/ament_cmake_symlink_install_targets_2_${CMAKE_INSTALL_CONFIG_NAME}.cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/rust_packages/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/rust_packages")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/rust_packages/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/rust_packages")

# install(DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_rs/mg400_msgs/rust" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_directory("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" DIRECTORY "/root/ros2_ws/build/mg400_msgs/rosidl_generator_rs/mg400_msgs/rust" "DESTINATION" "share/mg400_msgs")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/Arch.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/Arch.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/CollisionLevel.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/CollisionLevel.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/Command.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/Command.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DIIndex.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DIIndex.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DOIndex.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DOIndex.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DOStatus.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DOStatus.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DistanceMode.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/DistanceMode.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/ErrorID.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/ErrorID.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/IDArray.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/IDArray.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/JointMovJ.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/JointMovJ.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovJ.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovJ.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovJIO.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovJIO.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovL.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovL.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovLIO.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MovLIO.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MoveJog.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/MoveJog.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/RobotMode.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/RobotMode.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/Tool.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/Tool.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/ToolDIIndex.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/ToolDIIndex.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/ToolDOIndex.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/ToolDOIndex.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/User.idl" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/msg/User.idl" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/AccJ.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/AccJ.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/AccL.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/AccL.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/Arch.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/Arch.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/CP.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/CP.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ClearError.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ClearError.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/DI.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/DI.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/DO.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/DO.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/DisableRobot.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/DisableRobot.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/EmergencyStop.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/EmergencyStop.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/EnableRobot.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/EnableRobot.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/GetAngle.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/GetAngle.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/GetErrorID.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/GetErrorID.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/GetPose.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/GetPose.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/InverseSolution.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/InverseSolution.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/JointMovJ.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/JointMovJ.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/MoveJog.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/MoveJog.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/PayLoad.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/PayLoad.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/PositiveSolution.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/PositiveSolution.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ResetRobot.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ResetRobot.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/RobotMode.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/RobotMode.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SetCollisionLevel.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SetCollisionLevel.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SpeedFactor.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SpeedFactor.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SpeedJ.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SpeedJ.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SpeedL.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/SpeedL.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/Tool.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/Tool.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ToolDI.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ToolDI.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ToolDOExecute.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/ToolDOExecute.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/User.idl" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/srv/User.idl" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/CommandQueue.idl" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/CommandQueue.idl" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/JointMovJ.idl" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/JointMovJ.idl" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovJ.idl" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovJ.idl" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovJIO.idl" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovJIO.idl" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovL.idl" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovL.idl" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovLIO.idl" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_adapter/mg400_msgs/action/MovLIO.idl" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/Arch.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/Arch.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/CollisionLevel.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/CollisionLevel.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/Command.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/Command.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DIIndex.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DIIndex.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DOIndex.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DOIndex.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DOStatus.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DOStatus.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DistanceMode.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/DistanceMode.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/ErrorID.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/ErrorID.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/IDArray.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/IDArray.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/JointMovJ.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/JointMovJ.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovJ.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovJ.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovJIO.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovJIO.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovL.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovL.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovLIO.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MovLIO.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MoveJog.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/MoveJog.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/RobotMode.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/RobotMode.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/Tool.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/Tool.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/ToolDIIndex.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/ToolDIIndex.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/ToolDOIndex.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/ToolDOIndex.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/User.msg" "DESTINATION" "share/mg400_msgs/msg")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/msg/User.msg" "DESTINATION" "share/mg400_msgs/msg")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/AccJ.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/AccJ.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccJ_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccJ_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccJ_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccJ_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/AccL.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/AccL.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccL_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccL_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccL_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/AccL_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/Arch.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/Arch.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Arch_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Arch_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Arch_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Arch_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/CP.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/CP.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/CP_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/CP_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/CP_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/CP_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ClearError.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ClearError.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ClearError_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ClearError_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ClearError_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ClearError_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/DI.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/DI.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DI_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DI_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DI_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DI_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/DO.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/DO.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DO_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DO_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DO_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DO_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/DisableRobot.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/DisableRobot.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DisableRobot_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DisableRobot_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DisableRobot_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/DisableRobot_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/EmergencyStop.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/EmergencyStop.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EmergencyStop_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EmergencyStop_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EmergencyStop_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EmergencyStop_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/EnableRobot.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/EnableRobot.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EnableRobot_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EnableRobot_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EnableRobot_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/EnableRobot_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/GetAngle.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/GetAngle.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetAngle_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetAngle_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetAngle_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetAngle_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/GetErrorID.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/GetErrorID.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetErrorID_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetErrorID_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetErrorID_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetErrorID_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/GetPose.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/GetPose.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetPose_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetPose_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetPose_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/GetPose_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/InverseSolution.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/InverseSolution.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/InverseSolution_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/InverseSolution_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/InverseSolution_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/InverseSolution_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/JointMovJ.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/JointMovJ.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/JointMovJ_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/JointMovJ_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/JointMovJ_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/JointMovJ_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/MoveJog.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/MoveJog.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/MoveJog_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/MoveJog_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/MoveJog_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/MoveJog_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/PayLoad.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/PayLoad.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PayLoad_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PayLoad_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PayLoad_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PayLoad_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/PositiveSolution.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/PositiveSolution.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PositiveSolution_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PositiveSolution_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PositiveSolution_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/PositiveSolution_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ResetRobot.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ResetRobot.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ResetRobot_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ResetRobot_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ResetRobot_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ResetRobot_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/RobotMode.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/RobotMode.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/RobotMode_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/RobotMode_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/RobotMode_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/RobotMode_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SetCollisionLevel.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SetCollisionLevel.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SetCollisionLevel_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SetCollisionLevel_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SetCollisionLevel_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SetCollisionLevel_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SpeedFactor.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SpeedFactor.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedFactor_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedFactor_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedFactor_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedFactor_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SpeedJ.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SpeedJ.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedJ_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedJ_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedJ_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedJ_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SpeedL.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/SpeedL.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedL_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedL_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedL_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/SpeedL_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/Tool.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/Tool.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Tool_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Tool_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Tool_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/Tool_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ToolDI.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ToolDI.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDI_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDI_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDI_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDI_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ToolDOExecute.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/ToolDOExecute.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDOExecute_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDOExecute_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDOExecute_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/ToolDOExecute_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/User.srv" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/srv/User.srv" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/User_Request.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/User_Request.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/User_Response.msg" "DESTINATION" "share/mg400_msgs/srv")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/srv/User_Response.msg" "DESTINATION" "share/mg400_msgs/srv")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/CommandQueue.action" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/CommandQueue.action" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/JointMovJ.action" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/JointMovJ.action" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovJ.action" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovJ.action" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovJIO.action" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovJIO.action" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovL.action" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovL.action" "DESTINATION" "share/mg400_msgs/action")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovLIO.action" "DESTINATION" "share/mg400_msgs/action")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/action/MovLIO.action" "DESTINATION" "share/mg400_msgs/action")

# install("TARGETS" "ARCHIVE_DESTINATION" "lib" "LIBRARY_DESTINATION" "lib" "RUNTIME_DESTINATION" "bin")
include("/root/ros2_ws/build/mg400_msgs/ament_cmake_symlink_install_targets_3_${CMAKE_INSTALL_CONFIG_NAME}.cmake")

# install("TARGETS" "DESTINATION" "lib/mg400_msgs")
include("/root/ros2_ws/build/mg400_msgs/ament_cmake_symlink_install_targets_4_${CMAKE_INSTALL_CONFIG_NAME}.cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/package_run_dependencies/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/package_run_dependencies")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/package_run_dependencies/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/package_run_dependencies")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/parent_prefix_path/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/parent_prefix_path")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/parent_prefix_path/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/parent_prefix_path")

# install(FILES "/opt/ros/humble/share/ament_cmake_core/cmake/environment_hooks/environment/ament_prefix_path.sh" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/opt/ros/humble/share/ament_cmake_core/cmake/environment_hooks/environment/ament_prefix_path.sh" "DESTINATION" "share/mg400_msgs/environment")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/ament_prefix_path.dsv" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/ament_prefix_path.dsv" "DESTINATION" "share/mg400_msgs/environment")

# install(FILES "/opt/ros/humble/share/ament_cmake_core/cmake/environment_hooks/environment/path.sh" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/opt/ros/humble/share/ament_cmake_core/cmake/environment_hooks/environment/path.sh" "DESTINATION" "share/mg400_msgs/environment")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/path.dsv" "DESTINATION" "share/mg400_msgs/environment")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/path.dsv" "DESTINATION" "share/mg400_msgs/environment")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.bash" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.bash" "DESTINATION" "share/mg400_msgs")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.sh" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.sh" "DESTINATION" "share/mg400_msgs")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.zsh" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.zsh" "DESTINATION" "share/mg400_msgs")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.dsv" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/local_setup.dsv" "DESTINATION" "share/mg400_msgs")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/package.dsv" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_environment_hooks/package.dsv" "DESTINATION" "share/mg400_msgs")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/packages/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/packages")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_index/share/ament_index/resource_index/packages/mg400_msgs" "DESTINATION" "share/ament_index/resource_index/packages")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_dependencies/ament_cmake_export_dependencies-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_dependencies/ament_cmake_export_dependencies-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_include_directories/ament_cmake_export_include_directories-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_include_directories/ament_cmake_export_include_directories-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_libraries/ament_cmake_export_libraries-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_libraries/ament_cmake_export_libraries-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_targets/ament_cmake_export_targets-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_export_targets/ament_cmake_export_targets-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake_export_typesupport_targets-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake_export_typesupport_targets-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake_export_typesupport_libraries-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake_export_typesupport_libraries-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake_aggregate_target-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/rosidl_cmake/rosidl_cmake_aggregate_target-extras.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_core/mg400_msgsConfig.cmake" "/root/ros2_ws/build/mg400_msgs/ament_cmake_core/mg400_msgsConfig-version.cmake" "DESTINATION" "share/mg400_msgs/cmake")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/build/mg400_msgs/ament_cmake_core/mg400_msgsConfig.cmake" "/root/ros2_ws/build/mg400_msgs/ament_cmake_core/mg400_msgsConfig-version.cmake" "DESTINATION" "share/mg400_msgs/cmake")

# install(FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/package.xml" "DESTINATION" "share/mg400_msgs")
ament_cmake_symlink_install_files("/root/ros2_ws/src/MG400_ROS2/mg400_msgs" FILES "/root/ros2_ws/src/MG400_ROS2/mg400_msgs/package.xml" "DESTINATION" "share/mg400_msgs")
