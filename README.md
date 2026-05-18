# Module 10 - Asynchronous Programming

## Experiment 1.1: Original timer from the book

Program pada package `timer` dibuat berdasarkan contoh executor sederhana dari Rust Async Book. Bagian utama program terdiri dari `TimerFuture`, `Spawner`, `Executor`, dan `Task`. `Spawner` memasukkan future ke queue, lalu `Executor` mengambil task tersebut dan melakukan polling. Ketika timer belum selesai, `TimerFuture` mengembalikan `Poll::Pending` dan menyimpan `waker`. Setelah thread timer selesai tidur selama dua detik, waker dipanggil agar task dapat dipoll kembali. Hasil akhirnya adalah pesan `howdy!` muncul lebih dulu, lalu setelah jeda timer muncul pesan `done!`.

Bukti run:

![Experiment 1.1](docs/screenshots/experiment-1-1.png)

## Experiment 1.2: Understanding how it works.

Pada eksperimen ini saya menambahkan kalimat `Mahendra's Computer: hey hey!` setelah pemanggilan `spawner.spawn(...)`. Kalimat tersebut muncul sebelum `howdy!` dan `done!` karena `spawn` hanya memasukkan future ke queue, bukan langsung menjalankan seluruh isi async block sampai selesai. Isi async block baru mulai diproses ketika `executor.run()` dipanggil. Saat executor melakukan polling pertama, pesan `howdy!` muncul, lalu `TimerFuture` mengembalikan `Poll::Pending` karena timer belum selesai. Setelah dua detik, thread timer memanggil `waker` sehingga task dimasukkan kembali ke queue. Executor kemudian melakukan polling lagi dan program mencetak `done!`.

Bukti run:

![Experiment 1.2](docs/screenshots/experiment-1-2.png)
