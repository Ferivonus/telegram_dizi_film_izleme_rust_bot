# Film ve Dizi Botu

Bu Telegram botu, film ve dizi önerileri almanızı, izlediğiniz içerikleri takip etmenizi ve kendi film/dizi listelerinizi yönetmenizi sağlar.

## Bot Nedir?

Bu Rust tabanlı Telegram botu, film ve dizi meraklıları için tasarlanmıştır. Temel özellikleri şunlardır:

- **Rastgele Öneriler:** İzlenmemiş filmler ve diziler arasından rastgele öneriler alabilirsiniz.

- **İzleme Takibi:** İzlediğiniz filmleri ve dizileri kolayca işaretleyebilir, böylece hangi içerikleri bitirdiğinizi takip edebilirsiniz.

- **Liste Yönetimi:** Kendi film ve dizi listelerinize yeni içerikler ekleyebilir, tüm listelerinizi (izlenenler, izlenmeyenler, ana listeler) görüntüleyebilirsiniz.

---

## Kurulum ve Çalıştırma

Botu çalıştırmak için bazı ön gereksinimleri karşılamanız ve basit kurulum adımlarını takip etmeniz gerekmektedir.

### Ön Gereksinimler

- **Rust ve Cargo:** Sisteminizde Rust programlama dili ve paket yöneticisi Cargo kurulu olmalıdır. Kurulum için [rustup.rs](https://rustup.rs/) adresini ziyaret edebilirsiniz.

- **Git:** Proje dosyalarını klonlamak veya yönetmek için Git kurulu olmalıdır.

- **Telegram Bot Token:** Kendi Telegram botunuzu oluşturmak için [BotFather](https://t.me/BotFather) ile konuşmanız ve bir API token almanız gerekmektedir.

### Projeyi Klonlama

Öncelikle botun kaynak kodunu GitHub deposundan bilgisayarınıza klonlayın:

```bash
git clone https://github.com/Ferivonus/telegram_dizi_film_izleme_rust_Bot.git
cd telegram_dizi_film_izleme_rust_Bot
```

### .env Dosyası Oluşturma

Botunuzun Telegram API ile iletişim kurabilmesi için bir `.env` dosyasına ihtiyacı vardır. Projenizin ana dizininde (`Cargo.toml` ile aynı yerde) `.env` adında bir dosya oluşturun ve içine bot token'ınızı ekleyin.

Terminal üzerinden `.env` dosyasını oluşturmak ve düzenlemek için:

1.  **`nano`'yu Kurun (Eğer kurulu değilse):**

    - **Debian/Ubuntu tabanlı (Kali dahil):**

      ```bash
      sudo apt update
      sudo apt install nano
      ```

    - **Arch Linux tabanlı:**

      ```bash
      sudo pacman -Syu
      sudo pacman -S nano
      ```

    - Eğer `nano` kurmak istemiyorsanız veya kuramıyorsanız, bunun yerine `vi` veya `vim` gibi başka bir metin düzenleyici kullanabilirsiniz (örn: `vi .env`).

2.  **`.env` Dosyasını Oluşturun/Düzenleyin:**
    Projenizin kök dizininde (`telegram_dizi_film_izleme_rust_Bot` klasörünün içinde) aşağıdaki komutu çalıştırın:

    ```bash
    nano .env
    ```

    Açılan düzenleyiciye aşağıdaki satırı ekleyin:

    ```
    BOT_TOKEN=BURAYA_BOT_TOKENINIZI_YAZIN
    ```

    `BURAYA_BOT_TOKENINIZI_YAZIN` kısmını BotFather'dan aldığınız gerçek bot token'ınızla değiştirin.

3.  **Kaydedin ve Çıkın:**

    - `nano` içinde: `Ctrl + O` (kaydet), `Enter` (onayla), `Ctrl + X` (çıkış).

### Dosya Yapısı

Bot, film ve dizi listelerinizi aşağıdaki metin dosyalarında saklar:

- `filmler.txt`: Tüm film listenizi içerir. Her film adı yeni bir satırda olmalıdır (örn: `Matilda (1996)`).
- `diziler.txt`: Tüm dizi listenizi içerir. Her dizi adı yeni bir satırda olmalıdır (örn: `Zamanın Kapıları (2 Sezon)`).
- `izlenen_filmler.txt`: İzlediğiniz filmleri içerir. Bot bu dosyayı otomatik olarak günceller.
- `izlenen_diziler.txt`: İzlediğiniz dizileri içerir. Bot bu dosyayı otomatik olarak günceller.

Bu dosyalar ilk çalıştırmada otomatik olarak oluşturulacaktır, ancak içine manuel olarak içerik ekleyebilirsiniz.

---

## Platforma Göre Çalıştırma

### Windows

Windows'ta botu çalıştırmak için projenizin kök dizininde (`Cargo.toml` dosyasının olduğu yer) aşağıdaki komutu çalıştırın:

```bash
cargo run --release
```

Bu komut, botu derler ve çalıştırır.

### Linux

Eğer botu bir Linux makinesinde derleyip çalıştırmak istiyorsanız:

#### Kali Linux

1.  **Git'i Kurun:**

    ```bash
    sudo apt update
    sudo apt install git
    ```

2.  **Rust ve Cargo'yu Kurun:**

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env # Veya terminali yeniden başlatın
    ```

3.  **Gerekli Derleme Araçlarını Kurun:**
    Kali Linux (Ubuntu/Debian tabanlı) için C/C++ derleme araçları gereklidir:

    ```bash
    sudo apt update
    sudo apt install build-essential
    ```

4.  **Botu Derleyin ve Çalıştırın:** Projenizin kök dizininde:
    ```bash
    cargo run --release
    ```

#### Arch Linux

1.  **Git'i Kurun:**

    ```bash
    sudo pacman -Syu
    sudo pacman -S git
    ```

2.  **Rust ve Cargo'yu Kurun:**

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env # Veya terminali yeniden başlatın
    ```

3.  **Gerekli Derleme Araçlarını Kurun:**
    Arch Linux için C/C++ derleme araçları gereklidir:

    ```bash
    sudo pacman -Syu
    sudo pacman -S base-devel
    ```

4.  **Botu Derleyin ve Çalıştırın:** Projenizin kök dizininde:
    ```bash
    cargo run --release
    ```

---

## Bot Komutları

Bot ile etkileşim kurmak için aşağıdaki komutları kullanabilirsiniz:

- `/help`: Tüm komutların listesini ve açıklamalarını gösterir.

- `/recommend_film`: İzlenmemiş filmler listesinden rastgele bir film önerir.

- `/recommend_dizi`: İzlenmemiş diziler listesinden rastgele bir dizi önerir.

- `/izlenen_film_ekle <Film Adı>`: Önerilen veya izlediğin bir filmi 'izlenenler' listene ekler.

- `/izlenen_dizi_ekle <Dizi Adı>`: Önerilen veya izlediğin bir diziyi 'izlenenler' listene ekler.

- `/film_ekle <Film Adı>`: Yeni bir filmi ana filmler listesine ekler.

- `/dizi_ekle <Dizi Adı>`: Yeni bir diziyi ana diziler listesine ekler.

- `/watched_films`: İzlediğin tüm filmleri listeler.

- `/watched_series`: İzlediğin tüm dizileri listeler.

- `/tum_filmler`: Ana filmler listesindeki tüm filmleri gösterir.

- `/tum_diziler`: Ana diziler listesindeki tüm dizileri gösterir.

- `/izlenmemis_filmler`: Henüz izlemediğin filmleri listeler.

- `/izlenmemis_diziler`: Henüz izlemediğin dizileri listeler.

- `/hello`: Bota merhaba der ve sana özel bir mesaj gönderir.
