use crate::database;
use mysql::prelude::*;

// Estrutura representando uma Conquista
#[derive(Debug)]
pub struct Conquista {
    pub id_jogo: i32,
    pub id_conquista: i32,
    pub nome: String,
    pub descricao: Option<String>,
}

// Função para listar todas as conquistas de um jogo
pub fn listar_conquistas(id_jogo: i32) {
    let mut conn = database::conectar_mysql();

    let query = "SELECT id_jogo, id_conquista, nome, descricao FROM Conquista WHERE id_jogo = ?";

    let conquistas: Vec<Conquista> = conn
        .exec_map(
            query,
            (id_jogo,),
            |(id_jogo, id_conquista, nome, descricao)| Conquista {
                id_jogo,
                id_conquista,
                nome,
                descricao,
            },
        )
        .expect("Erro ao buscar conquistas");

    if conquistas.is_empty() {
        println!("Nenhuma conquista encontrada para o jogo ID {}.", id_jogo);
    } else {
        println!("--- Conquistas do Jogo ID {} ---", id_jogo);
        for conquista in conquistas {
            println!(
                "ID Conquista: {} | Nome: {} | Descrição: {}",
                conquista.id_conquista,
                conquista.nome,
                conquista.descricao.as_deref().unwrap_or("Sem descrição")
            );
        }
    }
}

// Função para adicionar uma conquista a um jogo
pub fn adicionar_conquista(id_jogo: i32, nome: &str, descricao: &str) {
    let mut conn = database::conectar_mysql();

    // Descobrir o próximo ID da conquista dentro do jogo
    let query_proximo_id =
        "SELECT COALESCE(MAX(id_conquista), 0) + 1 FROM Conquista WHERE id_jogo = ?";
    let novo_id_conquista: i32 = conn
        .exec_first(query_proximo_id, (id_jogo,))
        .expect("Erro ao buscar próximo ID")
        .unwrap_or(1);

    // Inserir a nova conquista
    let query =
        "INSERT INTO Conquista (id_jogo, id_conquista, nome, descricao) VALUES (?, ?, ?, ?)";

    match conn.exec_drop(query, (id_jogo, novo_id_conquista, nome, descricao)) {
        Ok(_) => println!(
            "Conquista '{}' adicionada com sucesso ao jogo ID {}.",
            nome, id_jogo
        ),
        Err(err) => eprintln!("Erro ao adicionar conquista: {}", err),
    }
}

// Função para remover uma conquista de um jogo
pub fn remover_conquista(id_jogo: i32, id_conquista: i32) {
    let mut conn = database::conectar_mysql();

    let query = "DELETE FROM Conquista WHERE id_jogo = ? AND id_conquista = ?";

    match conn.exec_drop(query, (id_jogo, id_conquista)) {
        Ok(_) => println!(
            "Conquista ID {} removida do jogo ID {}.",
            id_conquista, id_jogo
        ),
        Err(err) => eprintln!("Erro ao remover conquista: {}", err),
    }
}
