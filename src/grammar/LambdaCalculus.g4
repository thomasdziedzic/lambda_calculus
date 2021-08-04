grammar LambdaCalculus;

term
    : variable=VARIABLE #variable
    | 'λ' variables+=VARIABLE '.' abs_body=term #abstraction
    | left=term right=term #application
    | 'let' variable=VARIABLE '=' assignment=term 'in' body=term #let
    | '(' inner=term ')' #parens
    ;

VARIABLE : [a-zA-Z0-9_]+;

WS : [ \r\n]+ -> skip;
