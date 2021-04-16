cmake_minimum_required(VERSION 3.10 FATAL_ERROR)

project(LinuxSourceMirror)

###################################System#####################################
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED on)

# Build type
if (NOT CMAKE_BUILD_TYPE)
	set (CMAKE_BUILD_TYPE "Debug")

endif ()

if (NOT UNIX)
    message (FATAL_ERROR        "Operation system is not supported.")

endif ()

set (CMAKE_BUILD_TYPE "${CMAKE_BUILD_TYPE}"  CACHE STRING "${CMAKE_BUILD_TYPE}" FORCE)

if (GENERATOR_IS_MULTI_CONFIG OR CMAKE_CONFIGURATION_TYPES)
    set (CMAKE_CONFIGURATION_TYPES "${CMAKE_BUILD_TYPE}" CACHE STRING "${CMAKE_BUILD_TYPE}" FORCE)

endif()

message (STATUS "Build Type - ${CMAKE_BUILD_TYPE}.")
message (STATUS "Host System - ${CMAKE_HOST_SYSTEM_NAME}.")
message (STATUS "Host Processor - ${CMAKE_HOST_SYSTEM_PROCESSOR}.")
message (STATUS "Target System - ${CMAKE_SYSTEM_NAME}.")
message (STATUS "Target Processor - ${CMAKE_SYSTEM_PROCESSOR}.")

# Environment.
include_directories ("${CMAKE_CURRENT_SOURCE_DIR}/include")

# Options
add_compile_options("-fPIC" "-fexceptions" "-g" "-Wall")

# Output path.
set (OUTPUT_SUB_DIR "${CMAKE_BUILD_TYPE}/${CMAKE_SYSTEM_NAME}/${CMAKE_SYSTEM_PROCESSOR}")

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY	"${CMAKE_CURRENT_SOURCE_DIR}/lib/${OUTPUT_SUB_DIR}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY	"${CMAKE_CURRENT_SOURCE_DIR}/lib/${OUTPUT_SUB_DIR}")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY	"${CMAKE_CURRENT_SOURCE_DIR}/bin/${OUTPUT_SUB_DIR}")

link_directories ("${CMAKE_LIBRARY_OUTPUT_DIRECTORY}")

# Platform options.
# Debug/Release.
# Enable address sanitizer in debug mode if possible.
if ("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
	if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang"
			OR "${CMAKE_CXX_COMPILER_ID}" STREQUAL "AppleClang"
			OR "${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
		# Enable address sanitizer.
		add_compile_options (-fno-omit-frame-pointer -fsanitize=address)
		add_link_options (-fno-omit-frame-pointer -fsanitize=address)
		message (STATUS "Address sanitizer is enabled.")

	endif()

endif ()

###################################Dependencies################################

function (find_system_library)
    cmake_parse_arguments (ARGVAL "" "NAME" "HEADERS;LIBRARIES" ${ARGN})

    # Search header files.
    set (_dirs_found "")
    foreach (header IN LISTS ARGVAL_HEADERS)
        find_path (_ret  "${header}")
        if(_ret)
            message (STATUS "File \"${_ret}/${header}\" found.")
            list (APPEND _dirs_found "${_ret}")
            unset (_ret  CACHE)

        else ()
            unset (_ret  CACHE)
            unset (_dirs_found)
            message (FATAL_ERROR "Failed to find header file \"${header}\".")

        endif()

    endforeach ()

    set (${ARGVAL_NAME}_INCLUDE_DIRS "${_dirs_found}" CACHE STRING INTERNAL)
    unset (_dirs_found)

    # Search library files.
    set (_libs_found "")
    foreach (lib IN LISTS ARGVAL_LIBRARIES)
        find_library (_ret  "${lib}")
        if(_ret)
            message (STATUS "Library \"${_ret}\" found.")
            list (APPEND _libs_found "${_ret}")
            unset (_ret  CACHE)

        else ()
            unset (_ret  CACHE)
            unset (_libs_found)
            message (FATAL_ERROR "Failed to find library \"${lib}\".")

        endif()

    endforeach ()

    set (${ARGVAL_NAME}_LIBS "${_libs_found}" CACHE STRING INTERNAL)
    unset (_libs_found)

endfunction ()

# Doxygen
#find_package (Doxygen REQUIRED)

# Rapidjson
find_system_library (
    NAME    "Rapidjson"
   HEADERS  "rapidjson/document.h")

include_directories (${Rapidjson_INCLUDE_DIRS})

#################################Targets#######################################

# Common
set (COMMON_LIB "common" CACHE INTERNAL  "COMMON_LIB")
file (GLOB_RECURSE COMMON_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/source/common/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/common/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/common/*.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/common/*.C"
    )
add_library (${COMMON_LIB}  STATIC  
    ${COMMON_SRC})

# Config
set (CONFIG_LIB "config" CACHE INTERNAL  "CONFIG_LIB")
file (GLOB_RECURSE CONFIG_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/source/config/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/config/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/config/*.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/config/*.C"
    )
add_library (${CONFIG_LIB}  STATIC  
    ${CONFIG_SRC})
add_dependencies (${CONFIG_LIB} 
    ${COMMON_LIB})

# Logger
set (LOGGER_LIB "logger" CACHE INTERNAL  "LOGGER_LIB")
file (GLOB_RECURSE LOGGER_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/source/logger/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/logger/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/logger/*.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/logger/*.C"
    )
add_library (${LOGGER_LIB}  STATIC  
    ${LOGGER_SRC})
add_dependencies (${LOGGER_LIB} 
    ${CONFIG_LIB}
    ${COMMON_LIB})

# Plugin
set (PLUGIN_LIB "plugin" CACHE INTERNAL  "PLUGIN_LIB")
file (GLOB_RECURSE PLUGIN_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/source/plugin/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/plugin/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/plugin/*.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/plugin/*.C"
    )
add_library (${PLUGIN_LIB}  STATIC  
    ${PLUGIN_SRC})
add_dependencies (${PLUGIN_LIB} 
    ${LOGGER_LIB}
    ${CONFIG_LIB}
    ${COMMON_LIB})

# Core
file (GLOB_RECURSE CORE_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/source/core/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/core/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/core/*.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/core/*.C"
    )
add_executable (${PROJECT_NAME}     ${CORE_SRC})

add_dependencies (${PROJECT_NAME}
    ${PLUGIN_LIB} 
    ${LOGGER_LIB}
    ${CONFIG_LIB}
    ${COMMON_LIB})

target_link_libraries (${PROJECT_NAME}
    ${PLUGIN_LIB} 
    ${LOGGER_LIB}
    ${CONFIG_LIB}
    ${COMMON_LIB})

# Client Plugins

# Server Plugins

