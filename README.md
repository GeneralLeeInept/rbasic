# RBASIC
A BASIC interpreter, built in Rust.

## Grammar
```
statement   ->  expr

expr        ->  add_expr
add_expr    ->  add_expr add_op mul_expr
            |   mul_expr
mul_expr    ->  mul_expr mul_op pow_expr    
            |   pow_expr
pow_expr    ->  pow_expr ^ signed_term
            |   signed_term
signed_term ->  sign term
            |   term
term        ->  integer
            |   ( expr )

add_op      ->  + | -
mul_op      ->  * | /
sign        ->  + | -

integer     ->  [0-9]+      
```

