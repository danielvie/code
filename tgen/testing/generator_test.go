// generator_test.go
package generator

import (
	"os"
	"path/filepath"
	"testing"
	"tgen/gen"
)

// TestGenFile tests the GenFile function
func TestGenFile(t *testing.T) {
	filename := "./test_output/test_file.txt"
	templatePath := "templates/python/main.py" // Adjust this based on your templates

	// Cleanup after test
	defer os.RemoveAll(filepath.Dir(filename))

	err := gen.GenFile(filename, templatePath)
	if err != nil {
		t.Fatalf("Failed to generate file: %v", err)
	}

	if _, err := os.Stat(filename); os.IsNotExist(err) {
		t.Errorf("Expected file %s to exist, but it does not", filename)
	}
}

// TestGenPython tests the GenPython function
func TestGenPython(t *testing.T) {
	err := gen.GenPython()
	if err != nil {
		t.Fatalf("Failed to generate Python files: %v", err)
	}

	expectedFiles := []string{
		"./main.py",
		"./Taskfile.yml",
	}

	// Cleanup after test
	defer os.RemoveAll("main.py")
	defer os.RemoveAll("Taskfile.yml")

	for _, file := range expectedFiles {
		if _, err := os.Stat(file); os.IsNotExist(err) {
			t.Errorf("Expected file %s to exist, but it does not", file)
		}
	}
}

// TestGenCpp tests the GenCpp function
func TestGenCpp(t *testing.T) {
	err := gen.GenCpp()
	if err != nil {
		t.Fatalf("Failed to generate C++ files: %v", err)
	}

	expectedFiles := []string{
		"./src/main.cpp",
		"./CMakeLists.txt",
		"./Taskfile.yml",
	}

	// Cleanup after test
	defer os.RemoveAll("src")
	defer os.RemoveAll("CMakeLists.txt")
	defer os.RemoveAll("Taskfile.yml")

	for _, file := range expectedFiles {
		if _, err := os.Stat(file); os.IsNotExist(err) {
			t.Errorf("Expected file %s to exist, but it does not", file)
		}
	}
}
