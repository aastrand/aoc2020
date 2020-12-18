import ply.lex as lex
import ply.yacc as yacc

import sys

tokens = (
    'NUMBER',
    'PLUS', 'TIMES',
    'LPAREN', 'RPAREN',
)

t_PLUS = r'\+'
t_TIMES = r'\*'
t_LPAREN = r'\('
t_RPAREN = r'\)'


def t_NUMBER(t):
    r'\d+'
    t.value = int(t.value)
    return t


t_ignore = " \t"


def t_newline(t):
    r'\n+'
    t.lexer.lineno += t.value.count("\n")


def t_error(t):
    print(f"Illegal character {t.value[0]!r}")
    t.lexer.skip(1)


lex.lex()

precedence = (
    ('left', 'TIMES', 'PLUS'),
)

SUM = 0


def p_statement_expr(p):
    'statement : expression'
    global SUM
    SUM += p[1]


def p_expression_binop(p):
    '''expression : expression PLUS expression
                  | expression TIMES expression'''
    if p[2] == '+':
        p[0] = p[1] + p[3]
    elif p[2] == '*':
        p[0] = p[1] * p[3]


def p_expression_group(p):
    'expression : LPAREN expression RPAREN'
    p[0] = p[2]


def p_expression_number(p):
    'expression : NUMBER'
    p[0] = p[1]


def p_error(p):
    print(f"Syntax error at {p.value!r}")


yacc.yacc()

for line in sys.stdin:
    yacc.parse(line)
print(SUM)
