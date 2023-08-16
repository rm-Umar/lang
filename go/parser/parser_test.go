package parser

import (
	"lang/ast"
	"lang/lexer"
	"testing"
)

func TestMaanloStatement(t *testing.T) {
	input := `
	maanlo x = 5;
	maanlo y = 10;
	maanlo abc = 15;
	`

	l := lexer.New(input)
	p := New(l)

	program := p.ParseProgram()
	if program == nil {
		t.Fatalf("Parsed Program returned nill")
	}
	if len(program.Statements) != 3 {
		t.Fatalf("program doest not contain 3 statements. got=%d", len(program.Statements))
	}

	tests := []struct {
		expextedIdentifier string
	}{
		{"x"},
		{"y"},
		{"abc"},
	}

	for i, tt := range tests {
		stmt := program.Statements[i]
		if !testMaanloStatement(t, stmt, tt.expextedIdentifier) {
			return
		}
	}
}

func testMaanloStatement(t *testing.T, s ast.Statement, name string) bool {
	if s.TokenLiteral() != "maanlo" {
		t.Errorf("token literal is not maanlo. got=%q", s.TokenLiteral())
		return false
	}

	maanlosmt, ok := s.(*ast.MaanloStatement)
	if !ok {
		t.Errorf("s not *ast.MaanloStatement. got=%q", s)
		return false
	}

	if maanlosmt.Name.Value != name {
		t.Errorf("Name.Value not %s. got=%s", name, maanlosmt.Name.Value)
		return false
	}

	return true
}
