use std::collections::HashMap;

pub type ClienteId = u32;
pub type ProdutoId = u32;

/// Grafo bipartido (clientes <-> produtos), não direcionado e ponderado.
/// Lista de adjacência guardada nos dois sentidos para percorrer rápido.
#[derive(Default)]
pub struct Grafo {
    compras_por_cliente: HashMap<ClienteId, Vec<(ProdutoId, f32)>>,
    compradores_por_produto: HashMap<ProdutoId, Vec<(ClienteId, f32)>>,
}

impl Grafo {
    pub fn new() -> Self {
        Self::default()
    }

    /// Cria a aresta cliente-produto. Ignora se ela já existir.
    pub fn registrar_compra(&mut self, cliente: ClienteId, produto: ProdutoId, peso: f32) {
        let lista = self.compras_por_cliente.entry(cliente).or_default();
        if lista.iter().any(|(p, _)| *p == produto) {
            return;
        }
        lista.push((produto, peso));
        self.compradores_por_produto
            .entry(produto)
            .or_default()
            .push((cliente, peso));
    }

    pub fn produtos_do_cliente(&self, cliente: ClienteId) -> &[(ProdutoId, f32)] {
        self.compras_por_cliente
            .get(&cliente)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn compradores_do_produto(&self, produto: ProdutoId) -> &[(ClienteId, f32)] {
        self.compradores_por_produto
            .get(&produto)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nao_duplica_aresta() {
        let mut g = Grafo::new();
        g.registrar_compra(1, 10, 1.0);
        g.registrar_compra(1, 10, 1.0);
        assert_eq!(g.produtos_do_cliente(1).len(), 1);
        assert_eq!(g.compradores_do_produto(10).len(), 1);
    }

    #[test]
    fn cliente_inexistente_retorna_vazio() {
        let g = Grafo::new();
        assert!(g.produtos_do_cliente(99).is_empty());
    }
}