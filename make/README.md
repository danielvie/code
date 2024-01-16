CMakeLists.txt
```cmake
cmake_minimum_required(VERSION 3.20.0)
project(main)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 23)

include_directories(src)

file(GLOB my_sources CONFIGURE_DEPENDS "src/*.cpp")
add_executable(main ${my_sources})
```

Makefile
```makefile
all: b
all: r

b:
	cmake -B build
	cmake --build build
	
r:
	./build/Main

c:
	rm -rf build
```
