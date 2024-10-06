package main

import (
	"flag"
	"fmt"
	"os"

	"tgen/gen"
)

func main() {
	option := flag.String("option", "",
		`an option to generate, 
examples:
	tgen python
	tgen pytask
	tgen cpp
	tgen cpptask
	tgen go
`)

	flag.Parse()

	// Check if the option is provided via the flag
	if *option == "" {
		if len(os.Args) < 2 {
			fmt.Println("Error: options not provided")
			flag.Usage()
		} else {
			*option = os.Args[1] // Get the first argument after the program name
		}
	}

	switch *option {
	case "python", "py":
		gen.GenPython()
	case "pytask":
		gen.GenPythonTask()
	case "cpp":
		gen.GenCpp()
	case "cpptask":
		gen.GenCppTask()
	default:
		fmt.Println("name not known!")
		flag.Usage()
	}
}
