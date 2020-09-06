cmake_minimum_required(VERSION 3.17)

project(arete VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(Boost 1.72.0 REQUIRED COMPONENTS system filesystem regex timer date_time chrono random log)
find_package(Threads REQUIRED)

aux_source_directory(src DIR_SRCS)

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)

configure_file(src/config.h.in config.h)

add_executable(arete ${DIR_SRCS})


target_include_directories(arete 
    PUBLIC 
    "${PROJECT_BINARY_DIR}"
)
set_target_properties(arete PROPERTIES LINK_FLAGS_RELEASE -s)
target_link_libraries(arete ${Boost_LIBRARIES} ${CMAKE_THREAD_LIBS_INIT})