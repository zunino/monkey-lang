package parser

import (
	"fmt"

	"zunino.br/monkey-lang/ast"
	"zunino.br/monkey-lang/lexer"
	"zunino.br/monkey-lang/token"
)

type Parser struct {
    l *lexer.Lexer
    curToken token.Token
    peekToken token.Token
    errors []string
}

func New(l* lexer.Lexer) *Parser {
    p := &Parser{l: l, errors: []string{}}
    p.nextToken()
    p.nextToken()
    return p
}

func (p *Parser) nextToken() {
    p.curToken = p.peekToken
    p.peekToken = p.l.NextToken()
}

func (p *Parser) ParseProgram() *ast.Program {
    program := &ast.Program{Statements: []ast.Statement{}}
    for p.curToken.Type != token.EOF {
        stmt := p.parseStatement()
        if stmt != nil {
            program.Statements = append(program.Statements, stmt)
        }
        p.nextToken()
    }
    return program
}

func (p *Parser) parseStatement() ast.Statement {
    switch p.curToken.Type {
    case token.LET:
        return p.parseLetStatement()
    case token.RETURN:
        return p.parseReturnStatement()
    default:
        return nil
    }
}

func (p *Parser) parseLetStatement() *ast.LetStatement {
    stmt := &ast.LetStatement{Token: p.curToken}

    if !p.expectPeek(token.IDENT) {
        return nil
    }

    stmt.Name = &ast.Identifier{Token: p.curToken, Value: p.curToken.Text} 

    if !p.expectPeek(token.ASSIGN) {
        return nil
    }

    // TODO: for now, skipping rhs expressions until semicolon.
    for p.curToken.Type != token.SEMICOLON {
        p.nextToken()
    }

    return stmt
}

func (p *Parser) parseReturnStatement() *ast.ReturnStatement {
    stmt := &ast.ReturnStatement{Token: p.curToken}
    p.nextToken()

    // TODO: for now, skipping rhs expressions until semicolon.
    for p.curToken.Type != token.SEMICOLON {
        p.nextToken()
    }

    return stmt
}

func (p *Parser) expectPeek(tokType token.TokenType) bool {
    if p.peekToken.Type != tokType {
        p.addPeekError(tokType)
        return false
    }
    p.nextToken()
    return true
}

func (p *Parser) addPeekError(tokType token.TokenType) {
    msg := fmt.Sprintf("expected a token %s; got %s", tokType, p.peekToken.Type)
    p.errors = append(p.errors, msg)
}

func (p *Parser) Errors() []string {
    return p.errors
}
