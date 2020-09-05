cmake_minimum_required(VERSION 3.17)

project(arete VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
# set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -s")
# set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -s")

aux_source_directory(src DIR_SRCS)

configure_file(src/config.h.in config.h)

add_executable(arete ${DIR_SRCS})


target_include_directories(arete 
    PUBLIC 
    "${PROJECT_BINARY_DIR}"
)
set_target_properties(arete PROPERTIES LINK_FLAGS_RELEASE -s)