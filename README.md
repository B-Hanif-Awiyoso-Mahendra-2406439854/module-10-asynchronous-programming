# Module 10 - Asynchronous Programming

## Experiment 1.1: Original timer from the book

Di eksperimen pertama ini saya mencoba menjalankan contoh timer dari Rust Async Book. Programnya memakai `TimerFuture`, `Spawner`, `Executor`, dan `Task`. Awalnya saya masih perlu melihat lagi hubungan antar bagian ini, karena executor-nya dibuat manual dan bukan memakai runtime seperti Tokio. Dari hasil run, program mencetak `howdy!`, lalu setelah sekitar dua detik baru mencetak `done!`. Ini terjadi karena `TimerFuture` menunggu thread timer selesai terlebih dahulu sebelum future dianggap selesai. Menurut saya bagian yang paling penting di sini adalah `waker`, karena dari situ task yang tadinya pending bisa dijalankan lagi oleh executor.

Bukti run:

![Experiment 1.1](docs/screenshots/experiment-1-1.png)

## Experiment 1.2: Understanding how it works.

Di eksperimen ini saya menambahkan print `Mahendra's Computer: hey hey!` setelah bagian `spawner.spawn(...)`. Setelah dijalankan, ternyata `hey hey!` muncul lebih dulu daripada `howdy!` dan `done!`. Awalnya ini agak membingungkan, karena secara kode `howdy!` ditulis di dalam `spawn` sebelum print `hey hey!`. Setelah diperhatikan, `spawn` ternyata hanya memasukkan task ke queue, belum langsung menjalankan isi async block sampai selesai. Isi async block baru diproses ketika `executor.run()` dipanggil. Karena itu print yang berada setelah `spawn` bisa muncul duluan, lalu executor baru mulai mem-poll task async-nya.

Bukti run:

![Experiment 1.2](docs/screenshots/experiment-1-2.png)

## Experiment 1.3: Multiple Spawn and removing drop

Pada bagian ini saya menambahkan beberapa `spawner.spawn(...)` supaya ada lebih dari satu task yang dijalankan. Setelah dijalankan, semua pesan `howdy`, `howdy2`, dan `howdy3` muncul, lalu setelah timer selesai muncul pesan `done`, `done2`, dan `done3`. Urutan selesai task bisa berbeda-beda sedikit karena task-task tersebut menunggu timer secara asynchronous. `Spawner` berperan untuk mengirim task ke queue, sedangkan `Executor` mengambil task dari queue dan melakukan polling. `drop(spawner)` juga ternyata penting, karena bagian itu menandakan bahwa tidak ada task baru lagi yang akan dikirim dari spawner utama. Saat saya mencoba menghilangkan `drop(spawner)`, program bisa terlihat tidak berhenti sendiri karena executor masih menunggu kemungkinan task baru dari channel. Dari eksperimen ini saya jadi lebih paham bahwa spawner, executor, dan drop saling berhubungan dalam mengatur kapan task masuk, dijalankan, dan kapan program boleh selesai.

Bukti run dengan `drop(spawner)`:

![Experiment 1.3](docs/screenshots/experiment-1-3.png)

Catatan percobaan tanpa `drop(spawner)`: output task tetap dapat muncul, tetapi proses tidak selesai sendiri karena executor masih menunggu pesan baru dari channel.

## Experiment 2.1: Original code, and how it run

Untuk eksperimen 2.1 saya mencoba menjalankan broadcast chat dari Comprehensive Rust. Server dijalankan di satu terminal, lalu client dijalankan di beberapa terminal lain. Pada tahap ini port yang dipakai masih port awal, yaitu `2000`. Setelah tiga client berhasil connect, saya mencoba mengetik pesan dari salah satu client. Pesan tersebut masuk ke server melalui websocket, lalu server mengirim ulang pesan itu ke semua client yang sedang terhubung. Jadi client tidak saling mengirim pesan secara langsung, tetapi semuanya lewat server. Dari hasil ini terlihat kenapa websocket cocok untuk aplikasi chat, karena koneksi bisa tetap terbuka dan server bisa langsung mengirim pesan baru ke client.

Cara menjalankan server:
`cargo run -p broadcast-chat --bin server`

Cara menjalankan client:
`cargo run -p broadcast-chat --bin client`

Bukti run:

![Experiment 2.1](docs/screenshots/experiment-2-1-1.png)
![Experiment 2.1](docs/screenshots/experiment-2-1-2.png)
![Experiment 2.1](docs/screenshots/experiment-2-1-3.png)
![Experiment 2.1](docs/screenshots/experiment-2-1-4.png)
![Experiment 2.1](docs/screenshots/experiment-2-1-5.png)

## Experiment 2.2: Modifying port

Di eksperimen ini saya mengubah port websocket dari `2000` menjadi `8080`. Port ini tidak cukup diganti di satu tempat saja, karena server dan client harus menunjuk ke alamat yang sama. Di server, bagian yang saya ubah adalah `TcpListener::bind("127.0.0.1:8080")`. Di client, bagian yang saya ubah adalah URL websocket menjadi `ws://127.0.0.1:8080`. Prefix `ws://` menunjukkan bahwa koneksi yang dipakai adalah websocket. Kalau hanya server yang diganti, client masih mencoba connect ke port lama dan hasilnya gagal. Kalau hanya client yang diganti, client mencoba port baru tetapi server tidak mendengarkan di port tersebut. Setelah keduanya sama-sama memakai `8080`, aplikasi chat bisa berjalan lagi seperti sebelumnya.

Cara menjalankan server:
`cargo run -p broadcast-chat --bin server`

Cara menjalankan client:
`cargo run -p broadcast-chat --bin client`

Bukti run:

![Experiment 2.2](docs/screenshots/experiment-2-2-1.png)
![Experiment 2.2](docs/screenshots/experiment-2-2-2.png)

## Experiment 2.3: Small changes, add IP and Port

Pada eksperimen ini saya menambahkan informasi IP dan port pengirim ke pesan chat. Bagian ini saya ubah di server, karena server punya informasi alamat client dari hasil `listener.accept().await`. Saat server menerima pesan teks dari websocket, pesan itu saya ubah dulu menjadi format `{addr}: {message}` sebelum dikirim ke broadcast channel. Dengan begitu client lain bisa melihat pesan tersebut berasal dari koneksi yang mana. Saya memilih menaruh perubahan ini di server karena kalau alamat ditulis dari client, client bisa saja menulis identitas apa pun. Setelah dicoba, pesan yang muncul di terminal client sekarang punya tambahan alamat seperti `127.0.0.1:xxxxx`. Ini membuat alur pengiriman pesan antar client jadi lebih mudah diamati.

Cara menjalankan server:
`cargo run -p broadcast-chat --bin server`

Cara menjalankan client:
`cargo run -p broadcast-chat --bin client`

Bukti run:

![Experiment 2.3](docs/screenshots/experiment-2-3-1.png)
![Experiment 2.3](docs/screenshots/experiment-2-3-2.png)

## Experiment 3.1: Original code

Di eksperimen 3.1 saya menambahkan client webchat berbasis Yew. Web client ini dijalankan di browser dengan Trunk dan terhubung ke server websocket di `ws://127.0.0.1:8080`. Sebelum membuka webchat, server dari Tutorial 2 harus dijalankan dulu. Setelah halaman terbuka dan statusnya connected, pesan bisa dikirim dari browser ke server. Server kemudian membroadcast pesan itu seperti pada eksperimen sebelumnya. Pada tahap ini tampilannya masih sederhana karena fokus saya adalah memastikan koneksi websocket dari browser ke server Rust berjalan. Dari bagian ini saya melihat bahwa konsep asynchronous juga muncul di sisi browser, karena halaman tetap bisa dipakai sambil menunggu pesan baru dari websocket.

Cara menjalankan server:
`cargo run -p broadcast-chat --bin server`

Cara menjalankan webchat:
`rustup target add wasm32-unknown-unknown`
`cargo install trunk`
`cd webchat-yew`
`trunk serve --port 8081`

Bukti run:

![Experiment 3.1](docs/screenshots/experiment-3-1-1.png)
![Experiment 3.1](docs/screenshots/experiment-3-1-2.png)

## Experiment 3.2: Be Creative!

Untuk bagian kreativitas, saya mengubah tampilan webchat supaya tidak terlalu polos. Saya membuat layout dua panel, yaitu sidebar di kiri dan area chat di kanan. Sidebar berisi nama aplikasi, status koneksi, dan sedikit konteks tentang aplikasi. Pesan di area chat juga saya buat seperti kartu kecil supaya lebih mudah dibaca. Selain tampilan, browser client juga saya ubah agar mengirim pesan dalam format JSON dengan field `from` dan `text`. Kalau server belum berjalan, status akan menunjukkan bahwa aplikasi masih menunggu server dan tombol kirim tidak aktif. Layout juga dibuat responsif supaya saat layar kecil, tampilannya berubah menjadi satu kolom. Menurut saya perubahan ini membuat webchat terasa lebih seperti aplikasi yang benar-benar dipakai, bukan hanya demo input dan tombol.

Cara menjalankan server:
`cargo run -p broadcast-chat --bin server`

Cara menjalankan webchat:
`cd webchat-yew`
`trunk serve --port 8081`

Bukti run:

![Experiment 3.2](docs/screenshots/experiment-3-2-1.png)
![Experiment 3.2](docs/screenshots/experiment-3-2-2.png)

## Bonus: Rust Websocket server for YewChat!

Untuk bonus, saya mencoba membuat server Rust dari Tutorial 2 bisa dipakai oleh YewChat dari Tutorial 3. Tantangannya ada di format pesan, karena client terminal mengirim teks biasa, sedangkan webchat lebih enak kalau memakai JSON dengan `from` dan `text`. Solusi yang saya pakai adalah server mencoba membaca pesan sebagai JSON terlebih dahulu. Kalau pesan valid JSON, server akan membroadcast JSON itu lagi. Kalau pesan bukan JSON, server mengubahnya menjadi JSON dengan `from` berisi IP dan port pengirim, lalu `text` berisi pesan asli. Dengan cara ini, YewChat tetap bisa menampilkan label pengirim secara rapi, tetapi client terminal juga masih bisa digunakan. Saya menganggap perubahan ini berhasil karena YewChat bisa connect ke server Rust, mengirim pesan, dan menerima pesan broadcast. Menurut saya server Rust lebih menarik untuk kasus ini karena tipe data pesannya bisa dibuat jelas dengan `serde`, walaupun versi JavaScript mungkin lebih cepat untuk dibuat di awal.

Cara menjalankan server Rust:
`cargo run -p broadcast-chat --bin server`

Cara menjalankan YewChat:
`cd webchat-yew`
`trunk serve --port 8081`

Bukti run:

![Bonus](docs/screenshots/bonus-1.png)
![Bonus](docs/screenshots/bonus-2.png)
