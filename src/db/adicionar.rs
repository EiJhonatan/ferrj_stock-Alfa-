use sqlx::{Pool, MySql};
use std::io::{self, Write};

pub async fn adicionar_material(pool: &Pool<MySql>) -> sqlx::Result<()> {
    let mut codigo = String::new();
    let mut descricao = String::new();
    let mut quantidade_str = String::new();

    // Função auxiliar para verificar se o usuário digitou "EXIT"
    fn cancelar_se_exit(input: &str) -> bool {
        input.trim().eq_ignore_ascii_case("EXIT")
    }

    println!("Digite o código do material (digite EXIT para cancelar): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut codigo)?;

    if cancelar_se_exit(&codigo) {
        println!("Operação cancelada.");
        return Ok(());
    }

    let codigo = codigo.trim().to_string();

    println!("Digite a descrição do material (digite EXIT para cancelar): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut descricao)?;

    if cancelar_se_exit(&descricao) {
        println!("Operação cancelada.");
        return Ok(());
    }

    let descricao = descricao.trim().to_string();

    println!("Digite a quantidade do material (digite EXIT para cancelar): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut quantidade_str)?;

    if cancelar_se_exit(&quantidade_str) {
        println!("Operação cancelada.");
        return Ok(());
    }

    let quantidade = quantidade_str.trim().parse::<f64>().unwrap_or(0.0);

    sqlx::query!(
        "INSERT INTO materiais (codigo, descricao, quantidade) VALUES (?, ?, ?)",
        codigo,
        descricao,
        quantidade
    )
    .execute(pool)
    .await?;

    println!("Material adicionado com sucesso!");
    Ok(())
}
