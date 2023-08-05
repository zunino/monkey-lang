package lexer

import (
	"zunino.br/monkey-lang/token"
)

func New(input string) *Lexer {
	lexer := &Lexer{input: input}
	lexer.readChar()
	return lexer
}

type Lexer struct {
	input        string
	position     int
	readPosition int
	ch           byte
}

func newToken(tokType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokType, Text: string(ch)}
}

func (l *Lexer) NextToken() token.Token {
	var tok token.Token

	l.skipWhitespace()

	switch l.ch {
	case '=':
        if l.peekChar() == '=' {
            l.readChar()
            tok = token.Token{Type: token.EQUALS, Text: "=="}
        } else {
            tok = newToken(token.ASSIGN, l.ch)
        }
	case ',':
		tok = newToken(token.COMMA, l.ch)
	case ';':
		tok = newToken(token.SEMICOLON, l.ch)
	case '(':
		tok = newToken(token.LPAREN, l.ch)
	case ')':
		tok = newToken(token.RPAREN, l.ch)
	case '{':
		tok = newToken(token.LBRACE, l.ch)
	case '}':
		tok = newToken(token.RBRACE, l.ch)
	case '+':
		tok = newToken(token.PLUS, l.ch)
    case '-':
        tok = newToken(token.MINUS, l.ch)
    case '/':
        tok = newToken(token.SLASH, l.ch)
    case '*':
        tok = newToken(token.ASTERISK, l.ch)
    case '<':
        tok = newToken(token.LT, l.ch)
    case '>':
        tok = newToken(token.GT, l.ch)
    case '!':
        if l.peekChar() == '=' {
            l.readChar()
            tok = token.Token{Type: token.DIFFERENT, Text: "!="}
        } else {
            tok = newToken(token.BANG, l.ch)
        }
	case 0:
		tok.Text = ""
		tok.Type = token.EOF
	default:
		if isLetter(l.ch) {
			tok.Text = l.readIdentifier()
			tok.Type = token.LookupIdent(tok.Text)
			return tok
		} else if isDigit(l.ch) {
			tok.Text = l.readInt()
			tok.Type = token.INT
			return tok
		}
		tok = newToken(token.ILLEGAL, l.ch)
	}

	l.readChar()
	return tok
}

func (l *Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		l.ch = 0
	} else {
		l.ch = l.input[l.readPosition]
	}
	l.position = l.readPosition
	l.readPosition++
}

func (l *Lexer) readIdentifier() string {
	position := l.position
	for isLetter(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) readInt() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		l.readChar()
	}
}

func (l *Lexer) peekChar() byte {
    if l.readPosition <  len(l.input) {
        return l.input[l.readPosition]
    }
    return 0
}

func isLetter(ch byte) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

func isDigit(ch byte) bool {
	return ch >= '0' && ch <= '9'
}
