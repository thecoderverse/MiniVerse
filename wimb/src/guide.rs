pub fn show_guide() {
    let help_text = "\
        Where is My Book?

        Kitaplarınızın kütüphanenin neresinde olduğunu bulmanıza yardım eden uygulama.
        Kullanımı;

        - Yeni bir kitap eklemek için
        wimb -add

        - 91 numaralı kitabı silmek için
        wimb -del 91

        - tüm kitapları olduğu gibi listelemek için
        wimb -all

        - kitapları adlarına göre tersten sıralamak ve ilk 25ini getirmek için
        wimb -list title desc 25

        - adında 'rogramming rus' geçen kitapları getirmek için
        wimb -find \"rograming rus\"
    ";

    println!("{}", help_text);
}
