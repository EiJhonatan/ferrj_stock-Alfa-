# Software de Gerenciamento de Estoque

Este é um software de gerenciamento de estoque desenvolvido para registrar, atualizar, consultar e remover materiais de um inventário de forma simples e eficiente. Ele permite que os usuários gerenciem os itens do estoque através de um sistema fácil de usar, proporcionando controle total sobre a quantidade e peso dos materiais.

## Funcionalidades

- **Registrar materiais**: Adicione novos itens ao estoque, informando suas quantidades ou peso.
- **Atualizar estoque**: Modifique informações de materiais existentes, ajustando quantidades ou outras propriedades.
- **Consultar estoque**: Realize buscas para verificar a quantidade ou peso de itens específicos no estoque.
- **Remover materiais**: Exclua itens do estoque quando não forem mais necessários.
- **Integração com código de barras**: Possibilidade de utilizar códigos de barras para facilitar o processo de registro e consulta.

## Tecnologias utilizadas

- **Rust**: Linguagem de programação utilizada para desenvolvimento do sistema.
- **MySQL**: Banco de dados para armazenar informações sobre os materiais no estoque.
- **HeidiSQL**: Ferramenta de gerenciamento do banco de dados MySQL.

## Como usar

1. Clone este repositório:
   ```bash
   git clone https://github.com/EiJhonatan/ferrj_stock-Alfa-.git

## Instalação e Configuração

### Instale as dependências necessárias.

### Configure o banco de dados MySQL:

1. Crie um banco de dados no MySQL.
2. No arquivo `.env` na raiz do projeto, adicione as variáveis de configuração do banco de dados.

### Configurando o arquivo `.env`

Para configurar o arquivo `.env` corretamente em seu projeto, siga os passos abaixo:

#### Criando o arquivo `.env`

No diretório raiz do seu projeto (onde o arquivo `Cargo.toml` está localizado), crie um arquivo chamado `.env`. Esse arquivo armazenará suas variáveis de ambiente, como a URL de conexão com o banco de dados. (`O Mesmo ja ta criado basta Configura.`)

#### Configurando o `.env`

No arquivo `.env`, Configure a seguinte linha `DATABASE_URL`, que é usada para conectar ao banco de dados MySQL:

```
DATABASE_URL=mysql://usuario:senha@localhost:3306/nomedobanco
```

### Aqui está o que cada parte representa:

- `usuario`: o nome de usuário para acessar o banco de dados.
- `senha`: a senha do usuário.
- `localhost`: o endereço do servidor do banco de dados (se o banco estiver rodando na mesma máquina, use `localhost` ou `127.0.0.1`).
- `3306`: a porta padrão do MySQL.
- `nomedobanco`: o nome do banco de dados com o qual você deseja se conectar.

### Exemplo do arquivo `.env`

Aqui está um exemplo de como o arquivo `.env` pode parecer:
```
DATABASE_URL=mysql://root:senha@localhost:3306/estoque
```

Este exemplo configura uma conexão com o banco de dados chamado `estoque`, usando o usuário `root` e a senha `senha` no banco MySQL rodando na máquina local (`localhost`).

### Execute o software e comece a gerenciar seu estoque!
