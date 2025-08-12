# Interpretador Lox em Rust

Este projeto é uma implementação de um interpretador para a linguagem de programação Lox, escrita em Rust. O objetivo é fornecer uma base para estudos de construção de interpretadores, seguindo o livro [Crafting Interpreters](https://craftinginterpreters.com/).

## Estrutura do Projeto

- **src/**: Contém o código-fonte principal do interpretador.

  - [`main.rs`](src/main.rs): Ponto de entrada da aplicação.
  - [`scanner.rs`](src/scanner.rs): Responsável pela análise léxica (tokenização) do código-fonte.
  - [`parser.rs`](src/parser.rs): Realiza a análise sintática, transformando tokens em uma árvore de sintaxe abstrata (AST).
  - [`expr.rs`](src/expr.rs) e [`stmt.rs`](src/stmt.rs): Definem as estruturas da AST para expressões e declarações.
  - [`interpreter.rs`](src/interpreter.rs): Executa a AST, interpretando o código Lox.
  - [`environment.rs`](src/environment.rs): Gerencia os ambientes de variáveis (escopos).
  - [`token.rs`](src/token.rs) e [`token_type.rs`](src/token_type.rs): Definem os tipos de tokens e suas representações.
  - [`literal_object.rs`](src/literal_object.rs): Representa valores literais da linguagem.
  - [`ast_printer.rs`](src/ast_printer.rs): Utilitário para imprimir a AST (útil para depuração).

- **test**: Exemplo de script Lox para testar o interpretador.

## Como Executar

1. **Compilar o projeto:**

   ```sh
   cargo build
   ```

2. **Executar um arquivo Lox:**

   ```sh
   cargo run -- test
   ```

3. **Modo interativo (REPL):**

   ```sh
   cargo run
   ```

## Testando

O arquivo [`test`](test) contém um exemplo de código Lox para testar variáveis e escopos:

```lox
var a = 1;
var b = 2;
print(a + b);
{
  var a = 2;
  var b = 2;
  print(a + b);
}
```
