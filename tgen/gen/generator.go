// generator.go
package gen

import (
	"embed"
	"fmt"
	"os"
	"path/filepath"
	"text/template"
)

//go:embed templates/*
//go:embed templates/python/.python-version
var templates embed.FS

func GenFile(filename, templatePath string) error {
	// ensure the directory exists
	filePath := filepath.Dir(filename)
	if err := os.MkdirAll(filePath, os.ModePerm); err != nil {
		return err
	}

	// read the template file content from disk
	t, err := template.ParseFS(templates, templatePath)
	if err != nil {
		return err
	}

	// open or create the file
	file, err := os.OpenFile(filename, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	// execute the template and write the output to the file
	return t.Execute(file, nil)
}

func GenPython() error {

	if err := GenFile("./main.py", "templates/python/main.py"); err != nil {
		fmt.Println(err)
	}

	if err := GenFile("./.python-version", "templates/python/.python-version"); err != nil {
		fmt.Println(err)
	}

	if err := GenFile("./pyproject.toml", "templates/python/pyproject.toml"); err != nil {
		fmt.Println(err)
	}

	if err := GenFile("./Taskfile.yml", "templates/python/Taskfile.yml"); err != nil {
		fmt.Println(err)
	}

	return nil
}

func GenCpp() error {
	if err := GenFile("./src/main.cpp", "templates/cpp/main.cpp"); err != nil {
		fmt.Println(err)
	}

	if err := GenFile("./CMakeLists.txt", "templates/cpp/CMakeLists.txt"); err != nil {
		fmt.Println(err)
	}

	if err := GenCppTask(); err != nil {
		fmt.Println(err)
	}

	return nil
}

func GenCppZig() error {
	if err := GenFile("./src/main.cpp", "templates/cppzig/main.cpp"); err != nil {
		fmt.Println(err)
	}

	if err := GenFile("./build.zig", "templates/cppzig/build.zig"); err != nil {
		fmt.Println(err)
	}

	if err := GenFile("./Taskfile.yml", "templates/cppzig/Taskfile.yml"); err != nil {
		fmt.Println(err)
	}

	return nil
}

func GenCMake() error {
	if err := GenFile("./CMakeLists.txt", "templates/cmake/CMakeLists.txt"); err != nil {
		fmt.Println(err)
	}

	return nil
}

func GenCppTask() error {
	if err := GenFile("./Taskfile.yml", "templates/cpp/Taskfile.yml"); err != nil {
		fmt.Println(err)
	}

	return nil
}
