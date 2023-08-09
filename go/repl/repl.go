package repl

import (
	"bufio"
	"fmt"
	"io"

	"zunino.br/monkey-lang/lexer"
	"zunino.br/monkey-lang/token"
)

const prompt = "monkey:"

func Start(in io.Reader, out io.Writer) {
    scanner := bufio.NewScanner(in)
    for {
        fmt.Printf("%s ", prompt)

        ok := scanner.Scan()
        if !ok {
            return
        }

        line := scanner.Text()
        l := lexer.New(line)

        for tok := l.NextToken(); tok.Type != token.EOF; tok = l.NextToken() {
            fmt.Printf("%+v\n", tok)
        }
    }
}
