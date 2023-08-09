package ast

import "zunino.br/monkey-lang/token"

type Node interface {
    TokenText() string
}

type Statement interface {
    Node
    statementNode()
}

type Expression interface {
    Node
    expressionNode()
}

type Program struct {
    Statements []Statement
}

func (p *Program) TokenText() string {
    if len(p.Statements) > 0 {
        return p.Statements[0].TokenText()
    }
    return ""
}

type LetStatement struct {
    Token token.Token
    Name *Identifier
    Value Expression
}

func (l *LetStatement) TokenText() string {
    return l.Token.Text
}

func (l *LetStatement) statementNode() {}

type Identifier struct {
    Token token.Token
    Value string
}

func (i *Identifier) TokenText() string {
    return i.Token.Text
}

func (i *Identifier) expressionNode() {}

type ReturnStatement struct {
    Token token.Token
    ReturnValue Expression
}

func (r *ReturnStatement) TokenText() string {
    return r.Token.Text
}

func (r* ReturnStatement) statementNode() {}
