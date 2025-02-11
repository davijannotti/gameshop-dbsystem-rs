use crate::database::conectar_mysql;
use mysql::prelude::*;

pub fn listar_usuarios() -> Vec<(i32, String, String, String)> {
    let mut conn = conectar_mysql();
    let query = "SELECT id_usuario, nome, email, senha FROM Usuario";

    let usuarios: Vec<(i32, String, String, String)> = conn
        .query_map(query, |(id, nome, email, senha)| (id, nome, email, senha))
        .expect("Erro ao buscar usuários");

    for (id, nome, email, senha) in &usuarios {
        println!(
            "ID: {} | Nome: {} | Email: {} | Senha: {}",
            id, nome, email, senha
        );
    }

    return usuarios;
}

pub fn adicionar_usuario(nome: &str, email: &str, senha: &str) {
    let mut conn = conectar_mysql();
    let query = "INSERT INTO Usuario (nome, email, senha) VALUES (?, ?, ?)";

    conn.exec_drop(query, (nome, email, senha))
        .expect("Erro ao inserir usuário");

    println!("✅ Usuário cadastrado com sucesso!");
}

pub fn atualizar_usuario(id: i32, nome: &str, email: &str, senha: &str) {
    let mut conn = conectar_mysql();
    let mut query = "UPDATE Usuario SET".to_string();
    let mut params = Vec::new();

    if !nome.is_empty() {
        query.push_str(" nome = ?");
        params.push(nome);
    }
    if !email.is_empty() {
        if !params.is_empty() {
            query.push_str(",");
        }
        query.push_str(" email = ?");
        params.push(email);
    }
    if !senha.is_empty() {
        if !params.is_empty() {
            query.push_str(",");
        }
        query.push_str(" senha = ?");
        params.push(senha);
    }

    query.push_str(" WHERE id_usuario = ?");
    let binding = id.to_string();
    params.push(&binding);

    conn.exec_drop(query, params)
        .expect("Erro ao atualizar usuário");
    println!("✅ Usuário atualizado com sucesso!");
}

pub fn remover_usuario(id_usuario: i32) {
    let mut conn = conectar_mysql();
    let query = "DELETE FROM Usuario WHERE id_usuario = ?";

    conn.exec_drop(query, (id_usuario,))
        .expect("Erro ao remover usuário");

    println!("✅ Usuário removido com sucesso!");
}

pub fn is_admin(id_usuario: i32) -> bool {
    let mut conn = conectar_mysql();
    let query = "SELECT 1 FROM UsuarioAdmin WHERE id_usuario = ?";

    let result: Option<i32> = conn
        .exec_first(query, (id_usuario,))
        .expect("Erro ao verificar administrador");

    result.is_some()
}
