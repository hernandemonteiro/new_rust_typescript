# New Rust TypeScript

Este projeto é uma implementação de uma linguagem de programação interpretada inspirada no TypeScript, utilizando Rust como linguagem base. O objetivo é criar uma linguagem simples, com suporte a variáveis, operações aritméticas, funções básicas e impressão de valores.

## Funcionalidades

- **Atribuição de variáveis**: Suporte para declarar e atribuir valores a variáveis.
- **Operações aritméticas**: Suporte para soma, subtração, multiplicação e divisão.
- **Funções básicas**: Implementação de uma função `print` para exibir valores no console.
- **Interpretação de código**: O código é analisado, transformado em uma AST (Abstract Syntax Tree) e interpretado em tempo de execução.

## Estrutura do Projeto

- `src/ast.rs`: Define a estrutura da AST (Abstract Syntax Tree) usada para representar o código.
- `src/lexer.rs`: Implementa o analisador léxico (tokenizador) para dividir o código em tokens.
- `src/parser.rs`: Implementa o analisador sintático para transformar os tokens em uma AST.
- `src/interpreter.rs`: Implementa o interpretador que executa a AST.
- `src/main.rs`: Ponto de entrada do programa, responsável por integrar todas as partes.

## Exemplo de Código

```typescript
let x = 10 + 2;
print(x);
