// generator.go
package main

import (
	"embed"
	"fmt"
	"os"
	"path/filepath"
	"text/template"
)

//go:embed templates
var templates embed.FS

func genFile(filename, templatePath string) error {
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

func genPython() {
	if err := genFile("./main.py", "templates/python/main.py"); err != nil {
		fmt.Println(err)
	}

	if err := genPythonTask(); err != nil {
		fmt.Println(err)
	}
}

func genPythonTask() error {
	if err := genFile("./Taskfile.yml", "templates/python/Taskfile.yml"); err != nil {
		fmt.Println(err)
	}

	return nil
}

func genCpp() error {
	if err := genFile("./src/main.cpp", "templates/cpp/main.cpp"); err != nil {
		fmt.Println(err)
	}

	if err := genFile("./CMakeLists.txt", "templates/cpp/CMakeLists.txt"); err != nil {
		fmt.Println(err)
	}

	if err := genCppTask(); err != nil {
		fmt.Println(err)
	}

	return nil
}

func genCppTask() error {
	if err := genFile("./Taskfile.yml", "templates/cpp/Taskfile.yml"); err != nil {
		fmt.Println(err)
	}

	return nil
}
