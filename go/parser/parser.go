package parser

import (
	"lang/ast"
	"lang/lexer"
	"lang/token"
)

type Parser struct {
	l         *lexer.Lexer
	CurToken  token.Token
	PeekToken token.Token
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l}

	p.NextToken()
	p.NextToken()
	return p
}

func (p *Parser) NextToken() {
	p.CurToken = p.PeekToken
	p.PeekToken = p.l.NextToken()
}

func (p *Parser) ParseProgram() *ast.Program {
	return nil
}
