# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

Projeto da disciplina **Data Structure Strategy and Implementation**, desenvolvido em **Rust**.

🎥 **Vídeo pitch:** [COLE AQUI O LINK DO VÍDEO]

## Objetivo

A MegaStore (empresa fictícia de e-commerce) possui um catálogo enorme, mas recomenda apenas itens mais vendidos ou da mesma categoria. O ConectaStore modela as relações entre **clientes** e **produtos** como um grafo e percorre essas conexões para recomendar produtos relevantes ao comportamento de compra de cada cliente.

## Como funciona

A recomendação é feita por **filtragem colaborativa sobre o grafo**: "clientes parecidos com você também compraram...".

Para recomendar produtos a um cliente, o sistema percorre o grafo em três níveis (BFS limitado por profundidade):

1. **Nível 1:** produtos que o cliente já comprou.
2. **Nível 2:** outros clientes que compraram esses mesmos produtos. Cada um recebe uma pontuação de similaridade (soma de `peso × peso` dos produtos em comum).
3. **Nível 3:** produtos comprados por esses clientes similares. Cada candidato acumula `similaridade × peso`.

Em seguida, os produtos já comprados pelo cliente são descartados, os candidatos são ordenados por pontuação (desempate pelo id, para resultado determinístico) e os `k` melhores são devolvidos.

### Modelagem do grafo

| Elemento | Representação |
|---|---|
| Vértices | Clientes e produtos (grafo **bipartido**) |
| Arestas | Compras: ligam um cliente a um produto |
| Pesos | Intensidade da relação (ex.: nota ou quantidade); `f32` |
| Tipo | Não direcionado e ponderado |
| Representação | **Lista de adjacência**, guardada nos dois sentidos |

A lista de adjacência foi escolhida em vez da matriz porque o grafo de um e-commerce é **esparso**: um cliente compra poucos produtos entre milhões. A matriz ocuparia O(V²) de memória, enquanto a lista ocupa O(V + E).

## Tecnologias e estruturas de dados

- **Rust** (edição 2024, apenas biblioteca padrão; nenhuma dependência externa).
- `HashMap<u32, Produto>`: acesso a produtos por identificador em O(1) médio.
- `HashMap<ClienteId, Vec<(ProdutoId, f32)>>` e `HashMap<ProdutoId, Vec<(ClienteId, f32)>>`: listas de adjacência nos dois sentidos, para percorrer cliente → produto e produto → cliente.
- `HashSet<ProdutoId>`: exclui do resultado o que o cliente já comprou.
- `HashMap<ProdutoId, f32>`: acumula a pontuação dos candidatos, garantindo que cada produto apareça uma única vez (**prevenção de recomendações duplicadas**).
- `Vec`: armazena o ranking final, ordenado e truncado em `k`.

## Arquitetura

```
megastore/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs          # demonstração e modo benchmark (CLI)
│   ├── lib.rs           # expõe os módulos da biblioteca
│   ├── produto.rs       # struct Produto
│   ├── catalogo.rs      # cadastro e consulta de produtos (HashMap)
│   ├── grafo.rs         # grafo bipartido cliente-produto
│   ├── recomendador.rs  # algoritmo de recomendação
│   └── bench.rs         # geração de dados sintéticos e medição de tempo
└── tests/
    └── recomendacao.rs  # testes de integração
```

A lógica fica na biblioteca (`lib.rs`) e o `main.rs` apenas a utiliza, o que permite testá-la isoladamente nos testes de integração.

## Complexidade

Considere *d* = média de produtos por cliente, *g* = média de compradores por produto e *m* = número de produtos candidatos.

| Operação | Tempo | Espaço |
|---|---|---|
| Cadastrar / buscar produto | O(1) médio | O(P) |
| Registrar compra (aresta) | O(d) (verifica duplicata) | O(1) por aresta |
| Recomendação | O(d·g + d·g·d + m log m) | O(d·g + m) |
| Grafo completo | — | O(V + E) |

O custo da recomendação depende da **vizinhança** do cliente e não do tamanho total do grafo, o que explica a escalabilidade observada nos testes de desempenho.

## Como compilar e executar

### Pré-requisitos

- [Rust](https://rustup.rs) (inclui `cargo`).
- No Windows, é necessário o **Build Tools for Visual Studio** com a carga de trabalho "Desenvolvimento para desktop com C++" (fornece o linker `link.exe`).

### Executar a demonstração

```bash
git clone https://github.com/henrisilva604/megastore.git
cd megastore
cargo run
```

### Executar os testes de desempenho

```bash
cargo run --release -- bench
```

> Use sempre `--release` para medir desempenho: sem otimizações, os tempos não são representativos.

## Como executar os testes

```bash
cargo test
```

São 9 testes: 4 unitários (`catalogo` e `grafo`) e 5 de integração (`tests/recomendacao.rs`), que cobrem:

- cadastro, busca e rejeição de id duplicado;
- inexistência de arestas duplicadas;
- recomendação por clientes similares, na ordem correta;
- nunca recomendar produto já comprado;
- ausência de recomendações duplicadas;
- cliente sem compras e limite `k`.

## Exemplo de uso

Dados da demonstração: o cliente 1 comprou Notebook e Mouse; o cliente 2 comprou Notebook, Mouse e Teclado; o cliente 3 comprou Mouse e Livro Rust.

```
$ cargo run
Recomendações para o cliente 1:
  Teclado [Eletrônicos] R$ 150.00 (pontuação 2.0)
  Livro Rust [Livros] R$ 90.00 (pontuação 1.0)
```

O Teclado vem primeiro porque o cliente 2 é muito similar ao cliente 1 (dois produtos em comum, similaridade 2), enquanto o cliente 3 compartilha apenas o Mouse (similaridade 1).

Uso como biblioteca:

```rust
use megastore::grafo::Grafo;
use megastore::recomendador::recomendar_para_cliente;

let mut grafo = Grafo::new();
grafo.registrar_compra(1, 10, 1.0); // cliente 1 comprou o produto 10
grafo.registrar_compra(2, 10, 1.0);
grafo.registrar_compra(2, 20, 1.0);

let recomendacoes = recomendar_para_cliente(&grafo, 1, 5); // top 5
```

## Resultados dos testes de desempenho

Dados sintéticos: *N* produtos, *N* clientes, 10 compras por cliente (gerador pseudoaleatório com semente fixa). A recomendação é a média de 100 clientes (top 5).

| Produtos | Clientes | Construção do grafo | Recomendação (média) |
|---|---|---|---|
| 1.000 | 1.000 | 1,12 ms | 57 µs |
| 10.000 | 10.000 | 14,3 ms | 75 µs |
| 100.000 | 100.000 | 278,8 ms | 86 µs |

Ambiente de teste: AMD Ryzen 5 4500 (6 núcleos, 3,6 GHz), 16 GB de RAM, Windows 11 (64 bits), compilado com `--release`.

**Análise:**

- A **construção** cresce de forma aproximadamente linear com o volume de dados (O(V + E)); o pequeno excesso vem do redimensionamento dos `HashMap` e da menor localidade de memória.
- A **recomendação** praticamente não muda quando os dados crescem 100 vezes, porque o algoritmo visita apenas a vizinhança do cliente e não o grafo inteiro.

### Limitações e evoluções

- Os dados sintéticos são uniformes; em um catálogo real há produtos muito populares (muitos compradores), o que aumenta o custo. Uma mitigação é limitar quantos compradores são considerados por produto.
- O ranking final usa ordenação completa; um `BinaryHeap` de tamanho *k* reduziria o custo de O(m log m) para O(m log k).
- Recomendação a partir de um produto ("quem comprou este também comprou") reutilizando o mesmo grafo.
