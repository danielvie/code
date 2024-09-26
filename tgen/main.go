package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	option := flag.String("option", "",
		`an option to generate, 
examples:
	tgen python
	tgen cpp
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
		genPython()
	case "cpp":
		genCpp()
	default:
		fmt.Println("name not known!")
		flag.Usage()
	}
}
