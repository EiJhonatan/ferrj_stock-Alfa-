mod db;
use crate::db::{adicionar, consultar, atualizar, deletar};
use crossterm::terminal;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
};

use sqlx::{mysql::MySqlPool, MySql};
use std::{env, io::{stdout}};
use std::time::{Duration, Instant};
use dotenvy::dotenv; // Usando dotenvy agora

#[tokio::main]
async fn main() -> crossterm::Result<()> {
    // Carregar as variáveis de ambiente
    dotenv().ok();
    
    // Conectar ao banco de dados usando a URL no arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não está definida no .env");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    enable_raw_mode()?;  // Habilita o modo raw para capturar entradas do teclado
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;  // Esconde o cursor

    let options = vec![
        "Adiciona um Material",
        "Consulta Material",
        "Atualiza Material",
        "Deleta um Material",
        "Exit",
    ];
    let mut selected_index = 0;

    let mut last_event_time = Instant::now();

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        println!("===========================");
        println!("        MENU PRINCIPAL     ");
        println!("===========================");

        for (i, option) in options.iter().enumerate() {
            if i == selected_index {
                // Destaca a opção selecionada
                println!("\x1b[1;32m > {} <\x1b[0m ", option); // Cor verde e texto destacado
            } else {
                println!("    {}", option);
            }
        }

        println!("===========================");
        println!(" Use as setas ↑/↓ para navegar e Enter para selecionar.");
        println!("===========================");

        // Aguarda um evento de teclado
        if let Event::Key(key_event) = event::read()? {
            let now = Instant::now();
            if now.duration_since(last_event_time) < Duration::from_millis(150) {
                // Ignora eventos se estiverem muito próximos
                continue;
            }
            last_event_time = now;

            match key_event.code {
                KeyCode::Up => {
                    // Move a seleção para cima
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    // Move a seleção para baixo
                    if selected_index < options.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    if options[selected_index] == "Adiciona um Material" {
                        disable_raw_mode()?;
                        adicionar::adicionar_material(&pool).await.unwrap();
                        enable_raw_mode()?;
                    }
                    if options[selected_index] == "Consulta Material" {
                        disable_raw_mode()?;
                        consultar::consultar_material(&pool).await.unwrap();
                        enable_raw_mode()?;
                    }
                    if options[selected_index] == "Atualiza Material" {
                        disable_raw_mode()?;
                        atualizar::atualiza_material(&pool).await.unwrap();
                        enable_raw_mode()?;
                    }
                    if options[selected_index] == "Deleta um Material" {
                        disable_raw_mode()?;
                        deletar::deletar_material(&pool).await.unwrap();
                        enable_raw_mode()?;
                    }

                    // Sai do loop quando "Exit" é selecionado
                    if options[selected_index] == "Exit" {
                        break;
                    }

                    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

                    event::read()?; // Aguarda qualquer tecla antes de retornar ao menu
                }
                _ => {}
            }
        }
    }
    Ok(())
}
