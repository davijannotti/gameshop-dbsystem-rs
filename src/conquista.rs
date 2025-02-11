use crate::database;
use mysql::prelude::*;

pub fn listar_conquistas(id_jogo: i32) -> Vec<(i32, i32, String, Option<String>)> {
    let mut conn = database::conectar_mysql();

    let query = "SELECT id_jogo, id_conquista, nome, descricao FROM Conquista WHERE id_jogo = ?";

    let conquistas: Vec<(i32, i32, String, Option<String>)> = conn
        .exec_map(
            query,
            (id_jogo,),
            |(id_jogo, id_conquista, nome, descricao): (i32, i32, String, Option<String>)| {
                (id_jogo, id_conquista, nome, descricao)
            },
        )
        .expect("Erro ao buscar conquistas");

    if conquistas.is_empty() {
        println!("Nenhuma conquista encontrada para o jogo ID {}.", id_jogo);
    } else {
        println!("--- Conquistas do Jogo ID {} ---", id_jogo);
        for conquista in &conquistas {
            println!(
                "ID Conquista: {} | Nome: {} | Descrição: {}",
                conquista.1,
                conquista.2,
                conquista.3.as_deref().unwrap_or("Sem descrição")
            );
        }
    }

    conquistas
}

pub fn adicionar_conquista(id_jogo: i32, nome: &str, descricao: &str) {
    let mut conn = database::conectar_mysql();

    let query_proximo_id =
        "SELECT COALESCE(MAX(id_conquista), 0) + 1 FROM Conquista WHERE id_jogo = ?";
    let novo_id_conquista: i32 = conn
        .exec_first(query_proximo_id, (id_jogo,))
        .expect("Erro ao buscar próximo ID")
        .unwrap_or(1);

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

pub fn atualizar_conquista(
    id_jogo: i32,
    id_conquista: i32,
    novo_nome: Option<String>,
    nova_descricao: Option<String>,
) {
    let mut conn = database::conectar_mysql();

    let mut query = "UPDATE Conquista SET".to_string();
    let mut params: Vec<(String, String)> = Vec::new();

    if let Some(nome) = novo_nome {
        query.push_str(" nome = ?,");
        params.push(("nome".to_string(), nome));
    }
    if let Some(descricao) = nova_descricao {
        query.push_str(" descricao = ?,");
        params.push(("descricao".to_string(), descricao));
    }

    query.pop();

    query.push_str(" WHERE id_jogo = ? AND id_conquista = ?");

    params.push(("id_jogo".to_string(), id_jogo.to_string()));
    params.push(("id_conquista".to_string(), id_conquista.to_string()));

    match conn.exec_drop(
        query,
        (
            params[0].1.as_str(),
            params[1].1.as_str(),
            params[2].1.as_str(),
            params[3].1.as_str(),
        ),
    ) {
        Ok(_) => println!(
            "Conquista ID {} do jogo ID {} atualizada com sucesso.",
            id_conquista, id_jogo
        ),
        Err(err) => eprintln!("Erro ao atualizar conquista: {}", err),
    }
}
