// generator.go
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

	templateTaskfile :=
		`version: '3'
tasks:
  default:
    deps:
      - run
  run:
    cmds:
      - python main.py
`

	if err := genFile("./main.py", templatePython); err != nil {
		fmt.Println(err)
	}
	if err := genFile("./Taskfile.yml", templateTaskfile); err != nil {
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

	templateTaskfile :=
		`version: '3'
tasks:
  default:
    deps:
      - run

  build:
    cmds:
      - cmake -B build
      - cmake --build build

  run:
    deps:
      - build
    cmds:
      - ./build/debug/main.exe
`

	if err := genFile("./src/main.cpp", templateMain); err != nil {
		fmt.Println(err)
	}
	if err := genFile("./CMakeLists.txt", templateCMake); err != nil {
		fmt.Println(err)
	}
	if err := genFile("./Taskfile.yml", templateTaskfile); err != nil {
		fmt.Println(err)
	}
}
