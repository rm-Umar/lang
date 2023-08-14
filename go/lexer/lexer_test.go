package lexer

import (
	"lang/token"
	"testing"
)

func TestNextToken(t *testing.T) {
	input := `
	maanlo jammat = 8;
	kaam kya_jammat() {
	    agar jammat == 8 {
	        wapis sahih;
	    } warna {
	        wapis ghalat;
	    }
	}
	> <  <= >= == != + - / * !
`

	tests := []struct {
		expectedType    token.TokenType
		expextedLiteral string
	}{
		{token.MAANLO, "maanlo"},
		{token.IDENT, "jammat"},
		{token.ASSIGN, "="},
		{token.INT, "8"},
		{token.SEMICOLON, ";"},
		{token.KAAM, "kaam"},
		{token.IDENT, "kya_jammat"},
		{token.LPAREN, "("},
		{token.RPAREN, ")"},
		{token.LBRACE, "{"},
		{token.AGAR, "agar"},
		{token.IDENT, "jammat"},
		{token.EQ, "=="},
		{token.INT, "8"},
		{token.LBRACE, "{"},
		{token.WAPIS, "wapis"},
		{token.SAHIH, "sahih"},
		{token.SEMICOLON, ";"},
		{token.RBRACE, "}"},
		{token.WARNA, "warna"},
		{token.LBRACE, "{"},
		{token.WAPIS, "wapis"},
		{token.GHALAT, "ghalat"},
		{token.SEMICOLON, ";"},
		{token.RBRACE, "}"},
		{token.RBRACE, "}"},
		{token.GT, ">"},
		{token.LT, "<"},
		{token.LT_EQ, "<="},
		{token.GT_EQ, ">="},
		{token.EQ, "=="},
		{token.NOT_EQ, "!="},
		{token.PLUS, "+"},
		{token.MINUS, "-"},
		{token.SLASH, "/"},
		{token.ASTERISK, "*"},
		{token.BANG, "!"},
	}

	l := New(input)

	for i, tt := range tests {
		tok := l.NextToken()
		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - wrong token type, expected=%q, got=%q", i, tt.expectedType, tok.Type)
		}

		if tok.Literal != tt.expextedLiteral {
			t.Fatalf("tests[%d] - wrong linteral, expected=%q, got=%q", i, tt.expextedLiteral, tok.Literal)
		}
	}
}
