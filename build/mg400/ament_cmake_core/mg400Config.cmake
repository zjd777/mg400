# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_mg400_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED mg400_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(mg400_FOUND FALSE)
  elseif(NOT mg400_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(mg400_FOUND FALSE)
  endif()
  return()
endif()
set(_mg400_CONFIG_INCLUDED TRUE)

# output package information
if(NOT mg400_FIND_QUIETLY)
  message(STATUS "Found mg400: 1.7.0 (${mg400_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'mg400' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${mg400_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(mg400_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "")
foreach(_extra ${_extras})
  include("${mg400_DIR}/${_extra}")
endforeach()
