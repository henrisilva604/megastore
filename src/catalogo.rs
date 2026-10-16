use crate::produto::Produto;
use std::collections::HashMap;

pub struct Catalogo {
    produtos: HashMap<u32, Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo { produtos: HashMap::new() }
    }

    /// Retorna Err se já existir um produto com o mesmo id.
    pub fn cadastrar(&mut self, produto: Produto) -> Result<(), String> {
        if self.produtos.contains_key(&produto.id) {
            return Err(format!("Produto {} já cadastrado", produto.id));
        }
        self.produtos.insert(produto.id, produto);
        Ok(())
    }

    pub fn buscar(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn total(&self) -> usize {
        self.produtos.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cadastra_e_busca_produto() {
        let mut c = Catalogo::new();
        c.cadastrar(Produto::new(1, "Mouse", "Eletrônicos", 80.0)).unwrap();
        assert_eq!(c.total(), 1);
        assert_eq!(c.buscar(1).unwrap().nome, "Mouse");
        assert!(c.buscar(2).is_none());
    }

    #[test]
    fn rejeita_id_duplicado() {
        let mut c = Catalogo::new();
        c.cadastrar(Produto::new(1, "Mouse", "Eletrônicos", 80.0)).unwrap();
        let resultado = c.cadastrar(Produto::new(1, "Outro", "Livros", 50.0));
        assert!(resultado.is_err());
        assert_eq!(c.total(), 1);
    }
}