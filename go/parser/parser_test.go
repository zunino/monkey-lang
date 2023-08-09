package parser

import (
	"testing"

	"zunino.br/monkey-lang/ast"
	"zunino.br/monkey-lang/lexer"
)

func TestLetStatements(t *testing.T) {
	input := `
let x = 5;
let y = 10;
let foobar = 12500;
}
`
	l := lexer.New(input)
	p := New(l)

	program := p.ParseProgram()
    checkParserErrors(t, p)

    statements := program.Statements
	if len(statements) != 3 {
		t.Fatalf("Expected program to contain 3 statements; got %d", len(statements))
	}

	tests := []string{"x", "y", "foobar"}
    for i, expectedIdentifier := range tests {
        stmt := statements[i]
        if !testLetStatement(t, stmt, expectedIdentifier) {
            return
        }
    }
}

func TestReturnStatements(t *testing.T) {
    input := `
return 5;
return 10;
return 12500;
`

    l := lexer.New(input)
    p := New(l)

    program := p.ParseProgram()
    checkParserErrors(t, p)

    statements := program.Statements
    if len(statements) != 3 {
        t.Fatalf("Expected program to contain 3 statements; got %d", len(statements))
    }

    for _, stmt := range statements {
        retStmt, ok := stmt.(*ast.ReturnStatement)
        if !ok {
            t.Errorf("Expected statement to be *ast.ReturnStatement; got %T", stmt)
            continue
        }
        if retStmt.TokenText() != "return" {
            t.Errorf("Return statement's text not 'return'; got %q", retStmt.TokenText())
        }
    }
}

func testLetStatement(t *testing.T, stmt ast.Statement, expectedIdentifier string) bool {
    if stmt.TokenText() != "let" {
        t.Errorf("Expected token text to be 'let'; got %q", stmt.TokenText())
        return false
    }

    letStmt, ok := stmt.(*ast.LetStatement)
    if !ok {
        t.Errorf("Expected statement to be *ast.LetStatement; got %T", stmt)
        return false
    }
    if letStmt.Name.Value != expectedIdentifier {
        t.Errorf("Expected letStmt.Name.Value to be %q; got %q", expectedIdentifier,
            letStmt.Name.Value)
        return false
    }
    if letStmt.Name.TokenText() != expectedIdentifier {
        t.Errorf("Expected letStmt.Name.TokenText() to be %q; got %q",
            expectedIdentifier, letStmt.Name.TokenText())
        return false
    }

    return true
}

func checkParserErrors(t* testing.T, p *Parser) {
    errors := p.Errors()
    if len(errors) == 0 {
        return
    }

    t.Errorf("Parser encountered %d error(s)", len(errors))
    for _, errorMsg := range errors {
        t.Errorf("  - %q", errorMsg)
    }

    t.FailNow()
}

