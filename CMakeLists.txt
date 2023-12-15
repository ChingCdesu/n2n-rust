cmake_minimum_required(VERSION 3.15)
project(n2n-rust)

macro (install)
endmacro ()

if (WIN32)
    add_definitions(-DWIN32)
endif()

add_subdirectory(n2n)
