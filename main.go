package main

import (
    "fmt"
    "os"
    "os/user"
    "gonkey/repl"
)

func main() {
    user, err := user.Current()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Hello %s! This is the Gonkey programming language\n", user.Username)
    repl.Start(os.Stdin, os.Stdout)
}
