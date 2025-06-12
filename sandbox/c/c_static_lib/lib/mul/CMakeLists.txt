
add_library(mul STATIC mul.c) 

target_include_directories(mul PUBLIC ${CMAKE_SOURCE_DIR}/include)