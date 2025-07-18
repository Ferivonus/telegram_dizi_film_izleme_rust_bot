use dotenv;
use log::info;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use teloxide::{prelude::*, utils::command::BotCommands};

const MAX_MESSAGE_LENGTH: usize = 4000;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    info!("Starting command bot...");

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "Merhaba! Ben Film ve Dizi Botu. İşte kullanabileceğin komutlar:"
)]
enum Command {
    #[command(description = "Tüm komutların listesini ve açıklamalarını gösterir.")]
    Yardim,

    #[command(description = "İzlenmemiş filmler listesinden rastgele bir film önerir.")]
    FilmOner,

    #[command(description = "İzlenmemiş diziler listesinden rastgele bir dizi önerir.")]
    DiziOner,

    #[command(
        description = "Önerilen veya izlediğin bir filmi 'izlenenler' listene ekler. Kullanım: /izlenen_film_ekle <Film Adı>"
    )]
    IzlenenFilmEkle(String),

    #[command(
        description = "Önerilen veya izlediğin bir diziyi 'izlenenler' listene ekler. Kullanım: /izlenen_dizi_ekle <Dizi Adı>"
    )]
    IzlenenDiziEkle(String),

    #[command(
        description = "Yeni bir filmi ana filmler listesine ekler. Kullanım: /film_ekle <Film Adı>"
    )]
    FilmEkle(String),

    #[command(
        description = "Yeni bir diziyi ana diziler listesine ekler. Kullanım: /dizi_ekle <Dizi Adı>"
    )]
    DiziEkle(String),

    #[command(description = "İzlediğin tüm filmleri listeler.")]
    IzlenenFilmler,

    #[command(description = "İzlediğin tüm dizileri listeler.")]
    IzlenenDiziler,

    #[command(description = "Ana filmler listesindeki tüm filmleri gösterir.")]
    TumFilmler,

    #[command(description = "Ana diziler listesindeki tüm dizileri gösterir.")]
    TumDiziler,

    #[command(description = "Henüz izlemediğin filmleri listeler.")]
    IzlenmemisFilmler,

    #[command(description = "Henüz izlemediğin dizileri listeler.")]
    IzlenmemisDiziler,

    #[command(description = "Bota merhaba der ve sana özel bir mesaj gönderir.")]
    Merhaba,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Yardim => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }

        Command::FilmOner => match get_random_unwatched_film() {
            Some(film) => {
                bot.send_message(
                    msg.chat.id,
                    format!("🎬 Film Önerisi: {}. İzlediğinizde `/izlenen_film_ekle {}` komutunu kullanın.", film, film)
                ).await?;
            }
            None => {
                bot.send_message(
                    msg.chat.id,
                    "✅ Tüm filmler önerildi! `izlenen_filmler.txt` dosyasını silerek listeyi sıfırlayabilirsin.",
                )
                .await?;
            }
        },

        Command::DiziOner => match get_random_unwatched_series() {
            Some(series) => {
                bot.send_message(
                    msg.chat.id,
                    format!("📺 Dizi Önerisi: {}. İzlediğinizde `/izlenen_dizi_ekle {}` komutunu kullanın.", series, series)
                ).await?;
            }
            None => {
                bot.send_message(
                    msg.chat.id,
                    "✅ Tüm diziler önerildi! `izlenen_diziler.txt` dosyasını silerek listeyi sıfırlayabilirsin.",
                )
                .await?;
            }
        },

        Command::IzlenenFilmEkle(film_name_raw) => {
            let film_name_input = film_name_raw.trim().to_lowercase();
            info!("Attempting to mark film as watched: '{}'", film_name_input);
            let all_films_in_master_list = load_films("filmler.txt");

            let mut exact_match: Option<String> = None;
            let mut potential_matches: Vec<String> = Vec::new();

            for f in all_films_in_master_list.iter() {
                let f_lower = f.to_lowercase();
                if f_lower == film_name_input {
                    exact_match = Some(f.clone());
                    break;
                } else if f_lower.contains(&film_name_input) {
                    potential_matches.push(f.clone());
                }
            }

            if let Some(film_to_mark) = exact_match {
                mark_film_as_watched(&film_to_mark);
                bot.send_message(
                    msg.chat.id,
                    format!("✅ '{}' filmi izlenenlere eklendi.", film_to_mark),
                )
                .await?;
            } else {
                if !potential_matches.is_empty() {
                    let suggestions = potential_matches.join(", ");
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Hata: '{}' adında bir film bulunamadı. Bunu mu demek istediniz: {}?",
                            film_name_input, suggestions
                        ),
                    )
                    .await?;
                } else {
                    bot.send_message(
                        msg.chat.id,
                        format!("Hata: '{}' adında bir film bulunamadı. Lütfen `filmler.txt` dosyasındaki tam adı (yıl bilgisi dahil) kullanın.", film_name_input)
                    ).await?;
                }
            }
        }

        Command::IzlenenDiziEkle(series_name_raw) => {
            let series_name_input = series_name_raw.trim().to_lowercase();
            info!(
                "Attempting to mark series as watched: '{}'",
                series_name_input
            );
            let all_series_in_master_list = load_series("diziler.txt");

            let mut exact_match: Option<String> = None;
            let mut potential_matches: Vec<String> = Vec::new();

            for s in all_series_in_master_list.iter() {
                let s_lower = s.to_lowercase();
                if s_lower == series_name_input {
                    exact_match = Some(s.clone());
                    break;
                } else if s_lower.contains(&series_name_input) {
                    potential_matches.push(s.clone());
                }
            }

            if let Some(series_to_mark) = exact_match {
                mark_series_as_watched(&series_to_mark);
                bot.send_message(
                    msg.chat.id,
                    format!("✅ '{}' dizisi izlenenlere eklendi.", series_to_mark),
                )
                .await?;
            } else {
                if !potential_matches.is_empty() {
                    let suggestions = potential_matches.join(", ");
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Hata: '{}' adında bir dizi bulunamadı. Bunu mu demek istediniz: {}?",
                            series_name_input, suggestions
                        ),
                    )
                    .await?;
                } else {
                    bot.send_message(
                        msg.chat.id,
                        format!("Hata: '{}' adında bir dizi bulunamadı. Lütfen `diziler.txt` dosyasındaki tam adı (sezon bilgisi dahil) kullanın.", series_name_input)
                    ).await?;
                }
            }
        }

        Command::FilmEkle(film_name_raw) => {
            let film_name = film_name_raw.trim().to_string();
            info!("Attempting to add film to master list: '{}'", film_name);
            match add_film_to_file(&film_name) {
                Ok(added) => {
                    if added {
                        bot.send_message(
                            msg.chat.id,
                            format!("✅ '{}' filmi `filmler.txt` dosyasına eklendi.", film_name),
                        )
                        .await?;
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "ℹ️ Film '{}' zaten `filmler.txt` dosyasında mevcut.",
                                film_name
                            ),
                        )
                        .await?;
                    }
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Film eklenirken bir hata oluştu: {}", e),
                    )
                    .await?;
                }
            }
        }

        Command::DiziEkle(series_name_raw) => {
            let series_name = series_name_raw.trim().to_string();
            info!("Attempting to add series to master list: '{}'", series_name);
            match add_series_to_file(&series_name) {
                Ok(added) => {
                    if added {
                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "✅ '{}' dizisi `diziler.txt` dosyasına eklendi.",
                                series_name
                            ),
                        )
                        .await?;
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "ℹ️ Dizi '{}' zaten `diziler.txt` dosyasında mevcut.",
                                series_name
                            ),
                        )
                        .await?;
                    }
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Dizi eklenirken bir hata oluştu: {}", e),
                    )
                    .await?;
                }
            }
        }

        Command::TumFilmler => {
            let all_films = load_films("filmler.txt");
            if all_films.is_empty() {
                bot.send_message(msg.chat.id, "Henüz `filmler.txt` dosyasında kayıtlı bir film yok. `/film_ekle` komutunu kullanarak ekleyebilirsin.").await?;
            } else {
                let mut response_text = "🎬 Tüm Filmler:\n".to_string();
                let mut sorted_films: Vec<&String> = all_films.iter().collect();
                sorted_films.sort();
                for film in sorted_films.iter() {
                    response_text.push_str(&format!("- {}\n", film));
                }
                send_long_message(bot, msg.chat.id, response_text).await?;
            }
        }

        Command::TumDiziler => {
            let all_series = load_series("diziler.txt");
            if all_series.is_empty() {
                bot.send_message(msg.chat.id, "Henüz `diziler.txt` dosyasında kayıtlı bir dizi yok. `/dizi_ekle` komutunu kullanarak ekleyebilirsin.").await?;
            } else {
                let mut response_text = "📺 Tüm Diziler:\n".to_string();
                let mut sorted_series: Vec<&String> = all_series.iter().collect();
                sorted_series.sort();
                for series_name in sorted_series.iter() {
                    response_text.push_str(&format!("- {}\n", series_name));
                }
                send_long_message(bot, msg.chat.id, response_text).await?;
            }
        }

        Command::IzlenmemisFilmler => {
            let all_films = load_films("filmler.txt");
            let watched_films = load_watched_films("izlenen_filmler.txt");
            let mut unwatched_films: Vec<&String> = all_films
                .iter()
                .filter(|f| !watched_films.contains(f.as_str()))
                .collect();
            unwatched_films.sort();

            if unwatched_films.is_empty() {
                bot.send_message(msg.chat.id, "🎉 Harika! Tüm filmleri izlemişsin veya listen boş. Yeni filmler eklemek için `/film_ekle` komutunu kullanabilirsin.").await?;
            } else {
                let mut response_text = "🎬 İzlenmemiş Filmler:\n".to_string();
                for film in unwatched_films.iter() {
                    response_text.push_str(&format!("- {}\n", film));
                }
                send_long_message(bot, msg.chat.id, response_text).await?;
            }
        }

        Command::IzlenmemisDiziler => {
            let all_series = load_series("diziler.txt");
            let watched_series = load_watched_series("izlenen_diziler.txt");
            let mut unwatched_series: Vec<&String> = all_series
                .iter()
                .filter(|s| !watched_series.contains(s.as_str()))
                .collect();
            unwatched_series.sort();

            if unwatched_series.is_empty() {
                bot.send_message(msg.chat.id, "🎉 Harika! Tüm dizileri izlemişsin veya listen boş. Yeni diziler eklemek için `/dizi_ekle` komutunu kullanabilirsin.").await?;
            } else {
                let mut response_text = "📺 İzlenmemiş Diziler:\n".to_string();
                for series_name in unwatched_series.iter() {
                    response_text.push_str(&format!("- {}\n", series_name));
                }
                send_long_message(bot, msg.chat.id, response_text).await?;
            }
        }

        Command::IzlenenFilmler => {
            let watched_films = load_watched_films("izlenen_filmler.txt");
            if watched_films.is_empty() {
                bot.send_message(msg.chat.id, "Henüz izlenmiş bir film yok. `/film_oner` komutunu kullanarak ilk filmini öner!").await?;
            } else {
                let mut response_text = "🎬 İzlediğin Filmler:\n".to_string();
                let mut sorted_films: Vec<&String> = watched_films.iter().collect();
                sorted_films.sort();
                for film in sorted_films.iter() {
                    response_text.push_str(&format!("- {}\n", film));
                }
                send_long_message(bot, msg.chat.id, response_text).await?;
            }
        }

        Command::IzlenenDiziler => {
            let watched_series = load_watched_series("izlenen_diziler.txt");
            if watched_series.is_empty() {
                bot.send_message(msg.chat.id, "Henüz izlenmiş bir dizi yok. `/dizi_oner` komutunu kullanarak ilk dizini öner!").await?;
            } else {
                let mut response_text = "📺 İzlediğin Diziler:\n".to_string();
                let mut sorted_series: Vec<&String> = watched_series.iter().collect();
                sorted_series.sort();
                for series_name in sorted_series.iter() {
                    response_text.push_str(&format!("- {}\n", series_name));
                }
                send_long_message(bot, msg.chat.id, response_text).await?;
            }
        }

        Command::Merhaba => {
            let name = msg
                .from
                .map(|u| u.first_name.clone())
                .unwrap_or("orası".into());
            bot.send_message(msg.chat.id, format!("👋 Merhaba, {name}!"))
                .await?;
        }
    };

    Ok(())
}

async fn send_long_message(bot: Bot, chat_id: ChatId, text: String) -> ResponseResult<()> {
    let lines: Vec<&str> = text.lines().collect();
    let mut current_chunk = String::new();

    for line in lines {
        if current_chunk.len() + line.len() + 1 > MAX_MESSAGE_LENGTH {
            if !current_chunk.is_empty() {
                bot.send_message(chat_id, current_chunk.clone()).await?;
                current_chunk.clear();
            }
        }
        current_chunk.push_str(line);
        current_chunk.push('\n');
    }

    if !current_chunk.is_empty() {
        bot.send_message(chat_id, current_chunk).await?;
    }
    Ok(())
}

fn load_films(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| {
            info!(
                "'{}' dosyası bulunamadı veya okunamadı. Boş liste döndürüldü.",
                path
            );
            String::new()
        })
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

fn get_random_unwatched_film() -> Option<String> {
    let all_films = load_films("filmler.txt");
    let watched = load_watched_films("izlenen_filmler.txt");
    let unwatched: Vec<_> = all_films
        .iter()
        .filter(|f| !watched.contains(f.as_str()))
        .cloned()
        .collect();

    let mut rng = rand::thread_rng();
    unwatched.choose(&mut rng).cloned()
}

fn load_watched_films(path: &str) -> HashSet<String> {
    match fs::read_to_string(path) {
        Ok(contents) => contents.lines().map(str::to_string).collect(),
        Err(_) => HashSet::new(),
    }
}

fn mark_film_as_watched(film: &str) {
    let mut watched_films = load_watched_films("izlenen_filmler.txt");
    if watched_films.insert(film.to_string()) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("izlenen_filmler.txt")
            .expect("❌ izlenen_filmler.txt dosyasına yazılamıyor!");

        let mut sorted_films: Vec<&String> = watched_films.iter().collect();
        sorted_films.sort();
        for f in sorted_films {
            writeln!(file, "{}", f).expect("❌ Filme yazma başarısız!");
        }
        info!("Successfully marked film '{}' as watched.", film);
    } else {
        info!("Film '{}' zaten izlenenler listesinde.", film);
    }
}

fn add_film_to_file(film: &str) -> io::Result<bool> {
    let mut all_films = load_films("filmler.txt")
        .into_iter()
        .collect::<HashSet<String>>();
    if all_films.insert(film.to_string()) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("filmler.txt")?;

        let mut sorted_films: Vec<&String> = all_films.iter().collect();
        sorted_films.sort();
        for f in sorted_films {
            writeln!(file, "{}", f)?;
        }
        info!("Film '{}' filmler.txt dosyasına eklendi.", film);
        Ok(true)
    } else {
        info!("Film '{}' zaten filmler.txt dosyasında mevcut.", film);
        Ok(false)
    }
}

fn load_series(path: &str) -> HashSet<String> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| {
            info!(
                "'{}' dosyası bulunamadı veya okunamadı. Boş liste döndürüldü.",
                path
            );
            String::new()
        })
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

fn load_watched_series(path: &str) -> HashSet<String> {
    match fs::read_to_string(path) {
        Ok(contents) => contents.lines().map(str::to_string).collect(),
        Err(_) => HashSet::new(),
    }
}

fn get_random_unwatched_series() -> Option<String> {
    let all_series = load_series("diziler.txt");
    let watched = load_watched_series("izlenen_diziler.txt");
    let unwatched: Vec<_> = all_series
        .iter()
        .filter(|s| !watched.contains(s.as_str()))
        .cloned()
        .collect();

    let mut rng = rand::thread_rng();
    unwatched.choose(&mut rng).cloned()
}

fn mark_series_as_watched(series: &str) {
    let mut watched_series = load_watched_series("izlenen_diziler.txt");
    if watched_series.insert(series.to_string()) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("izlenen_diziler.txt")
            .expect("❌ izlenen_diziler.txt dosyasına yazılamıyor!");

        let mut sorted_series: Vec<&String> = watched_series.iter().collect();
        sorted_series.sort();
        for s in sorted_series {
            writeln!(file, "{}", s).expect("❌ Diziye yazma başarısız!");
        }
        info!("Successfully marked series '{}' as watched.", series);
    } else {
        info!("Dizi '{}' zaten izlenenler listesinde.", series);
    }
}

fn add_series_to_file(series: &str) -> io::Result<bool> {
    let mut all_series = load_series("diziler.txt");
    if all_series.insert(series.to_string()) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("diziler.txt")?;

        let mut sorted_series: Vec<&String> = all_series.iter().collect();
        sorted_series.sort();
        for s in sorted_series {
            writeln!(file, "{}", s)?;
        }
        info!("Dizi '{}' diziler.txt dosyasına eklendi.", series);
        Ok(true)
    } else {
        info!("Dizi '{}' zaten diziler.txt dosyasında mevcut.", series);
        Ok(false)
    }
}
