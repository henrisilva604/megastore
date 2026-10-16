#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

impl Produto {
    pub fn new(id: u32, nome: &str, categoria: &str, preco: f64) -> Self {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}