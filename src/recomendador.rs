use crate::grafo::{ClienteId, Grafo, ProdutoId};
use std::collections::{HashMap, HashSet};

/// Percorre o grafo em 3 níveis:
/// cliente -> seus produtos -> outros compradores -> produtos deles.
pub fn recomendar_para_cliente(
    grafo: &Grafo,
    cliente: ClienteId,
    k: usize,
) -> Vec<(ProdutoId, f32)> {
    // Conjunto para nunca recomendar o que o cliente já comprou
    let ja_comprados: HashSet<ProdutoId> = grafo
        .produtos_do_cliente(cliente)
        .iter()
        .map(|(p, _)| *p)
        .collect();

    // Nível 2: similaridade com outros clientes (soma dos pesos dos produtos em comum)
    let mut similares: HashMap<ClienteId, f32> = HashMap::new();
    for &(produto, peso_a) in grafo.produtos_do_cliente(cliente) {
        for &(outro, peso_b) in grafo.compradores_do_produto(produto) {
            if outro != cliente {
                *similares.entry(outro).or_insert(0.0) += peso_a * peso_b;
            }
        }
    }

    // Nível 3: pontuação dos produtos candidatos (HashMap evita duplicados)
    let mut pontuacao: HashMap<ProdutoId, f32> = HashMap::new();
    for (&outro, &sim) in &similares {
        for &(produto, peso) in grafo.produtos_do_cliente(outro) {
            if !ja_comprados.contains(&produto) {
                *pontuacao.entry(produto).or_insert(0.0) += sim * peso;
            }
        }
    }

    // Ordena por pontuação (desempate por id) e devolve os k melhores
    let mut resultado: Vec<(ProdutoId, f32)> = pontuacao.into_iter().collect();
    resultado.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    resultado.truncate(k);
    resultado
}