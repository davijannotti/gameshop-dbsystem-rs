mod avaliacao;
mod compra;
mod conquista;
mod database;
mod jogo;
mod usuario;
use database::UserRole;
use inquire::Select;
use mysql::prelude::Queryable;
use usuario::listar_usuarios;

pub fn criar_conta(admin_id: i32) {
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

    // Se o admin estiver criando um novo admin, atribuir nível de acesso
    if is_admin {
        let query_admin = "INSERT INTO UsuarioAdmin (id_usuario, nivel_acesso) 
                            VALUES ((SELECT id_usuario FROM Usuario WHERE email = ?), ?)";
        conn.exec_drop(query_admin, (email, "Admin"))
            .expect("Erro ao atribuir privilégio de admin");

        println!("✅ O novo usuário é agora um administrador!");
    }
}

fn menu_principal() {
    match database::login() {
        Some((id_usuario_logado, nome, role)) => {
            println!("Bem-vindo, {}", nome);

            // Se o usuário não for Admin, não pode acessar o sistema
            if role != UserRole::Admin {
                println!("❌ Você precisa ser um administrador para acessar o sistema.");
                return; // Encerra a função e impede o acesso ao menu
            }

            let opcoes = vec!["Usuário", "Jogo", "Criar Conta", "Sair"];

            loop {
                let escolha = Select::new("Escolha uma categoria:", opcoes.clone()).prompt();

                match escolha {
                    Ok("Usuário") => menu_usuario(id_usuario_logado),
                    Ok("Jogo") => menu_jogo(),
                    Ok("Criar Conta") => criar_conta(id_usuario_logado),
                    Ok("Sair") => break,
                    _ => println!("Opção inválida"),
                }
            }
        }
        None => {
            println!("Falha no login. Tente novamente.");
        }
    }
}

// Menu CRUD para Usuários
pub fn menu_usuario(id_usuario_logado: i32) {
    // Verificando se o usuário logado é administrador
    let is_admin = usuario::is_admin(id_usuario_logado);

    let opcoes = if is_admin {
        vec![
            "Listar Usuários",
            "Adicionar Usuário",
            "Atualizar Usuário",
            "Remover Usuário",
            "Compra",
            "Voltar",
        ]
    } else {
        vec!["Listar Usuários", "Atualizar Meu Usuário", "Voltar"]
    };

    loop {
        let escolha = Select::new("Gerenciamento de Usuário:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Usuários") => {
                listar_usuarios();
            }
            Ok("Adicionar Usuário") if is_admin => {
                let nome = inquire::Text::new("Nome:").prompt().unwrap();
                let email = inquire::Text::new("Email:").prompt().unwrap();
                let senha = inquire::Text::new("Senha:").prompt().unwrap();
                usuario::adicionar_usuario(&nome, &email, &senha);
            }
            Ok("Atualizar Meu Usuário") => {
                let usuarios = listar_usuarios();
                let usuarios_nomes: Vec<String> = usuarios.iter().map(|u| u.1.clone()).collect();
                let escolha_usuario = Select::new("Escolha um Usuário:", usuarios_nomes)
                    .prompt()
                    .unwrap();

                let id_usuario = usuarios
                    .iter()
                    .find(|u| u.1 == escolha_usuario)
                    .map(|u| u.0)
                    .unwrap();

                loop {
                    let opcoes = vec![
                        "Atualizar Nome do Usuário",
                        "Atualizar E-mail do Usuário",
                        "Atualizar Senha do Usuário",
                        "Voltar",
                    ];

                    let escolha = Select::new("Gerenciamento de Usuário:", opcoes.clone()).prompt();

                    match escolha {
                        Ok("Atualizar Nome do Usuário") => {
                            let novo_nome = inquire::Text::new("Novo Nome:").prompt().unwrap();
                            usuario::atualizar_usuario(id_usuario, &novo_nome, "", "");
                        }
                        Ok("Atualizar E-mail do Usuário") => {
                            let novo_email = inquire::Text::new("Novo E-mail:").prompt().unwrap();
                            usuario::atualizar_usuario(id_usuario, "", &novo_email, "");
                        }
                        Ok("Atualizar Senha do Usuário") => {
                            let nova_senha = inquire::Text::new("Nova Senha:").prompt().unwrap();
                            usuario::atualizar_usuario(id_usuario, "", "", &nova_senha);
                        }
                        Ok("Voltar") => break,
                        _ => println!("Opção inválida"),
                    }
                }
            }
            Ok("Remover Usuário") if is_admin => {
                let usuarios = listar_usuarios();
                let usuarios_nomes: Vec<String> = usuarios.iter().map(|u| u.1.clone()).collect();
                let escolha_usuario =
                    Select::new("Escolha um Usuário para Remover:", usuarios_nomes)
                        .prompt()
                        .unwrap();
                let id_usuario = usuarios
                    .iter()
                    .find(|u| u.1 == escolha_usuario)
                    .map(|u| u.0)
                    .unwrap();
                usuario::remover_usuario(id_usuario);
            }
            Ok("Compra") => {
                menu_compras();
            }
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

// Menu CRUD para Jogos, incluindo Avaliações e Conquistas
fn menu_jogo() {
    let opcoes = vec![
        "Listar Jogos",
        "Adicionar Jogo",
        "Atualizar Jogo",
        "Remover Jogo",
        "Gerenciar Conquistas",
        "Gerenciar Avaliações",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Jogos:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Jogos") => menu_listar_jogos(),
            Ok("Adicionar Jogo") => {
                let titulo = inquire::Text::new("Título:").prompt().unwrap();
                let descricao = inquire::Text::new("Descrição:").prompt().unwrap();
                let desenvolvedor = inquire::Text::new("Desenvolvedor:").prompt().unwrap();
                let preco: f64 = inquire::Text::new("Preço:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                jogo::adicionar_jogo(&titulo, &descricao, &desenvolvedor, preco);
            }
            Ok("Atualizar Jogo") => {
                let id: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let novo_titulo = inquire::Text::new("Novo Título:").prompt().unwrap();
                let descricao = inquire::Text::new("Descrição do jogo:").prompt().unwrap();
                let desenvolvedor = inquire::Text::new("Desenvolvedor do jogo:")
                    .prompt()
                    .unwrap();
                let preco: f64 = inquire::Text::new("Preço do jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                jogo::atualizar_jogo(id, &novo_titulo, &descricao, &desenvolvedor, preco);
            }
            Ok("Remover Jogo") => {
                let id: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                jogo::remover_jogo(id);
            }
            Ok("Gerenciar Conquistas") => menu_conquistas(),
            Ok("Gerenciar Avaliações") => menu_avaliacoes(),
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

fn menu_listar_jogos() {
    let opcoes = vec![
        "Listar Todos os Jogos",
        "Listar Jogos Mais Bem Avaliados",
        "Listar Jogos Mais Vendidos",
        "Listar Jogos Mais Caros",
        "Listar Jogos Mais Recentes",
        "Listar Jogos com Mais Conquistas",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Escolha uma opção:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Todos os Jogos") => jogo::listar_jogos(),
            Ok("Listar Jogos Mais Bem Avaliados") => jogo::listar_jogos_mais_bem_avaliados(),
            Ok("Listar Jogos Mais Vendidos") => jogo::listar_jogos_mais_vendidos(),
            Ok("Listar Jogos Mais Caros") => jogo::listar_jogos_mais_caros(),
            Ok("Listar Jogos Mais Recentes") => jogo::listar_jogos_mais_recentes(),
            Ok("Listar Jogos com Mais Conquistas") => jogo::listar_jogos_com_mais_conquistas(),
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

// Menu CRUD para Conquistas de um Jogo
fn menu_conquistas() {
    let opcoes = vec![
        "Listar Conquistas",
        "Adicionar Conquista",
        "Remover Conquista",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Conquistas:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Conquistas") => {
                let id_jogo: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                conquista::listar_conquistas(id_jogo);
            }
            Ok("Adicionar Conquista") => {
                let id_jogo: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let nome = inquire::Text::new("Nome da Conquista:").prompt().unwrap();
                let descricao = inquire::Text::new("Descrição:").prompt().unwrap();
                conquista::adicionar_conquista(id_jogo, &nome, &descricao);
            }
            Ok("Remover Conquista") => {
                let id_jogo: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let id_conquista: i32 = inquire::Text::new("ID da Conquista:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                conquista::remover_conquista(id_jogo, id_conquista);
            }
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

// Menu CRUD para Avaliações de um Jogo
fn menu_avaliacoes() {
    let opcoes = vec![
        "Listar Avaliações",
        "Adicionar Avaliação",
        "Remover Avaliação",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Avaliações:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Avaliações") => avaliacao::listar_avaliacoes(),
            Ok("Adicionar Avaliação") => {
                let id_usuario: i32 = inquire::Text::new("ID do Usuário:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let id_jogo: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let nota: i32 = inquire::Text::new("Nota (1-10):")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let comentario = inquire::Text::new("Comentário:").prompt().unwrap();
                avaliacao::adicionar_avaliacao(id_usuario, id_jogo, nota, Some(&comentario));
            }
            Ok("Remover Avaliação") => {
                let id_avaliacao: i32 = inquire::Text::new("ID da Avaliação:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                avaliacao::remover_avaliacao(id_avaliacao);
            }
            Ok("Atualizar Avaliação") => {
                let id_avaliacao: i32 = inquire::Text::new("ID da Avaliação:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let nota: i32 = inquire::Text::new("Nova Nota (1-10):")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let comentario = inquire::Text::new("Novo Comentário:").prompt().unwrap();
                avaliacao::atualizar_avaliacao(id_avaliacao, nota, Some(&comentario));
            }
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_compras() {
    let opcoes = vec![
        "Listar Compras",
        "Adicionar Compra",
        "Atualizar Compra",
        "Remover Compra",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Compras:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Compras") => {
                compra::listar_compras();
            }
            Ok("Adicionar Compra") => {
                let id_usuario: i32 = inquire::Text::new("ID do Usuário:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let id_jogo: i32 = inquire::Text::new("ID do Jogo:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let total: f64 = inquire::Text::new("Total da Compra:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let metodo_pagamento = inquire::Text::new("Método de Pagamento:").prompt().unwrap();
                compra::adicionar_compra(id_usuario, id_jogo, total, &metodo_pagamento);
            }
            Ok("Atualizar Compra") => {
                let id_compra: i32 = inquire::Text::new("ID da Compra:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let total: Option<f64> =
                    inquire::Text::new("Novo Total da Compra (deixe vazio para não alterar):")
                        .prompt()
                        .ok()
                        .and_then(|s| s.parse().ok());
                let metodo_pagamento: Option<String> =
                    inquire::Text::new("Novo Método de Pagamento (deixe vazio para não alterar):")
                        .prompt()
                        .ok();
                compra::atualizar_compra(id_compra, total, metodo_pagamento.as_deref());
            }
            Ok("Remover Compra") => {
                let id_compra: i32 = inquire::Text::new("ID da Compra:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                compra::remover_compra(id_compra);
            }
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}
fn main() {
    menu_principal();
}
