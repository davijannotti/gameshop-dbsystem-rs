use crate::database::conectar_mysql;
use mysql::prelude::*;

pub fn listar_compras() {
    let mut conn = conectar_mysql();
    let query =
        "SELECT c.id_compra, c.id_usuario, j.titulo, c.data_compra, c.total, c.metodo_pagamento 
                 FROM Compra c 
                 JOIN Jogo j ON c.id_jogo = j.id_jogo";

    let compras: Vec<(i32, i32, String, String, f64, String)> = conn
        .query_map(query, |(id, usuario, titulo, data, total, pagamento)| {
            (id, usuario, titulo, data, total, pagamento)
        })
        .expect("Erro ao buscar compras");

    for (id, usuario, titulo, data, total, pagamento) in compras {
        println!(
            "ID: {} | Usuário: {} | Jogo: {} | Data: {} | Total: R${:.2} | Método: {}",
            id, usuario, titulo, data, total, pagamento
        );
    }
}

pub fn adicionar_compra(id_usuario: i32, id_jogo: i32, total: f64, metodo_pagamento: &str) {
    let mut conn = conectar_mysql();
    let query = "INSERT INTO Compra (id_usuario, id_jogo, data_compra, total, metodo_pagamento) VALUES (?, ?, NOW(), ?, ?)";

    conn.exec_drop(query, (id_usuario, id_jogo, total, metodo_pagamento))
        .expect("Erro ao inserir compra");

    println!("✅ Compra cadastrada com sucesso!");
}

pub fn atualizar_compra(id_compra: i32, total: Option<f64>, metodo_pagamento: Option<&str>) {
    let mut conn = conectar_mysql();
    let mut campos = Vec::new();
    let mut valores = Vec::new();

    if let Some(total) = total {
        campos.push("total = ?");
        valores.push(total.to_string());
    }
    if let Some(metodo_pagamento) = metodo_pagamento {
        campos.push("metodo_pagamento = ?");
        valores.push(metodo_pagamento.to_string());
    }

    if campos.is_empty() {
        println!("Nenhum campo para atualizar.");
        return;
    }

    let query = format!(
        "UPDATE Compra SET {} WHERE id_compra = ?",
        campos.join(", ")
    );
    valores.push(id_compra.to_string());
    conn.exec_drop(query, valores)
        .expect("Erro ao atualizar compra");

    println!("✅ Compra atualizada com sucesso!");
}

pub fn remover_compra(id_compra: i32) {
    let mut conn = conectar_mysql();
    let query = "DELETE FROM Compra WHERE id_compra = ?";

    conn.exec_drop(query, (id_compra,))
        .expect("Erro ao remover compra");
}
