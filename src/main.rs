use dotenv;
use log::info;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use teloxide::{prelude::*, utils::command::BotCommands};

// Filmler filmler.txt dosyasından yüklenir.
static FILMS: Lazy<Vec<String>> = Lazy::new(|| load_films("filmler.txt"));
// Diziler diziler.txt dosyasından yüklenir, artık sadece isimler tutulur.
static SERIES: Lazy<HashSet<String>> = Lazy::new(|| load_series("diziler.txt"));

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
    rename_rule = "snake_case", // snake_case olarak değiştirildi
    description = "Bu komutlar desteklenmektedir:" // Açıklama Türkçeye çevrildi
)]
enum Command {
    #[command(description = "Bu metni gösterir.")] // Açıklama Türkçeye çevrildi
    Help,

    #[command(description = "filmler.txt dosyasından rastgele bir film önerir.")]
    // Açıklama Türkçeye çevrildi
    RecommendFilm,

    #[command(description = "diziler.txt dosyasından rastgele bir dizi önerir.")]
    // Açıklama Türkçeye çevrildi
    RecommendDizi,

    #[command(
        description = "İzlediğiniz filmler listesine bir film ekler. Kullanım: /izlenen_film_ekle <Film Adı>"
    )] // Komut adı ve açıklama Türkçeye çevrildi
    IzlenenFilmEkle(String),

    #[command(
        description = "İzlediğiniz diziler listesine bir dizi ekler. Kullanım: /izlenen_dizi_ekle <Dizi Adı>"
    )] // Komut adı ve açıklama Türkçeye çevrildi
    IzlenenDiziEkle(String),

    #[command(description = "İzlediğiniz filmleri gösterir.")] // Açıklama Türkçeye çevrildi
    WatchedFilms,

    #[command(description = "İzlediğiniz dizileri gösterir.")] // Açıklama Türkçeye çevrildi
    WatchedSeries,

    #[command(description = "Bota merhaba der.")] // Açıklama Türkçeye çevrildi
    Hello,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }

        Command::RecommendFilm => {
            match get_random_unwatched_film() {
                Some(film) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("🎬 Film Önerisi: {}. İzlediğinizde `/izlenen_film_ekle {}` komutunu kullanın.", film, film) // Komut adı güncellendi
                    ).await?;
                }
                None => {
                    bot.send_message(
                        msg.chat.id,
                        "✅ Tüm filmler önerildi! `izlenen_filmler.txt` dosyasını silerek listeyi sıfırlayabilirsin.",
                    )
                    .await?;
                }
            }
        }

        Command::RecommendDizi => {
            match get_random_unwatched_series() {
                Some(series) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("📺 Dizi Önerisi: {}. İzlediğinizde `/izlenen_dizi_ekle {}` komutunu kullanın.", series, series) // Komut adı güncellendi
                    ).await?;
                }
                None => {
                    bot.send_message(
                        msg.chat.id,
                        "✅ Tüm diziler önerildi! `izlenen_diziler.txt` dosyasını silerek listeyi sıfırlayabilirsin.",
                    )
                    .await?;
                }
            }
        }

        Command::IzlenenFilmEkle(film_name_raw) => {
            let film_name = film_name_raw.trim().to_string();
            info!("Attempting to add watched film: '{}'", film_name);
            // FILMS listesindeki filmlerin de yıl bilgisi içerebileceğini varsayarak kontrolü güncelledik
            if FILMS.iter().any(|f| {
                f.starts_with(&film_name)
                    && (f.len() == film_name.len() || f[film_name.len()..].trim().starts_with('('))
            }) {
                mark_film_as_watched(&film_name);
                bot.send_message(
                    msg.chat.id,
                    format!("✅ '{}' filmi izlenenlere eklendi.", film_name),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("Hata: '{}' adında bir film bulunamadı. Lütfen `filmler.txt` dosyasındaki tam adı (yıl bilgisi dahil) kullanın.", film_name)
                ).await?;
            }
        }

        Command::IzlenenDiziEkle(series_name_raw) => {
            let series_name = series_name_raw.trim().to_string();
            info!("Attempting to add watched series: '{}'", series_name);
            // SERIES setindeki dizilerin de sezon bilgisi içerebileceğini varsayarak kontrolü güncelledik
            if SERIES.iter().any(|s| {
                s.starts_with(&series_name)
                    && (s.len() == series_name.len()
                        || s[series_name.len()..].trim().starts_with('('))
            }) {
                mark_series_as_watched(&series_name);
                bot.send_message(
                    msg.chat.id,
                    format!("✅ '{}' dizisi izlenenlere eklendi.", series_name),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("Hata: '{}' adında bir dizi bulunamadı. Lütfen `diziler.txt` dosyasındaki tam adı (sezon bilgisi dahil) kullanın.", series_name)
                ).await?;
            }
        }

        Command::WatchedFilms => {
            let watched_films = load_watched_films("izlenen_filmler.txt");
            if watched_films.is_empty() {
                bot.send_message(msg.chat.id, "Henüz izlenmiş bir film yok. `/recommend_film` komutunu kullanarak ilk filmini öner!").await?;
            } else {
                let mut response_text = "🎬 İzlediğin Filmler:\n".to_string();
                let mut sorted_films: Vec<&String> = watched_films.iter().collect();
                sorted_films.sort(); // Filmleri alfabetik olarak sırala
                for film in sorted_films.iter() {
                    response_text.push_str(&format!("- {}\n", film));
                }
                bot.send_message(msg.chat.id, response_text).await?;
            }
        }

        Command::WatchedSeries => {
            let watched_series = load_watched_series("izlenen_diziler.txt");
            if watched_series.is_empty() {
                bot.send_message(msg.chat.id, "Henüz izlenmiş bir dizi yok. `/recommend_dizi` komutunu kullanarak ilk dizini öner!").await?;
            } else {
                let mut response_text = "📺 İzlediğin Diziler:\n".to_string();
                let mut sorted_series: Vec<&String> = watched_series.iter().collect();
                sorted_series.sort(); // Dizileri alfabetik olarak sırala
                for series_name in sorted_series.iter() {
                    response_text.push_str(&format!("- {}\n", series_name));
                }
                bot.send_message(msg.chat.id, response_text).await?;
            }
        }

        Command::Hello => {
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

// Filmleri doğrudan satır olarak yükler (yıl bilgisi dahil).
fn load_films(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("❌ filmler.txt bulunamadı veya okunamadı!")
        .lines()
        .map(|s| s.trim().to_string()) // Her satırı doğrudan String olarak al
        .collect()
}

fn get_random_unwatched_film() -> Option<String> {
    let watched = load_watched_films("izlenen_filmler.txt");
    let unwatched: Vec<_> = FILMS
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
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("izlenen_filmler.txt")
        .expect("❌ izlenen_filmler.txt dosyasına yazılamıyor!");
    writeln!(file, "{}", film).expect("❌ Filme yazma başarısız!");
}

// Dizileri doğrudan satır olarak yükler (sezon bilgisi dahil).
fn load_series(path: &str) -> HashSet<String> {
    let contents = fs::read_to_string(path).expect("❌ diziler.txt bulunamadı veya okunamadı!");
    let mut loaded_series = HashSet::new();
    for line in contents.lines() {
        let clean_line = line.trim().to_string(); // Her satırı doğrudan String olarak al
        info!("Loaded series name: '{}'", clean_line);
        loaded_series.insert(clean_line);
    }
    loaded_series
}

// İzlenen dizileri sadece isimleriyle yükler.
fn load_watched_series(path: &str) -> HashSet<String> {
    match fs::read_to_string(path) {
        Ok(contents) => contents.lines().map(str::to_string).collect(),
        Err(_) => HashSet::new(),
    }
}

// Rastgele izlenmemiş bir dizi önerir.
fn get_random_unwatched_series() -> Option<String> {
    let watched = load_watched_series("izlenen_diziler.txt");
    let unwatched: Vec<_> = SERIES
        .iter()
        .filter(|s| !watched.contains(s.as_str()))
        .cloned()
        .collect();

    let mut rng = rand::thread_rng();
    unwatched.choose(&mut rng).cloned()
}

// Bir diziyi izlenenler listesine ekler.
fn mark_series_as_watched(series: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("izlenen_diziler.txt")
        .expect("❌ izlenen_diziler.txt dosyasına yazılamıyor!");
    writeln!(file, "{}", series).expect("❌ Diziye yazma başarısız!");
    info!("Successfully marked series '{}' as watched.", series);
}
