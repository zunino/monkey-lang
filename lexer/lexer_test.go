package lexer

import (
	"testing"
	"zunino.br/monkey-lang/token"
)

func TestNextToken(t *testing.T) {
	input := `let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
`

	tests := []token.TokenType{
		token.LET,
		token.IDENT,
		token.ASSIGN,
		token.INT,
		token.SEMICOLON,
		token.LET,
		token.IDENT,
		token.ASSIGN,
		token.INT,
		token.SEMICOLON,
		token.LET,
		token.IDENT,
		token.ASSIGN,
		token.FUNCTION,
		token.LPAREN,
		token.IDENT,
		token.COMMA,
		token.IDENT,
		token.RPAREN,
		token.LBRACE,
		token.IDENT,
		token.PLUS,
		token.IDENT,
		token.SEMICOLON,
		token.RBRACE,
		token.SEMICOLON,
		token.LET,
		token.IDENT,
		token.ASSIGN,
		token.IDENT,
		token.LPAREN,
		token.IDENT,
		token.COMMA,
		token.IDENT,
		token.RPAREN,
		token.SEMICOLON,
        token.BANG,
        token.MINUS,
        token.SLASH,
        token.ASTERISK,
        token.INT,
        token.SEMICOLON,
        token.INT,
        token.LT,
        token.INT,
        token.GT,
        token.INT,
        token.SEMICOLON,
        token.IF,
        token.LPAREN,
        token.INT,
        token.LT,
        token.INT,
        token.RPAREN,
        token.LBRACE,
        token.RETURN,
        token.TRUE,
        token.SEMICOLON,
        token.RBRACE,
        token.ELSE,
        token.LBRACE,
        token.RETURN,
        token.FALSE,
        token.SEMICOLON,
        token.RBRACE,
        token.INT,
        token.EQUALS,
        token.INT,
        token.SEMICOLON,
        token.INT,
        token.DIFFERENT,
        token.INT,
        token.SEMICOLON,
		token.EOF,
	}

	lexer := New(input)

	for _, expectedType := range tests {
		tok := lexer.NextToken()

		if tok.Type != expectedType {
			t.Fatalf("wrong token type; expected %s; got %s (%q)", expectedType, tok.Type, tok.Text)
		}
	}
}
