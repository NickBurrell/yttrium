cmake_minimum_required(VERSION 3.16)
project(yttrium)

set(CMAKE_CXX_STANDARD 17)

option(ENABLE_LOT "Enable cross language LTO" ON)
if(ENABLE_LTO)
    include(CheckIPOSupported)
    check_ipo_supported(RESULT supported OUTPUT error)
    if(supported)
        message(STATUS "LTO enabled")
        set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
        add_link_options(-fuse-ld=lld)
    else()
        message(STATUS "LTO not supported: <${error}>")
    endif()
endif()

include_directories(${CMAKE_SOURCE_DIR}/include)

include_directories(lib)

add_subdirectory(${CMAKE_SOURCE_DIR}/yttrium_sys)

include_directories(${YTTRIUM_SYS_SRC_OUT_DIR})

add_library(yttrium SHARED ${CMAKE_SOURCE_DIR}/src/yttrium.cpp)

target_link_libraries(yttrium yttrium_sys)