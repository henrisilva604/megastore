use megastore::catalogo::Catalogo;
use megastore::grafo::Grafo;
use megastore::produto::Produto;
use megastore::recomendador;
fn main() {
        if std::env::args().any(|a| a == "bench") {
        megastore::bench::executar();
        return;
    }
    let mut catalogo = Catalogo::new();
    catalogo.cadastrar(Produto::new(1, "Notebook", "Eletrônicos", 3500.0)).unwrap();
    catalogo.cadastrar(Produto::new(2, "Mouse", "Eletrônicos", 80.0)).unwrap();
    catalogo.cadastrar(Produto::new(3, "Teclado", "Eletrônicos", 150.0)).unwrap();
    catalogo.cadastrar(Produto::new(4, "Livro Rust", "Livros", 90.0)).unwrap();

    let mut grafo = Grafo::new();
    // cliente 1 comprou Notebook e Mouse
    grafo.registrar_compra(1, 1, 1.0);
    grafo.registrar_compra(1, 2, 1.0);
    // cliente 2 comprou Notebook, Mouse e Teclado
    grafo.registrar_compra(2, 1, 1.0);
    grafo.registrar_compra(2, 2, 1.0);
    grafo.registrar_compra(2, 3, 1.0);
    // cliente 3 comprou Mouse e Livro Rust
    grafo.registrar_compra(3, 2, 1.0);
    grafo.registrar_compra(3, 4, 1.0);

    println!("Recomendações para o cliente 1:");
    for (id, pontos) in recomendador::recomendar_para_cliente(&grafo, 1, 3) {
        if let Some(p) = catalogo.buscar(id) {
            println!("  {} [{}] R$ {:.2} (pontuação {:.1})", p.nome, p.categoria, p.preco, pontos);
        }
    }
}