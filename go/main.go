package main

import (
	"fmt"
	"lang/repl"
	"os"
)

func main() {
	args := os.Args

	if len(args) > 2 {
		fmt.Printf("Usage: lang [file]")
		os.Exit(420)
	} else if len(args) == 2 {
		file, err := os.Open(args[1])
		if err != nil {
			panic("File error")
		}
		repl.Start(file, os.Stdout)
	} else {
		repl.Start(os.Stdin, os.Stdout)
	}
}
