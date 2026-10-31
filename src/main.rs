use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        println!("\n=========================================");
        println!("    YouTube Downloader Pro (Rust)         ");
        println!("=========================================");
        println!("1. Baixar Vídeo / Áudio (Individual)");
        println!("2. Baixar Playlist Completa");
        println!("3. Atualizar o yt-dlp");
        println!("4. Sair");
        print!("Escolha uma opção (1-4): ");
        io::stdout().flush().unwrap();

        let mut opcao_menu = String::new();
        io::stdin().read_line(&mut opcao_menu).expect("Falha ao ler opção");

        match opcao_menu.trim() {
            "1" => iniciar_download(false),
            "2" => iniciar_download(true),
            "3" => atualizar_ytdlp(),
            "4" => {
                println!("\nA encerrar o programa. Até logo!");
                break;
            }
            _ => println!("\nOpção inválida! Tente novamente."),
        }
    }
}

fn iniciar_download(e_playlist: bool) {
    let tipo_str = if e_playlist { "da playlist" } else { "do vídeo" };
    print!("\nDigite ou cole la URL {}: ", tipo_str);
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Falha ao ler URL");
    let url = url.trim();

    if url.is_empty() {
        println!("A URL não pode estar vazia!");
        return;
    }

    println!("\nEscolha o formato desejado:");
    println!("1. MP4  (Vídeo em alta qualidade)");
    println!("2. MKV  (Vídeo com suporte a múltiplas legendas/áudios)");
    println!("3. MP3  (Apenas Áudio limpo, sem ID ou números)");
    print!("Escolha o formato (1-3): ");
    io::stdout().flush().unwrap();

    let mut formato_escolha = String::new();
    io::stdin().read_line(&mut formato_escolha).expect("Falha ao ler formato");

    let mut cmd = Command::new("yt-dlp");

    // Define o diretório de saída e força APENAS o título e a extensão (sem ID ou códigos extras)
    if let Some(pasta_downloads) = dirs::download_dir() {
        if let Some(caminho_str) = pasta_downloads.to_str() {
            let template = format!("{}/%(title)s.%(ext)s", caminho_str);
            cmd.arg("-o").arg(template);
        }
    } else {
        cmd.arg("-o").arg("%(title)s.%(ext)s");
    }

    // Configura se deve baixar a playlist inteira ou apenas o vídeo isolado
    if e_playlist {
        cmd.arg("--yes-playlist");
        cmd.arg("--no-play-number");
    } else {
        cmd.arg("--no-playlist");
    }

    // Ativa o download acelerado em 8 conexões paralelas
    cmd.arg("-N").arg("8");

    match formato_escolha.trim() {
        "2" => {
            println!("\nBaixando no formato MKV...");
            cmd.arg("-f").arg("bv*+ba/b")
               .arg("--merge-output-format").arg("mkv");
        }
        "3" => {
            println!("\nExtraindo áudio MP3...");
            cmd.arg("-x")
               .arg("--audio-format").arg("mp3")
               .arg("--audio-quality").arg("0")
               .arg("--embed-thumbnail")
               .arg("--parse-metadata").arg("nothing:%(meta_track)s");
        }
        _ => {
            println!("\nBaixando no formato MP4...");
            cmd.arg("-f").arg("bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b")
               .arg("--merge-output-format").arg("mp4");
        }
    }

    cmd.arg(url);

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("\nDownload concluído! O arquivo foi salvo na sua pasta Downloads.");
        }
        _ => {
            println!("\nOcorreu um erro durante o download.");
            println!("Certifique-se de que o ffmpeg está instalado no seu Linux.");
        }
    }
}

fn atualizar_ytdlp() {
    println!("\nA verificar atualizações do yt-dlp...");
    let status = Command::new("yt-dlp").arg("-U").status();

    match status {
        Ok(s) if s.success() => println!("\nyt-dlp verificado/atualizado com sucesso!"),
        _ => println!("\nFalha ao atualizar o yt-dlp."),
    }
}
