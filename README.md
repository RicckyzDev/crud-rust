# Gerenciamento de Clientes

CRUD para gerenciamento de clientes desenvolvido em Rust, utilizando Actix-Web, autenticação JWT e controle de permissões por role (papel de usuário). O banco de dados é PostgreSQL, executado via Docker.

---

## 🛠️ Tecnologias e Dependências

- **Rust** (2024 edition)
- **Actix-Web** – API web assíncrona
- **JWT** – Autenticação baseada em tokens
- **bcrypt** – Hash de senhas
- **Chrono**, **Time** – Manipulação de datas
- **Serde** – Serialização/Deserialização
- **SQLx** – ORM assíncrono para PostgreSQL
- **dotenv** – Carregamento de variáveis ambiente
- **Docker** – Ambiente para banco de dados PostgreSQL

### Trecho do `Cargo.toml`

```toml
[dependencies]
actix-cors = "0.7.1"
actix-web = "4.10.2"
bcrypt = "0.17.0"
chrono = "0.4.40"
dotenv = "0.15.0"
futures-util = "0.3.31"
jsonwebtoken = "9.3.1"
time = { version = "0.3", features = ["serde"] }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
sqlx = { version = "0.8.5", features = ["postgres", "runtime-async-std", "time"] }
```

---

## 🚀 Como Rodar

### 1. Subindo o Banco de Dados com Docker

Execute no terminal:

```sh
docker run --name clientes-db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=clientes_db \
  -p 5432:5432 \
  -d postgres
```

- Banco: `clientes_db`  
- Usuário: `postgres`  
- Senha: `postgres`  
- Porta: `5432`  

### 2. Configurando as Variáveis do Projeto

Crie um arquivo `.env` na raiz do projeto contendo:

```env
DATABASE_URL=postgres://postgres:postgres@localhost/clientes_db
SECRET_KEY=sua_chave_secreta
```

### 3. Instalando Dependências

```sh
cargo build
```

### 4. Rodando a Aplicação

```sh
cargo run
```

A API estará disponível em: [http://localhost:8080](http://localhost:8080)

---

## 📡 Endpoints Principais

| Método | Rota               | Descrição                          | Autenticação |
|--------|--------------------|------------------------------------|--------------|
| POST   | `/register`        | Cadastro de usuários               | ❌           |
| POST   | `/login`           | Geração de token JWT               | ❌           |
| GET    | `/clientes`        | Listagem de clientes               | ✅           |
| POST   | `/clientes`        | Cadastro de novo cliente           | ✅ (por role)|
| PUT    | `/clientes/{id}`   | Atualização de cliente             | ✅           |
| DELETE | `/clientes/{id}`   | Remoção de cliente                 | ✅ (admin)   |

> ⚠️ Todas as rotas protegidas requerem um token JWT válido no header `Authorization: Bearer <token>`

---

## 🔐 Permissões (Roles)

O sistema utiliza controle de acesso baseado em **roles** (`admin`, `user`, etc).  
Após o login, a role do usuário é embutida no token JWT, e o backend valida isso em cada rota sensível.

---

## 🐳 Parando e Removendo o Container do Banco

```sh
docker stop clientes-db
docker rm clientes-db
```

---

## ⚠️ Observações

- **Nunca** utilize a mesma `SECRET_KEY` deste exemplo em produção.
- Certifique-se de que o Docker esteja em execução antes de iniciar o backend.
- Para payloads e schemas de resposta, consulte os modelos no código-fonte (`src/models`).

---

## 📄 Licença

[MIT](https://opensource.org/licenses/MIT)

---

## 🤝 Contribuições

Contribuições são bem-vindas!  
Sinta-se livre para abrir issues ou pull requests. 🚀
