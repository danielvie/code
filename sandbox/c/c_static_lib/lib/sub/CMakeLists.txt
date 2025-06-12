
add_library(sub STATIC sub.c) 

target_include_directories(sub PUBLIC ${CMAKE_SOURCE_DIR}/include)