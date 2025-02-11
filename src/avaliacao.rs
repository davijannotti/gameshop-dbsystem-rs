use crate::database::conectar_mysql;
use mysql::prelude::*;

pub fn listar_avaliacoes() {
    let mut conn = conectar_mysql();

    let query = "
        SELECT 
            a.id_avaliacao, 
            u.nome AS nome_usuario, 
            j.titulo AS nome_jogo, 
            a.nota, 
            a.comentario, 
            a.data_avaliacao 
        FROM Avaliacao a
        JOIN Usuario u ON a.id_usuario = u.id_usuario
        JOIN Jogo j ON a.id_jogo = j.id_jogo
    ";

    let avaliacoes: Vec<(i32, String, String, i32, Option<String>, String)> = conn
        .query_map(
            query,
            |(id, nome_usuario, nome_jogo, nota, comentario, data)| {
                (id, nome_usuario, nome_jogo, nota, comentario, data)
            },
        )
        .expect("Erro ao buscar avaliações");

    if avaliacoes.is_empty() {
        println!("Nenhuma avaliação encontrada.");
        return;
    }

    println!("--- Avaliações ---");
    for (id, nome_usuario, nome_jogo, nota, comentario, data) in avaliacoes {
        println!(
            "ID: {} | Usuário: {} | Jogo: {} | Nota: {} | Comentário: {} | Data: {}",
            id,
            nome_usuario,
            nome_jogo,
            nota,
            comentario.unwrap_or_else(|| "Sem comentário".to_string()),
            data
        );
    }
}

pub fn listar_avaliacoes_por_usuario(id_usuario: i32) -> Vec<(i32, String, i32, Option<String>)> {
    let mut conn = conectar_mysql();

    let query = "
        SELECT 
            a.id_avaliacao, 
            j.titulo AS nome_jogo, 
            a.nota, 
            a.comentario
        FROM Avaliacao a
        JOIN Jogo j ON a.id_jogo = j.id_jogo
        WHERE a.id_usuario = ?
    ";

    let avaliacoes: Vec<(i32, String, i32, Option<String>)> = conn
        .exec_map(query, (id_usuario,), |(id, nome_jogo, nota, comentario)| {
            (id, nome_jogo, nota, comentario)
        })
        .expect("Erro ao buscar avaliações do usuário");

    if avaliacoes.is_empty() {
        println!("Você ainda não fez nenhuma avaliação.");
    } else {
        println!("--- Suas Avaliações ---");
        for (id, nome_jogo, nota, comentario) in &avaliacoes {
            println!(
                "ID: {} | Jogo: {} | Nota: {} | Comentário: {}",
                id,
                nome_jogo,
                nota,
                comentario.as_deref().unwrap_or("Sem comentário")
            );
        }
    }

    return avaliacoes;
}

pub fn adicionar_avaliacao(id_usuario: i32, id_jogo: i32, nota: i32, comentario: Option<&str>) {
    let mut conn = conectar_mysql();
    let query = "INSERT INTO Avaliacao (id_usuario, id_jogo, nota, comentario, data_avaliacao) VALUES (?, ?, ?, ?, NOW())";

    conn.exec_drop(query, (id_usuario, id_jogo, nota, comentario))
        .expect("Erro ao inserir avaliação");

    println!("✅ Avaliação cadastrada com sucesso!");
}

pub fn atualizar_avaliacao(id_avaliacao: i32, nota: i32, comentario: Option<&str>) {
    let mut conn = conectar_mysql();
    let query = "UPDATE Avaliacao SET nota = ?, comentario = ?, data_avaliacao = NOW() WHERE id_avaliacao = ?";

    conn.exec_drop(query, (nota, comentario, id_avaliacao))
        .expect("Erro ao atualizar avaliação");

    println!("✅ Avaliação atualizada com sucesso!");
}

pub fn remover_avaliacao(id_avaliacao: i32) {
    let mut conn = conectar_mysql();
    let query = "DELETE FROM Avaliacao WHERE id_avaliacao = ?";

    conn.exec_drop(query, (id_avaliacao,))
        .expect("Erro ao remover avaliação");

    println!("✅ Avaliação removida com sucesso!");
}
