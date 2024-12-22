use sqlx::{Pool, MySql};
use std::io::{self, Write};

pub async fn consultar_material(pool: &Pool<MySql>) -> sqlx::Result<()> {
    let mut codigo = String::new();

    // Função auxiliar para verificar se o usuário digitou "EXIT"
    fn cancelar_se_exit(input: &str) -> bool {
        input.trim().eq_ignore_ascii_case("EXIT")
    }

    println!("Digite o código do material para consulta (digite EXIT para cancelar): ");
    io::stdout().flush()?; // Força a exibição da mensagem antes de aguardar a entrada
    io::stdin().read_line(&mut codigo)?;

    if cancelar_se_exit(&codigo) {
        println!("Operação cancelada.");
        return Ok(());
    }

    let codigo = codigo.trim();

    let row = sqlx::query!(
        "SELECT codigo, descricao, quantidade FROM materiais WHERE codigo = ?",
        codigo
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(material) => {
            println!("Código: {}", material.codigo);
            println!("Descrição: {}", material.descricao);
            println!("Quantidade: {}", material.quantidade);
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
