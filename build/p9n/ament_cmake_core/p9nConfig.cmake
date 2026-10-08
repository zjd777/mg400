# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_p9n_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED p9n_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(p9n_FOUND FALSE)
  elseif(NOT p9n_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(p9n_FOUND FALSE)
  endif()
  return()
endif()
set(_p9n_CONFIG_INCLUDED TRUE)

# output package information
if(NOT p9n_FIND_QUIETLY)
  message(STATUS "Found p9n: 1.1.2 (${p9n_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'p9n' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${p9n_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(p9n_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "")
foreach(_extra ${_extras})
  include("${p9n_DIR}/${_extra}")
endforeach()
