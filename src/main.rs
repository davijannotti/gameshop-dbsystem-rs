mod avaliacao;
mod compra;
mod conquista;
mod database;
mod jogo;
mod usuarios;
use database::UserRole;
use inquire::Select;
use mysql::prelude::Queryable;
use usuarios::{usuario_admin::menu_principal_admin, usuario_comum::menu_usuario_comum};

pub fn criar_conta() {
    let nome = inquire::Text::new("Nome do novo usuário:")
        .prompt()
        .unwrap();
    let email = inquire::Text::new("Email do novo usuário:")
        .prompt()
        .unwrap();
    let senha = inquire::Text::new("Senha do novo usuário:")
        .prompt()
        .unwrap();

    let is_admin = Select::new("O novo usuário será um admin?", vec!["Sim", "Não"])
        .prompt()
        .unwrap()
        == "Sim";

    let mut conn = database::conectar_mysql();
    let query = "INSERT INTO Usuario (nome, email, senha) VALUES (?, ?, ?)";

    conn.exec_drop(query, (nome.clone(), email.clone(), senha.clone()))
        .expect("Erro ao criar conta");

    println!("✅ Conta criada com sucesso!");

    if is_admin {
        let query_admin = "INSERT INTO UsuarioAdmin (id_usuario, nivel_acesso) 
                            VALUES ((SELECT id_usuario FROM Usuario WHERE email = ?), ?)";
        conn.exec_drop(query_admin, (email, "Admin"))
            .expect("Erro ao atribuir privilégio de admin");

        println!("✅ O novo usuário é agora um administrador!");
    }
}

fn tela_inicial() {
    let opcoes = vec!["Criar Conta", "Fazer Login", "Sair"];

    loop {
        let escolha = Select::new("Escolha uma opção:", opcoes.clone()).prompt();

        match escolha {
            Ok("Criar Conta") => criar_conta(),
            Ok("Fazer Login") => set_menu(),
            Ok("Sair") => break,
            _ => println!("Opção inválida"),
        }
    }
}

fn set_menu() {
    match database::login() {
        Some((id_usuario_logado, nome, role)) => {
            println!("Bem-vindo, {}", nome);

            match role {
                UserRole::Admin => menu_principal_admin(id_usuario_logado),
                UserRole::Comum => menu_usuario_comum(id_usuario_logado),
            }
        }
        None => {
            println!("Falha no login. Tente novamente.");
        }
    }
}

fn main() {
    tela_inicial();
}
