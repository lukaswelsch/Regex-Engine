// Regex.g4

grammar Regex;

// Parser rules
expr        : term ( '|' term )* #union;        // Union
term        : factor+            #concat ;      // Concatenation
factor      : primary kleene?        ;      // Kleene star
kleene      : '*';
primary     : '(' expr ')'      #parenthesis
            | CHAR              #char
            ;

// Lexer rules
CHAR        : [a-zA-Z0-9] ;          // Single character
WS          : [ \t\r\n]+ -> skip ;  // Whitespace
