use sqlx::{Pool, MySql};
use std::io::{self, Write};

pub async fn deletar_material(pool: &Pool<MySql>) -> sqlx::Result<()> {
    let mut codigo = String::new();

    // Função auxiliar para verificar se o usuário digitou "EXIT"
    fn cancelar_se_exit(input: &str) -> bool {
        input.trim().eq_ignore_ascii_case("EXIT")
    }

    println!("Digite o código do material para deletar (digite EXIT para cancelar): ");
    io::stdout().flush()?;  // Força a exibição da mensagem antes de aguardar a entrada
    io::stdin().read_line(&mut codigo)?;

    if cancelar_se_exit(&codigo) {
        println!("Operação cancelada.");
        return Ok(());
    }

    let codigo = codigo.trim();

    sqlx::query!("DELETE FROM materiais WHERE codigo = ?", codigo)
        .execute(pool)
        .await?;

    println!("Material deletado com sucesso!");
    Ok(())
}

