# Module 10 - Asynchronous Programming

## Experiment 1.1: Original timer from the book

Program pada package `timer` dibuat berdasarkan contoh executor sederhana dari Rust Async Book. Bagian utama program terdiri dari `TimerFuture`, `Spawner`, `Executor`, dan `Task`. `Spawner` memasukkan future ke queue, lalu `Executor` mengambil task tersebut dan melakukan polling. Ketika timer belum selesai, `TimerFuture` mengembalikan `Poll::Pending` dan menyimpan `waker`. Setelah thread timer selesai tidur selama dua detik, waker dipanggil agar task dapat dipoll kembali. Hasil akhirnya adalah pesan `howdy!` muncul lebih dulu, lalu setelah jeda timer muncul pesan `done!`.

Bukti run:

![Experiment 1.1](docs/screenshots/experiment-1-1.png)
