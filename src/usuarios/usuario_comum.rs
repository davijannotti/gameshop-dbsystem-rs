use inquire::Select;

use crate::{avaliacao, compra, jogo};

pub fn menu_usuario_comum(id_usuario_logado: i32) {
    let opcoes = vec![
        "Explorar Catálogo",
        "Realizar Compra",
        "Registrar Avaliação",
        "Sair",
    ];

    loop {
        let escolha = Select::new("Escolha uma opção:", opcoes.clone()).prompt();

        match escolha {
            Ok("Explorar Catálogo") => menu_explorar_catalogo(),
            Ok("Realizar Compra") => menu_compras_comum(id_usuario_logado),
            Ok("Registrar Avaliação") => menu_avaliacoes_comum(id_usuario_logado),
            Ok("Sair") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_explorar_catalogo() {
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

pub fn menu_compras_comum(id_usuario_logado: i32) {
    let opcoes = vec!["Listar Compras", "Adicionar Compra", "Voltar"];

    loop {
        let escolha = Select::new("Gerenciamento de Compras:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Suas Compras") => {
                {
                    compra::listar_compras_por_usuario(id_usuario_logado)
                };
            }
            Ok("Comprar Jogo") => {
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

                let preco_jogo = jogos
                    .iter()
                    .find(|j| j.1 == escolha_jogo)
                    .map(|j| j.2)
                    .unwrap();

                let metodo_pagamento = inquire::Text::new("Método de Pagamento:").prompt().unwrap();

                compra::adicionar_compra(id_usuario_logado, id_jogo, preco_jogo, &metodo_pagamento);
            }
            Ok("Voltar") => break,
            _ => println!("Opção inválida"),
        }
    }
}

pub fn menu_avaliacoes_comum(id_usuario_logado: i32) {
    let opcoes = vec![
        "Listar Avaliações",
        "Adicionar Avaliação",
        "Remover Avaliação",
        "Atualizar Avaliação",
        "Voltar",
    ];

    loop {
        let escolha = Select::new("Gerenciamento de Avaliações:", opcoes.clone()).prompt();

        match escolha {
            Ok("Listar Avaliações") => {
                avaliacao::listar_avaliacoes_por_usuario(id_usuario_logado);
            }

            Ok("Adicionar Avaliação") => {
                let compras = compra::listar_compras_por_usuario(id_usuario_logado);
                let jogos_nomes: Vec<String> = compras.iter().map(|c| c.1.clone()).collect();

                if jogos_nomes.is_empty() {
                    println!("Você ainda não comprou nenhum jogo para avaliar.");
                    continue;
                }

                let escolha_jogo = Select::new("Escolha um jogo para avaliar:", jogos_nomes)
                    .prompt()
                    .unwrap();

                let id_jogo = compras
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

                avaliacao::adicionar_avaliacao(id_usuario_logado, id_jogo, nota, Some(&comentario));
            }

            Ok("Remover Avaliação") => {
                let avaliacoes = avaliacao::listar_avaliacoes_por_usuario(id_usuario_logado);

                if avaliacoes.is_empty() {
                    println!("Você não tem avaliações para remover.");
                    continue;
                }

                let avaliacoes_texto: Vec<String> = avaliacoes
                    .iter()
                    .map(|(id, jogo, nota, comentario)| {
                        format!(
                            "ID: {} | Jogo: {} | Nota: {} | Comentário: {}",
                            id,
                            jogo,
                            nota,
                            comentario.as_deref().unwrap_or("Sem comentário")
                        )
                    })
                    .collect();

                let escolha_avaliacao =
                    Select::new("Escolha uma avaliação para remover:", avaliacoes_texto)
                        .prompt()
                        .unwrap();

                let id_avaliacao = avaliacoes
                    .iter()
                    .find(|(_id, jogo, _, _)| escolha_avaliacao.contains(jogo))
                    .map(|(id, _, _, _)| *id)
                    .unwrap();

                avaliacao::remover_avaliacao(id_avaliacao);
            }

            Ok("Atualizar Avaliação") => {
                let avaliacoes = avaliacao::listar_avaliacoes_por_usuario(id_usuario_logado);

                if avaliacoes.is_empty() {
                    println!("Você não tem avaliações para atualizar.");
                    continue;
                }

                let avaliacoes_texto: Vec<String> = avaliacoes
                    .iter()
                    .map(|(id, jogo, nota, comentario)| {
                        format!(
                            "ID: {} | Jogo: {} | Nota: {} | Comentário: {}",
                            id,
                            jogo,
                            nota,
                            comentario.as_deref().unwrap_or("Sem comentário")
                        )
                    })
                    .collect();

                let escolha_avaliacao =
                    Select::new("Escolha uma avaliação para atualizar:", avaliacoes_texto)
                        .prompt()
                        .unwrap();

                let id_avaliacao = avaliacoes
                    .iter()
                    .find(|(_id, jogo, _, _)| escolha_avaliacao.contains(jogo))
                    .map(|(id, _, _, _)| *id)
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
