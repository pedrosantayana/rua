pub const SOURCE: &str = r#"
chunk ::= block

block ::= {stat} [retstat]

stat ::=  ';' | 
     varlist '=' explist | 
     functioncall | 
     label | 
     'break' | 
     'goto' Name | 
     'do' block 'end' | 
     'while' exp 'do' block 'end' | 
     'repeat' block 'until' exp | 
     'if' exp 'then' block {'elseif' exp 'then' block} ['else' block] 'end' | 
     'for' Name '=' exp ',' exp [',' exp] 'do' block 'end' | 
     'for' namelist 'in' explist 'do' block 'end' | 
     'function' funcname funcbody | 
     'local' 'function' Name funcbody | 
     'local' attnamelist ['=' explist] 

attnamelist ::=  Name attrib {',' Name attrib}

attrib ::= ['<' Name '>']

retstat ::= return [explist] [';']

label ::= '::' Name '::'

funcname ::= Name {'.' Name} [':' Name]

varlist ::= var {',' var}

var ::=  Name | prefixexp '[' exp ']' | prefixexp '.' Name 

namelist ::= Name {',' Name}

explist ::= exp {',' exp}

exp ::=  'nil' | 'false' | 'true' | Numeral | LiteralString | '...' | functiondef | 
     prefixexp | tableconstructor | exp binop exp | unop exp 

prefixexp ::= var | functioncall | '(' exp ')'

functioncall ::=  prefixexp args | prefixexp ':' Name args 

args ::=  '(' [explist] ')' | tableconstructor | LiteralString 

functiondef ::= 'function' funcbody

funcbody ::= '(' [parlist] ')' block 'end'

parlist ::= namelist [',' '...'] | '...'

tableconstructor ::= '{' [fieldlist] '}'

fieldlist '[' exp ']' '=' exp | Name '=' exp | exp

fieldsep ::= ',' | ';'

binop ::=  '+' | '-' | '*' | '/' | '//' | '^' | '%' | 
     '&' | '~' | '|' | '>>' | '<<' | '..' | 
     '<' | '<=' | '>' | '>=' | '==' | '~=' | 
     'and' | 'or'

unop ::= '-' | not | '#' | '~'
field ::= '[' exp ']' '=' exp | Name '=' exp | exp

fieldsep ::= ',' | ';'

binop ::=  '+' | '-' | '*' | '/' | '//' | '^' | '%' | 
     '&' | '~' | '|' | '>>' | '<<' | '..' | 
     '<' | '<=' | '>' | '>=' | '==' | '~=' | 
     'and' | 'or'

unop ::= '-' | 'not' | '#' | '~'

LiteralString := '"' ( EscapeSequence | ~('\\'|'"') )* '"' | "'" ( EscapeSequence | ~("\\"|"'") )* "'"

Name := #'[a-zA-Z_][a-zA-Z_0-9]*'

EscapeSequence := 
                '\\' '\r'? '\n'
               | DecimalEscape
               | '\\' 'x' HexDigit HexDigit
               | UtfEscape

DecimalEscape :=
               '\\' Digit
               | '\\' Digit Digit
               | '\\' #'[0-2]' Digit Digit

Digit := #'[0-9]'

HexDigit := #'[0-9a-fA-F]'

UtfEscape := '\\' 'u{' HexDigit+ '}'

Numeral := #'[0-9]+'
"#;