
add_library(sum STATIC sum.c) 

target_include_directories(sum PUBLIC ${CMAKE_SOURCE_DIR}/include)