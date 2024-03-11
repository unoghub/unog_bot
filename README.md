# ÜNOG Bot

ÜNOG Discord sunucusunda kullanılan Discord bot'u

Şu anda sadece kullanıcıların doğrulanmasını sağlar.

## Doğrulanma

Kullanıcı formu doldurduğunda bot:

- Ayarlanan kanala bir mesaj atar. Bu mesajda kullanıcının formda yazdıkları ve _Doğrula_ butonu bulunur.
- Sheet'e kullanıcının Discord ID'sini ve formda yazdıklarını ekler.

_Doğrula_ butonuna basıldığında bot:

- Kullanıcının ismini formdaki isim soyisme ayarlar.
    - Aynı zamanda isim ve soyismin ilk harflerini büyük harf yapar.
- Belirlenmiş doğrulandı rolünü kullanıcıya verir.
- Sheet'teki doğrulanma durumunu günceller.

## Host'lama

> Bu bilgiler bot'u sunucusunda host'layan kişi için gerekli.

### Build'leme

1. [Rust'ı kurun](https://www.rust-lang.org/learn/get-started): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. `PATH`'a Rust'ın binary'lerini ekleyin: `source "$HOME/.cargo/env"`
3. [CMake](https://cmake.org)'i kurun: `sudo apt install cmake`
4. Bu repo'yu clone'layın: `git clone https://github.com/unoghub/unog_bot`
5. Repo klasörünün içine girin: `cd unog_bot`
6. Build'leyin: `cargo build --release`
7. Binary `target/release/unog_bot` konumunda

### Environment Variable'lar

> `.env` dosyası kullanılabilir.

- `GOOGLE_SERVICE_ACCOUNT_EMAIL`: Google Sheets için kullanılacak olan servis hesabının e-postası
- `GUILD_ID`: Komutların oluşturulacağı sunucunun ID'si
- `SHEET_ID`: Doğrulanma bilgilerinin kaydedileceği Google Sheet'in ID'si
- `TOKEN`: Bot'un Discord Developer Portal'dan alınan token'ı
- `TRACING_WEBHOOK_URL`: Bot'un error'larını vs. göndermek için kullanılacak webhook'un linki
- `VERIFICATION_SUBMISSIONS_CHANNEL_ID`: Kullanıcılar doğrulanma formunu doldurduğunda, formun ve dogrulama butonunun
  olduğu mesajın atılacağı kanalın ID'si, bu kanal sadece doğrulanma yetkisi olanların görebildiği bir kanal olmalı.
- `VERIFIED_ROLE_ID`: Kullanıcılar doğrulandığında onlara verilecek rolün ID'si

### Dosyalar

- `service_account_key.json`: Google Sheets için kullanılacak olan servis hesabının anahtarı

### Bot'u Davet Etme

#### Scope'lar

- bot
- applications.commands

#### İzinler

##### Genel

- Manage Roles
- Manage Nicknames

##### Kanallara Özel

- `/doğrulanma_mesajını_at` komutunun kullanıldığı kanalda:
    - Send Messages
- `VERIFICATION_SUBMISSIONS_CHANNEL_ID`:
    - Send Messages

#### Davet Linki

> Bu link, bot'un scope'larını ve izinlerini de belirtir.

`https://discord.com/api/oauth2/authorize?client_id={CLIENT_ID}&permissions=402655232&scope=applications.commands+bot`

> `{CLIENT_ID}`'yi bot'un application ID'si ile değiştirin.

#### Ekledikten Sonra Yapılacaklar

##### Bot'un Rolünün Konumu

Doğrulandı rolünün verilebilmesi için bot'un rolünü, doğrulandı rolünün ve doğrulanacak kullanıcının rollerinin üstüne
yerleştirin.

##### Doğrulanma Mesajının Atılması

Doğrulanma mesajını, `/doğrulanma_mesajını_at` komutuyla atın. Bu komut, kullanıldığı kanala doğrulanma mesajını atar.
Doğrulanma mesajı doğrulanma formunu açan butonun olduğu mesajdır.

> Bu komutu sadece _Sunucuyu Yönet_ izni olan kişiler görür ve kullanabilir.
