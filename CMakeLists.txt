cmake_minimum_required(VERSION 3.17)

project(arete VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(Threads REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(Poco REQUIRED Foundation Net JSON Crypto XML JWT)
find_package(Boost REQUIRED system filesystem regex timer date_time chrono random log)

include_directories(
    /usr/include/hiredis
    /usr/include/mysql
    /usr/include/mysql++
)

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
target_link_libraries(arete 
    ${CMAKE_THREAD_LIBS_INIT}
    ${Boost_LIBRARIES}    
    ${OPENSSL_SSL_LIBRARIES}
    pqxx pq
    mysqlpp mysqlclient
    sqlite3
    hiredis rabbitmq sodium
    Poco::Foundation Poco::Net Poco::JSON Poco::Crypto Poco::XML Poco::JWT
)