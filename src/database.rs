use mysql::{prelude::Queryable, Pool, PooledConn};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UserRole {
    Admin,
    Comum,
}

pub fn conectar_mysql() -> PooledConn {
    let url = "mysql://root:root@localhost:3306/LojaJogosOnline";
    let pool = Pool::new(url).expect("Erro ao conectar ao banco");
    pool.get_conn().expect("Erro ao obter conexão")
}

pub fn login() -> Option<(i32, String, UserRole)> {
    let email = inquire::Text::new("Digite seu e-mail:").prompt().unwrap();
    let senha = inquire::Text::new("Digite sua senha:").prompt().unwrap();

    let mut conn = conectar_mysql();
    let query = "SELECT id_usuario, nome, email, senha FROM Usuario WHERE email = ? AND senha = ?";

    if let Some(user) = conn
        .exec_first(query, (email, senha))
        .expect("Erro ao autenticar")
    {
        let (id, nome, _email, _senha): (i32, String, String, String) = user;
        let role = if is_admin(id) {
            UserRole::Admin
        } else {
            UserRole::Comum
        };
        Some((id, nome, role))
    } else {
        println!("Login ou senha incorretos.");
        None
    }
}

fn is_admin(id_usuario: i32) -> bool {
    let mut conn = conectar_mysql();
    let query = "SELECT 1 FROM UsuarioAdmin WHERE id_usuario = ?";

    let result: Option<i32> = conn
        .exec_first(query, (id_usuario,))
        .expect("Erro ao verificar administrador");

    result.is_some()
}
