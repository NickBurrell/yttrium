function(watch)
    set_property(DIRECTORY
            APPEND
            PROPERTY CMAKE_CONFIGURE_DEPENDS ${ARGV})
endfunction()

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD  cargo build --verbose)
    set(TARGET_DIR "debug")
else()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif()

if(ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-12" "-Clink-arg=-fuse-ld=ldd-12")
endif()

set(YTTRIUM_SYS_SRC_OUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/out")

file(MAKE_DIRECTORY "${YTTRIUM_SYS_SRC_OUT_DIR}")

set(YTTRIUM_SYS_CXX "${YTTRIUM_SYS_SRC_OUT_DIR}/yttrium_sys.cpp")

set(ENV{RUSTFLAGS{ "${RUST_FLAGS}")

add_library(yttrium_sys STATIC ${YTTRIUM_SYS_CXX})


add_custom_command(
        OUTPUT ${YTTRIUM_SYS_CXX}
        COMMAND ${CMAKE_COMMAND} -E env "CARGO_TARGET_DIR=${CMAKE_CURRENT_SOURCE_DIR}" ${CARGO_CMD}
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/cxxbridge/yttrium_sys/src/lib.rs.cc ${YTTRIUM_SYS_CXX}
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/cxxbridge/yttrium_sys/src/lib.rs.h ${YTTRIUM_SYS_SRC_OUT_DIR}/yttrium_sys.h
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

#target_link_libraries(yttrium_sys pthread dl ${YTTRIUM_SYS_LIB})
target_link_libraries(yttrium_sys ${YTTRIUM_SYS_LIB})

get_target_property(YTTRIUM_LIB_NAME yttrium_sys ARCHIVE_OUTPUT_NAME)

message(STATUS ${YTTRIUM_LIB_NAME})

set(YTTRIUM_SYS_LIB "${CMAKE_CURRENT_SOURCE_DIR}/${TARGET_DIR}/${YTTRIUM_LIB_NAME}")

add_test(NAME yttrium_sys_test
        COMMAND cargo test
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})