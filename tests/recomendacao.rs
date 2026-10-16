use megastore::grafo::Grafo;
use megastore::recomendador::recomendar_para_cliente;
use std::collections::HashSet;

fn grafo_exemplo() -> Grafo {
    let mut g = Grafo::new();
    g.registrar_compra(1, 1, 1.0);
    g.registrar_compra(1, 2, 1.0);
    g.registrar_compra(2, 1, 1.0);
    g.registrar_compra(2, 2, 1.0);
    g.registrar_compra(2, 3, 1.0);
    g.registrar_compra(3, 2, 1.0);
    g.registrar_compra(3, 4, 1.0);
    g
}

fn ids(recs: &[(u32, f32)]) -> Vec<u32> {
    recs.iter().map(|(id, _)| *id).collect()
}

#[test]
fn recomenda_produtos_de_clientes_similares_em_ordem() {
    let recs = recomendar_para_cliente(&grafo_exemplo(), 1, 3);
    assert_eq!(ids(&recs), vec![3, 4]);
}

#[test]
fn nunca_recomenda_produto_ja_comprado() {
    let recs = recomendar_para_cliente(&grafo_exemplo(), 1, 10);
    let ids = ids(&recs);
    assert!(!ids.contains(&1) && !ids.contains(&2));
}

#[test]
fn nao_gera_recomendacoes_duplicadas() {
    let recs = recomendar_para_cliente(&grafo_exemplo(), 1, 10);
    let ids = ids(&recs);
    let unicos: HashSet<u32> = ids.iter().copied().collect();
    assert_eq!(unicos.len(), ids.len());
}

#[test]
fn cliente_sem_compras_nao_recebe_recomendacao() {
    let recs = recomendar_para_cliente(&grafo_exemplo(), 99, 5);
    assert!(recs.is_empty());
}

#[test]
fn respeita_o_limite_k() {
    let recs = recomendar_para_cliente(&grafo_exemplo(), 1, 1);
    assert_eq!(recs.len(), 1);
}