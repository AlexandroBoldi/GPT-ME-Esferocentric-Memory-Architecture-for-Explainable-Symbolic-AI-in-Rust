// gpt_me_prototipo.rs
// Protótipo simbólico baseado na Teoria da Memória Esferocêntrica em Rust

use std::collections::HashMap;

#[derive(Debug)]
struct Esfera {
    nome: String,
    coordenadas: (f32, f32, f32),
    conexoes: HashMap<String, u32>,
    historico: Vec<String>,
}

impl Esfera {
    fn new(nome: &str, coordenadas: (f32, f32, f32)) -> Self {
        Esfera {
            nome: nome.to_string(),
            coordenadas,
            conexoes: HashMap::new(),
            historico: Vec::new(),
        }
    }

    fn ativar(&mut self, contexto: &str) {
        println!("[Ativando] Esfera: {} | Coord: {:?}", self.nome, self.coordenadas);
        self.historico.push(contexto.to_string());
        for (conexao, peso) in &self.conexoes {
            println!("   ↪ Conectado a: {} (peso {})", conexao, peso);
        }
    }

    fn reforcar(&mut self, destino: &str) {
        let counter = self.conexoes.entry(destino.to_string()).or_insert(0);
        *counter += 1;
    }
}

fn main() {
    // Banco simbólico de esferas
    let mut memoria: HashMap<String, Esfera> = HashMap::new();
    memoria.insert("gato".to_string(), Esfera::new("animal.domestico.felino", (0.3, 0.6, 0.1)));
    memoria.insert("pulou".to_string(), Esfera::new("acao.saltar", (0.4, 0.5, 0.2)));
    memoria.insert("muro".to_string(), Esfera::new("objeto.obstaculo.vertical", (0.5, 0.5, 0.3)));

    let entrada = vec!["gato", "pulou", "muro"];
    let mut rota: Vec<String> = Vec::new();

    println!("=== Ativacao Simbolica GPT-ME ===\n");
    for token in &entrada {
        if let Some(esfera) = memoria.get_mut(&token.to_string()) {
            esfera.ativar("entrada");
            rota.push(esfera.nome.clone());
        }
    }

    println!("\n=== Reforcando Conexoes ===");
    for i in 0..entrada.len() - 1 {
        let origem_token = entrada[i];
        let destino_token = entrada[i + 1];
        if let Some(origem) = memoria.get_mut(&origem_token.to_string()) {
            if let Some(destino) = memoria.get(&destino_token.to_string()) {
                origem.reforcar(&destino.nome);
                println!("   [+] {} → {}", origem.nome, destino.nome);
            }
        }
    }

    println!("\n=== Rota Final da Luz ===");
    println!("{}", rota.join(" → "));
}
