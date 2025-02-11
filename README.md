# 🎮 Sistema de Gerenciamento de Loja de Jogos (CLI)

## 👥 Integrantes do Grupo  
- Davi Jannotti Coelho Pinheiro  
- João Pedro Dani Laguardia 
- Lucas de Lima Bergami

---

## 📌 Sobre o Projeto  

Este projeto é um **Sistema de Gerenciamento de Loja de Jogos** desenvolvido em **Rust** com um banco de dados **MySQL**. O sistema permite que usuários possam explorar o catálogo de jogos, realizar compras, registrar avaliações e registrar troféus. Além disso, há um painel administrativo para moderação (CRUD) e gerenciamento geral.  

### **Principais Funcionalidades:**  
- 🔹 Cadastro e login de usuários (com diferenciação entre administradores e usuários comuns).  
- 🔹 Compra de jogos e listagem do histórico de compras.  
- 🔹 Avaliação de jogos adquiridos pelos usuários.  
- 🔹 Gerenciamento de conquistas e avaliações de jogos.  
- 🔹 Moderação de conteúdo e controle geral (CRUD) pelos administradores.  

---

## 🚀 Como Executar o Projeto  

### **📌 Pré-requisitos:**  
Certifique-se de ter instalado na sua máquina:  

- **Rust** (versão mais recente) → [Instalar Rust](https://www.rust-lang.org/pt-BR/tools/install)  
- **MySQL Server** → [Instalar MySQL](https://dev.mysql.com/downloads/)  

### **1️⃣ Clonar o Repositório**  
```sh
git clone https://github.com/davijannotti/gameshop-dbsystem-rs.git
cd gameshop-dbsystem-rs
```
### **2️⃣ Configurar o Banco de Dados**  

1. Acesse o MySQL e crie o banco de dados:  
    ```sql
    CREATE DATABASE LojaJogosOnline;
    ```

2. Importe o script SQL para criar as tabelas e inserir dados iniciais:  
    ```sh
    mysql -u seu_usuario -p LojaJogosOnline < database.sql
    ```

3. Configure as credenciais do banco de dados no arquivo **.env** (caso necessário).  

---

### **3️⃣ Compilar e Executar o Sistema**  
Para rodar o sistema, basta compilar e executar o projeto com os seguintes comandos:  
```sh
cargo build
cargo run
```

Se quiser rodar diretamente sem compilar separadamente:  
```sh
cargo run
```

---

## **🛠 Tecnologias Utilizadas** 

- 🦀 **Rust** (Linguagem principal)  
- 🗄 **MySQL** (Banco de Dados)  
- 🎛 **Inquire** (Biblioteca Rust para entrada interativa no CLI)  

---

📢 **Dúvidas ou sugestões?** Sinta-se à vontade para contribuir ou entrar em contato! 🚀