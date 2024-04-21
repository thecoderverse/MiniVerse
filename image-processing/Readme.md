# Image Processing

Bu uygulama, resim işleme işlemlerini gerçekleştiren bir console uygulamasıdır. Uygulama, resim dosyalarını okuyarak, üzerinde çeşitli işlemler yapar ve sonuçları yeni bir dosyada saklar. Uygulama, rust programlama dili ile yazılmıştır ve projede image paketi kullanılmıştır. 

## Kullanım

Uygulamayı çalıştırmak için, terminalde aşağıdaki komutu çalıştırmanız yeterlidir.

```bash
cargo run
```

## Filtreler

Uygulama, aşağıdaki filtreleri destekler.

- GrayScale
- Invert
- Sepia
- Threshold (0-255)
- Blur (0-100)

## Örnek

Uygulama, aşağıdaki resim üzerinde filtreler uygulayarak yeni bir resim oluşturur.

![Original Image](/image-processing/src/assets/football.jpeg)
![GrayScale Image](/image-processing/src/assets/gray_football.jpeg)
![Invert Image](/image-processing/src/assets/inverted_football.jpeg)
![Sepia Image](/image-processing/src/assets/sepia_football.jpeg)
![Threshold 50 Image](/image-processing/src/assets/threshold_50_football.jpeg)
![Threshold 100 Image](/image-processing/src/assets/threshold_100_football.jpeg)
![Threshold 150 Image](/image-processing/src/assets/threshold_150_football.jpeg)
![Threshold 200 Image](/image-processing/src/assets/threshold_200_football.jpeg)
![Blur 5 Image](/image-processing/src/assets/blur_5_football.jpeg)
![Blur 10 Image](/image-processing/src/assets/blur_10_football.jpeg)
