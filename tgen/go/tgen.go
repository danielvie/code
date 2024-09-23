package main

import (
	"fmt"
	"os"
	"path/filepath"
	"text/template"
)

func genFile(filename, templateString string) error {
	filePath := filepath.Dir(filename)
	if err := os.MkdirAll(filePath, os.ModePerm); err != nil {
		return err
	}

	file, err := os.OpenFile(filename, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	tmpl, err := template.New("file").Parse(templateString)
	if err != nil {
		return err
	}

	return tmpl.Execute(file, nil)
}

func genPython() {
	templatePython := 
`if __name__ == "__main__":
    print('bla ble')
`

	templateMakefile := 
`r:
	python main.py
`

	if err := genFile("./main.py", templatePython); err != nil {
		fmt.Println(err)
	}
	if err := genFile("./Makefile", templateMakefile); err != nil {
		fmt.Println(err)
	}
}

func genCpp() {
	templateMain := 
`#include <iostream>

int main(int argc, char const *argv[])
{
	std::cout << "bla ble\n";
	return 0;
}
`

	templateCMake := 
`cmake_minimum_required(VERSION 3.20.0)
project(main)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 23)

file(GLOB sources "src/*.cpp")
add_executable(main ${sources})

target_include_directories(main PRIVATE
  include
)
`

	templateMakefile := 
`all:b
all:r

b:
	cmake -B build
	cmake --build build

r:
	./build/debug/main.exe
`

	if err := genFile("./src/main.cpp", templateMain); err != nil {
		fmt.Println(err)
	}
	if err := genFile("./CMakeLists.txt", templateCMake); err != nil {
		fmt.Println(err)
	}
	if err := genFile("./Makefile", templateMakefile); err != nil {
		fmt.Println(err)
	}
}

func main() {
	if len(os.Args) != 2 {
		fmt.Println("no arguments")
		return
	}

	switch os.Args[1] {
		case "python":
			genPython()
		case "cpp":
			genCpp()
		default:
			fmt.Println("name not known!")
	}
}
