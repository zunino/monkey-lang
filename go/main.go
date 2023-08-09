package main

import (
	"fmt"
	"os"
	"os/user"

	"zunino.br/monkey-lang/repl"
)

func main() {
    user, err := user.Current()
    if err != nil {
        panic(err)
    }
    fmt.Printf("Hi, %s, and welcome to Monkey!\n", user.Username)
    repl.Start(os.Stdin, os.Stdout)
}
