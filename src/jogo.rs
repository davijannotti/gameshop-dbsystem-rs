use crate::database::conectar_mysql;
use mysql::prelude::*;

pub fn listar_jogos() -> Vec<(i32, String, f64, String)> {
    let mut conn = conectar_mysql();
    let query = "SELECT id_jogo, titulo, preco, data_lancamento FROM Jogo";

    let jogos: Vec<(i32, String, f64, String)> = conn
        .query_map(query, |(id, titulo, preco, data_lancamento)| {
            (id, titulo, preco, data_lancamento)
        })
        .expect("Erro ao buscar jogos");

    if jogos.is_empty() {
        println!("Nenhum jogo disponível para compra.");
    } else {
        println!("--- Jogos Disponíveis para Compra ---");
        for (id, titulo, preco, data_lancamento) in &jogos {
            println!(
                "ID: {} | Título: {} | Preço: {} | Data de Lançamento: {}",
                id, titulo, preco, data_lancamento
            );
        }
    }

    jogos
}

pub fn adicionar_jogo(
    titulo: &str,
    descricao: &str,
    desenvolvedor: &str,
    preco: f64,
    data_lancamento: &str,
) {
    let mut conn = conectar_mysql();

    let query = "INSERT INTO Jogo (titulo, descricao, desenvolvedor, data_lancamento, preco) VALUES (?, ?, ?, ?, ?)";

    conn.exec_drop(
        query,
        (titulo, descricao, desenvolvedor, data_lancamento, preco),
    )
    .expect("Erro ao inserir jogo");

    println!("✅ Jogo cadastrado com sucesso!");
}

pub fn atualizar_jogo(
    id_jogo: i32,
    titulo: &str,
    descricao: &str,
    desenvolvedor: &str,
    preco: f64,
    data_lancamento: &str, 
) {
    let mut conn = conectar_mysql();

    let query = "
        UPDATE Jogo 
        SET titulo = ?, descricao = ?, desenvolvedor = ?, preco = ?, data_lancamento = ? 
        WHERE id_jogo = ?
    ";

    conn.exec_drop(
        query,
        (
            titulo,
            descricao,
            desenvolvedor,
            preco,
            data_lancamento,
            id_jogo,
        ),
    )
    .expect("Erro ao atualizar jogo");

    println!("✅ Jogo atualizado com sucesso!");
}

pub fn remover_jogo(id_jogo: i32) {
    let mut conn = conectar_mysql();
    let query = "DELETE FROM Jogo WHERE id_jogo = ?";

    conn.exec_drop(query, (id_jogo,))
        .expect("Erro ao remover jogo");

    println!("✅ Jogo removido com sucesso!");
}

pub fn listar_jogos_mais_bem_avaliados() {
    let mut conn = conectar_mysql();
    let query = "
        SELECT J.id_jogo, J.titulo, COALESCE(AVG(A.nota), 0) AS media_notas
        FROM Jogo J
        LEFT JOIN Avaliacao A ON J.id_jogo = A.id_jogo
        GROUP BY J.id_jogo, J.titulo
        ORDER BY media_notas DESC
        LIMIT 10
    ";

    let jogos: Vec<(i32, String, f64)> = conn
        .query_map(query, |(id, titulo, media)| (id, titulo, media))
        .expect("Erro ao buscar jogos mais bem avaliados");

    println!("🎮 Jogos Mais Bem Avaliados:");
    for (id, titulo, media) in jogos {
        println!("ID: {} | Título: {} | Nota Média: {:.2}", id, titulo, media);
    }
}

pub fn listar_jogos_mais_vendidos() {
    let mut conn = conectar_mysql();
    let query = "
        SELECT J.id_jogo, J.titulo, COUNT(C.id_compra) AS vendas
        FROM Jogo J
        LEFT JOIN Compra C ON J.id_jogo = C.id_jogo
        GROUP BY J.id_jogo, J.titulo
        ORDER BY vendas DESC
        LIMIT 10
    ";

    let jogos: Vec<(i32, String, i32)> = conn
        .query_map(query, |(id, titulo, vendas)| (id, titulo, vendas))
        .expect("Erro ao buscar jogos mais vendidos");

    println!("💰 Jogos Mais Vendidos:");
    for (id, titulo, vendas) in jogos {
        println!("ID: {} | Título: {} | Vendas: {}", id, titulo, vendas);
    }
}

pub fn listar_jogos_mais_caros() {
    let mut conn = conectar_mysql();
    let query = "SELECT id_jogo, titulo, preco FROM Jogo ORDER BY preco DESC LIMIT 10";

    let jogos: Vec<(i32, String, f64)> = conn
        .query_map(query, |(id, titulo, preco)| (id, titulo, preco))
        .expect("Erro ao buscar jogos mais caros");

    println!("💸 Jogos Mais Caros:");
    for (id, titulo, preco) in jogos {
        println!("ID: {} | Título: {} | Preço: R${:.2}", id, titulo, preco);
    }
}

pub fn listar_jogos_mais_recentes() {
    let mut conn = conectar_mysql();
    let query =
        "SELECT id_jogo, titulo, data_lancamento FROM Jogo ORDER BY data_lancamento DESC LIMIT 10";

    let jogos: Vec<(i32, String, String)> = conn
        .query_map(query, |(id, titulo, data)| (id, titulo, data))
        .expect("Erro ao buscar jogos mais recentes");

    println!("🆕 Jogos Mais Recentes:");
    for (id, titulo, data) in jogos {
        println!("ID: {} | Título: {} | Lançamento: {}", id, titulo, data);
    }
}

pub fn listar_jogos_com_mais_conquistas() {
    let mut conn = conectar_mysql();
    let query = "
        SELECT J.id_jogo, J.titulo, COUNT(C.id_conquista) AS total_conquistas
        FROM Jogo J
        LEFT JOIN Conquista C ON J.id_jogo = C.id_jogo
        GROUP BY J.id_jogo, J.titulo
        ORDER BY total_conquistas DESC
        LIMIT 10
    ";

    let jogos: Vec<(i32, String, i32)> = conn
        .query_map(query, |(id, titulo, total)| (id, titulo, total))
        .expect("Erro ao buscar jogos com mais conquistas");

    println!("🏆 Jogos com Mais Conquistas:");
    for (id, titulo, total) in jogos {
        println!("ID: {} | Título: {} | Conquistas: {}", id, titulo, total);
    }
}
