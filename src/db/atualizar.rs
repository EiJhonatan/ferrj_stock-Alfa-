use sqlx::{Pool, MySql};
use std::io::{self, Write};

pub async fn atualiza_material(pool: &Pool<MySql>) -> sqlx::Result<()> {
    let mut codigo = String::new();
    let mut descricao = String::new();
    let mut quantidade_str = String::new();

    // Função auxiliar para verificar se o usuário digitou "EXIT"
    fn cancelar_se_exit(input: &str) -> bool {
        input.trim().eq_ignore_ascii_case("EXIT")
    }

    println!("Digite o código do material a ser atualizado (digite EXIT para cancelar): ");
    io::stdout().flush()?; // Força a exibição da mensagem antes de aguardar a entrada
    io::stdin().read_line(&mut codigo)?;

    if cancelar_se_exit(&codigo) {
        println!("Operação cancelada.");
        return Ok(());
    }

    let codigo = codigo.trim();

    // Verificando se o material existe antes de continuar
    let row = sqlx::query!(
        "SELECT codigo, descricao, quantidade FROM materiais WHERE codigo = ?",
        codigo
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(material) => {
            // Exibe as informações atuais do material
            println!("Material encontrado: ");
            println!("Código: {}", material.codigo);
            println!("Descrição: {}", material.descricao);
            println!("Quantidade: {}", material.quantidade);

            // Solicita novos dados para atualização
            println!("Digite a nova descrição do material (digite EXIT para cancelar): ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut descricao)?;

            if cancelar_se_exit(&descricao) {
                println!("Operação cancelada.");
                return Ok(());
            }

            let descricao = descricao.trim().to_string();

            println!("Digite a nova quantidade do material (digite EXIT para cancelar): ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut quantidade_str)?;

            if cancelar_se_exit(&quantidade_str) {
                println!("Operação cancelada.");
                return Ok(());
            }

           

            // Atualizando os dados no banco de dados
            sqlx::query!(
                "UPDATE materiais SET descricao = ?, quantidade = ? WHERE codigo = ?",
                descricao,
                quantidade_str,
                codigo
            )
            .execute(pool)
            .await?;

            println!("Material atualizado com sucesso!");
        }
        None => println!("Material não encontrado."),
    }

    // Aguarda o pressionamento de "Enter" antes de continuar
    println!("\nPressione Enter para voltar ao menu...");
    io::stdout().flush()?; // Garante que a mensagem seja exibida antes de aguardar a entrada
    let mut pause = String::new();
    io::stdin().read_line(&mut pause)?;

    Ok(())
}
