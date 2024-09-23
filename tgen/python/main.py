from pathlib import Path
import sys
from jinja2 import Template

def gen_file(filename, template_string: str):
    file = Path(filename)
    file.parent.mkdir(exist_ok=True, parents=True)
    file.touch(exist_ok=True)
    
    template = Template(template_string)
    
    with open(file, 'w') as f:
        f.write(template.render())

def gen_python():
    template_python = '''
if __name__ == "__main__":
    print('bla ble')
'''

    template_makefile = '''
r:
\tpython main.py
'''
    
    gen_file('./output/main.py', template_python)
    gen_file('./output/Makefile', template_makefile)
    
def gen_cpp():
    template_main = '''
#include <iostream>

int main(int argc, char const *argv[])
{
\tstd::cout << "bla ble\\n";
\treturn 0;
}
'''

    template_cmake = '''
cmake_minimum_required(VERSION 3.20.0)
project(main)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 23)

file(GLOB sources "src/*.cpp")
add_executable(main ${sources})

target_include_directories(main PRIVATE
  include
)
'''

    template_makefile = '''
all:b
all:r

b:
\tcmake -B build
\tcmake --build build

r:
\t./build/debug/main.exe
'''
    
    gen_file('./output/src/main.cpp', template_main)
    gen_file('./output/CMakeLists.txt', template_cmake)
    gen_file('./output/Makefile', template_makefile)


def main(name):
    # Store the value of the argument in a variable
    
    if name == 'python':
        gen_python()
    elif name == 'cpp':
        gen_cpp()
    else:
        print('name not known!')


if __name__ == "__main__":
    if len(sys.argv) == 2:
        main(sys.argv[1])
    else:
        print('no arguments')