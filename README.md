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

TODO

### Environment Variable'lar

> `.env` dosyası kullanılabilir.

- `TOKEN`: Bot'un Discord Developer Portal'dan alınan token'ı
- `GUILD_ID`: Komutların oluşturulacağı sunucunun ID'si
- `VERIFICATION_SUBMISSIONS_CHANNEL_ID`: Kullanıcılar doğrulanma formunu doldurduğunda, formun ve dogrulama butonunun
  olduğu mesajın atılacağı kanalın ID'si, bu kanal sadece doğrulanma yetkisi olanların görebildiği bir kanal olmalı.
- `VERIFIED_ROLE_ID`: Kullanıcılar doğrulandığında onlara verilecek rolün ID'si

Bu bilgileri [Lara](https://lara.lv)'ya sorun:

- `LOGGING_WEBHOOK_URL`: Bot'un error'ları vs. için kullanılacak webhook'un linki
- `SPREADSHEET_ID`: Doğrulanma bilgilerinin kaydedileceği Google Sheet'in ID'si
- `GOOGLE_SERVICE_ACCOUNT_EMAIL`: Google Sheets için kullanılacak olan servis hesabının e-postası

### Dosyalar

- `google_service_account_private_key`: Google Sheets için kullanılacak olan servis hesabının gizli anahtarı

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
