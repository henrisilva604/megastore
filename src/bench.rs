use crate::catalogo::Catalogo;
use crate::grafo::Grafo;
use crate::produto::Produto;
use crate::recomendador::recomendar_para_cliente;
use std::hint::black_box;
use std::time::Instant;

/// Gerador pseudoaleatório simples (evita depender de crates externas).
struct Lcg(u64);

impl Lcg {
    fn proximo(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

pub fn gerar_dados(
    n_produtos: u32,
    n_clientes: u32,
    compras_por_cliente: u32,
) -> (Catalogo, Grafo) {
    let mut rng = Lcg(42);

    let mut catalogo = Catalogo::new();
    for id in 1..=n_produtos {
        catalogo
            .cadastrar(Produto::new(id, &format!("Produto {}", id), "Geral", 10.0))
            .unwrap();
    }

    let mut grafo = Grafo::new();
    for cliente in 1..=n_clientes {
        for _ in 0..compras_por_cliente {
            let produto = (rng.proximo() % n_produtos as u64) as u32 + 1;
            grafo.registrar_compra(cliente, produto, 1.0);
        }
    }
    (catalogo, grafo)
}

pub fn executar() {
    println!(
        "{:>10} {:>10} {:>14} {:>22}",
        "produtos", "clientes", "construção", "recomendação (média)"
    );

    for &n in &[1_000u32, 10_000, 100_000] {
        let inicio = Instant::now();
        let (_catalogo, grafo) = gerar_dados(n, n, 10);
        let construcao = format!("{:?}", inicio.elapsed());

        let amostra = 100u32;
        let inicio = Instant::now();
        for cliente in 1..=amostra {
            black_box(recomendar_para_cliente(&grafo, cliente, 5));
        }
        let media = format!("{:?}", inicio.elapsed() / amostra);

        println!("{:>10} {:>10} {:>14} {:>22}", n, n, construcao, media);
    }
}