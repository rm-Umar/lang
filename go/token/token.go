package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// Identifiers + literals
	IDENT = "IDENT"
	INT   = "INT"

	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"
	SLASH    = "/"

	LT = "<"
	GT = ">"

	EQ     = "=="
	NOT_EQ = "!="
	LT_EQ  = "<="
	GT_EQ  = ">="

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"

	LPAREN = "("
	RPAREN = ")"
	LBRACE = "{"
	RBRACE = "}"

	// Keywords
	KAAM   = "KAAM"
	MAANLO = "MAANLO"
	SAHIH  = "SAHIH"
	GHALAT = "GHALAT"
	AGAR   = "AGAR"
	WARNA  = "WARNA"
	WAPIS  = "WAPIS"
)

var Keywords = map[string]TokenType{
	"kaam":   KAAM,
	"maanlo": MAANLO,
	"sahih":  SAHIH,
	"ghalat": GHALAT,
	"agar":   AGAR,
	"warna":  WARNA,
	"wapis":  WAPIS,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := Keywords[ident]; ok {
		return tok
	}
	return IDENT
}
