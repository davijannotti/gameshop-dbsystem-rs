use inquire::Select;

use crate::{
    avaliacao,
    compra::{self, listar_compras_por_usuario},
    conquista::{self, listar_conquistas},
    criar_conta,
    jogo::{self, listar_jogos},
};

use super::usuario::{self, listar_usuarios};

pub fn menu_principal_admin(id_usuario_logado: i32) {
    let opcoes = vec![
        "Gerenciar Usuários",
        "Gerenciar Jogos",
        "Gerenciar Compras",
        "Gerenciar Conquistas",
        "Gerenciar Avaliações",
        "Sair",
    ];

    loop {
        let escolha = Select::new("Escolha uma categoria:", opcoes.clone()).prompt();

        match escolha {
            Ok("Gerenciar Usuários") => menu_usuario_admin(id_usuario_logado),
            Ok("Gerenciar Jogos") => menu_jogo_admin(),
            Ok("Gerenciar Compras") => {
                menu_compras_admin();
            }
            Ok("Gerenciar Conquistas") => menu_conquistas_admin(),
            Ok("Gerenciar Avaliações") => menu_avaliacoes_admin(),
            Ok("Sair") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_usuario_admin(id_usuario_logado: i32) {
    let is_admin = usuario::is_admin(id_usuario_logado);

    let opcoes = if is_admin {
        vec![
            "Listar Usuários",
            "Adicionar Usuário",
            "Atualizar Usuário",
            "Remover Usuário",
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
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_jogo_admin() {
    let opcoes = vec![
        "Listar Jogos",
        "Adicionar Jogo",
        "Atualizar Jogo",
        "Remover Jogo",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Jogos:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Jogos") => menu_listar_jogos_admin(),
            Ok("Adicionar Jogo") => {
                let titulo = inquire::Text::new("Título:").prompt().unwrap();
                let descricao = inquire::Text::new("Descrição:").prompt().unwrap();
                let desenvolvedor = inquire::Text::new("Desenvolvedor:").prompt().unwrap();
                let preco: f64 = inquire::Text::new("Preço:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let data_lancamento = inquire::Text::new("Data de Lançamento(YYYY-MM-DD)")
                    .prompt()
                    .unwrap();
                jogo::adicionar_jogo(&titulo, &descricao, &desenvolvedor, preco, &data_lancamento);
            }
            Ok("Atualizar Jogo") => {
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|j| j.1.clone()).collect();

                if jogos_nomes.is_empty() {
                    println!("Nenhum jogo disponível para atualizar.");
                    continue;
                }

                let escolha_jogo = Select::new("Escolha um Usuário para Remover:", jogos_nomes)
                    .prompt()
                    .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|j| j.1 == escolha_jogo)
                    .map(|j| j.0)
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
                let data_lancamento = inquire::Text::new("Data de Lançamento(YYYY-MM-DD)")
                    .prompt()
                    .unwrap();
                jogo::atualizar_jogo(
                    id_jogo,
                    &novo_titulo,
                    &descricao,
                    &desenvolvedor,
                    preco,
                    &data_lancamento,
                );
            }
            Ok("Remover Jogo") => {
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|j| j.1.clone()).collect();

                if jogos_nomes.is_empty() {
                    println!("Nenhum jogo disponível para atualizar.");
                    continue;
                }

                let escolha_jogo = Select::new("Escolha um jogo para atualizar:", jogos_nomes)
                    .prompt()
                    .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|j| j.1 == escolha_jogo)
                    .map(|j| j.0)
                    .unwrap();
                jogo::remover_jogo(id_jogo);
            }

            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_compras_admin() {
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
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|j| j.1.clone()).collect();

                if jogos_nomes.is_empty() {
                    println!("Nenhum jogo disponível para compra.");
                    continue;
                }

                let escolha_jogo = Select::new("Escolha um jogo para comprar:", jogos_nomes)
                    .prompt()
                    .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|j| j.1 == escolha_jogo)
                    .map(|j| j.0)
                    .unwrap();

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

                let preco_jogo = jogos
                    .iter()
                    .find(|j| j.1 == escolha_jogo)
                    .map(|j| j.2)
                    .unwrap();

                let metodo_pagamento = inquire::Text::new("Método de Pagamento:").prompt().unwrap();

                compra::adicionar_compra(id_usuario, id_jogo, preco_jogo, &metodo_pagamento);
            }
            Ok("Atualizar Compra") => {
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

                let compras = listar_compras_por_usuario(id_usuario);

                let compras_info: Vec<String> = compras
                    .iter()
                    .map(|(id, titulo)| format!("ID do Jogo: {} | Título: {}", id, titulo))
                    .collect();

                let escolha_compra = Select::new("Escolha uma Compra:", compras_info)
                    .prompt()
                    .unwrap();

                let id_compra = compras
                    .iter()
                    .find(|(id, titulo)| {
                        let escolha = format!("ID do Jogo: {} | Título: {}", id, titulo);
                        escolha == escolha_compra
                    })
                    .map(|(id, _)| *id)
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

                let compras = listar_compras_por_usuario(id_usuario);

                let compras_info: Vec<String> = compras
                    .iter()
                    .map(|(id, titulo)| format!("ID do Jogo: {} | Título: {}", id, titulo))
                    .collect();

                let escolha_compra = Select::new("Escolha uma Compra:", compras_info)
                    .prompt()
                    .unwrap();

                let id_compra = compras
                    .iter()
                    .find(|(id, titulo)| {
                        let escolha = format!("ID do Jogo: {} | Título: {}", id, titulo);
                        escolha == escolha_compra
                    })
                    .map(|(id, _)| *id)
                    .unwrap();

                compra::remover_compra(id_compra);
            }

            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_listar_jogos_admin() {
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
            Ok("Listar Todos os Jogos") => {
                jogo::listar_jogos();
            }
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

pub fn menu_conquistas_admin() {
    let opcoes = vec![
        "Listar Conquistas",
        "Adicionar Conquista",
        "Atualizar Conquista",
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
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|j| j.1.clone()).collect();

                if jogos_nomes.is_empty() {
                    println!("Nenhum jogo disponível.");
                    continue;
                }

                let escolha_jogo =
                    Select::new("Escolha um jogo para adicionar uma conquista:", jogos_nomes)
                        .prompt()
                        .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|j| j.1 == escolha_jogo)
                    .map(|j| j.0)
                    .unwrap();

                let nome = inquire::Text::new("Nome da Conquista:").prompt().unwrap();
                let descricao = inquire::Text::new("Descrição:").prompt().unwrap();
                conquista::adicionar_conquista(id_jogo, &nome, &descricao);
            }
            Ok("Atualizar Conquista") => {
                let jogos = listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|u| u.1.clone()).collect();
                let escolha_jogo = Select::new("Escolha um Jogo:", jogos_nomes)
                    .prompt()
                    .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|u| u.1 == escolha_jogo)
                    .map(|u| u.0)
                    .unwrap();

                let conquistas = listar_conquistas(id_jogo);
                let conquistas_info: Vec<String> = conquistas
                    .iter()
                    .map(|(_, _, nome, _)| nome.clone())
                    .collect();

                let escolha_conquista = Select::new("Escolha uma Conquista:", conquistas_info)
                    .prompt()
                    .unwrap();

                let id_conquista = conquistas
                    .iter()
                    .find(|(_, _, nome, _)| *nome == escolha_conquista)
                    .map(|(_, id_conquista, _, _)| *id_conquista)
                    .unwrap();

                let novo_nome: Option<String> =
                    inquire::Text::new("Novo Nome da Conquista (deixe vazio para não alterar):")
                        .prompt()
                        .ok()
                        .and_then(|s| s.parse().ok());
                let nova_descricao: Option<String> =
                    inquire::Text::new("Nova Descrição (deixe vazio para não alterar):")
                        .prompt()
                        .ok();
                conquista::atualizar_conquista(id_jogo, id_conquista, novo_nome, nova_descricao);
            }
            Ok("Remover Conquista") => {
                let jogos = listar_jogos();
                let jogos_info: Vec<String> = jogos
                    .iter()
                    .map(|(_, titulo, _, _)| titulo.clone())
                    .collect();

                let escolha_jogo = Select::new("Escolha um Jogo:", jogos_info)
                    .prompt()
                    .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|(_, titulo, _, _)| *titulo == escolha_jogo)
                    .map(|(id, _, _, _)| *id)
                    .unwrap();

                let conquistas = listar_conquistas(id_jogo);
                let conquistas_info: Vec<String> = conquistas
                    .iter()
                    .map(|(_, _, nome, _)| nome.clone())
                    .collect();

                let escolha_conquista = Select::new("Escolha uma Conquista:", conquistas_info)
                    .prompt()
                    .unwrap();

                let id_conquista = conquistas
                    .iter()
                    .find(|(_, _, nome, _)| *nome == escolha_conquista)
                    .map(|(_, id_conquista, _, _)| *id_conquista)
                    .unwrap();

                conquista::remover_conquista(id_jogo, id_conquista);
            }

            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_avaliacoes_admin() {
    let opcoes = vec![
        "Listar Avaliações",
        "Adicionar Avaliações",
        "Remover Avaliações",
        "Atualizar Avaliações",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Avaliações:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Avaliações") => avaliacao::listar_avaliacoes(),
            Ok("Adicionar Avaliações") => {
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|c| c.1.clone()).collect();

                let escolha_jogo =
                    Select::new("Escolha um jogo para adicionar avaliação:", jogos_nomes)
                        .prompt()
                        .unwrap();

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

                let id_jogo = jogos
                    .iter()
                    .find(|c| c.1 == escolha_jogo)
                    .map(|c| c.0)
                    .unwrap();

                let nota: i32 = inquire::Text::new("Nota (1-10):")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let comentario = inquire::Text::new("Comentário:").prompt().unwrap();

                avaliacao::adicionar_avaliacao(id_usuario, id_jogo, nota, Some(&comentario));
            }
            Ok("Remover Avaliações") => {
                // Listar todos os jogos
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|c| c.1.clone()).collect();
            
                // Permitir que o usuário escolha um jogo
                let escolha_jogo = Select::new("Escolha um Jogo:", jogos_nomes)
                    .prompt()
                    .unwrap();
            
                // Encontrar o ID do jogo escolhido
                let id_jogo = jogos
                    .iter()
                    .find(|c| c.1 == escolha_jogo)
                    .map(|c| c.0)
                    .unwrap();
            
                // Listar avaliações do jogo escolhido
                let avaliacoes = avaliacao::listar_avaliacoes_por_jogo(id_jogo);
                let avaliacoes_info: Vec<String> = avaliacoes
                    .iter()
                    .map(|(id, nota, comentario)| {
                        format!(
                            "ID: {} | Nota: {} | Comentário: {}",
                            id,
                            nota,
                            comentario.as_deref().unwrap_or("Sem comentário")
                        )
                    })
                    .collect();
            
                // Permitir que o usuário escolha uma avaliação
                let escolha_avaliacao = Select::new("Escolha uma Avaliação:", avaliacoes_info)
                    .prompt()
                    .unwrap();
            
                // Encontrar o ID da avaliação escolhida
                let id_avaliacao = avaliacoes
                    .iter()
                    .find(|(id, nota, comentario)| {
                        format!("ID: {} | Nota: {} | Comentário: {}", id, nota, comentario.as_deref().unwrap_or("Sem comentário")) == escolha_avaliacao
                    })
                    .map(|(id, _, _)| *id)
                    .unwrap();
            
                // Remover a avaliação escolhida
                avaliacao::remover_avaliacao(id_avaliacao);
            }
            
            Ok("Atualizar Avaliações") => {
                let jogos = jogo::listar_jogos();
                let jogos_nomes: Vec<String> = jogos.iter().map(|c| c.1.clone()).collect();

                let escolha_jogo =
                    Select::new("Escolha um jogo para atualizar avaliação:", jogos_nomes)
                        .prompt()
                        .unwrap();

                let id_jogo = jogos
                    .iter()
                    .find(|c| c.1 == escolha_jogo)
                    .map(|c| c.0)
                    .unwrap();

                let avaliacoes = avaliacao::listar_avaliacoes_por_jogo(id_jogo);
                let avaliacoes_info: Vec<String> = avaliacoes
                    .iter()
                    .map(|(id, nota, comentario)| {
                        format!(
                            "ID: {} | Nota: {} | Comentário: {}",
                            id,
                            nota,
                            comentario.as_deref().unwrap_or("Sem comentário")
                        )
                    })
                    .collect();

                let escolha_avaliacao =
                    Select::new("Escolha uma avaliação para atualizar:", avaliacoes_info)
                        .prompt()
                        .unwrap();

                let id_avaliacao = avaliacoes
                    .iter()
                    .find(|(id, nota, comentario)| format!("ID: {} | Nota: {} | Comentário: {}", id,nota, comentario.as_deref().unwrap_or("Sem comentário")) == escolha_avaliacao)
                    .map(|(id, _, _)| *id)
                    .unwrap();

                let nova_nota: i32 = inquire::Text::new("Nova Nota (1-10):")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                let novo_comentario = inquire::Text::new("Novo Comentário:").prompt().unwrap();

                avaliacao::atualizar_avaliacao(id_avaliacao, nova_nota, Some(&novo_comentario));
            }

            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}
