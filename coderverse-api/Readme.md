<!-- export DATABASE_URL=postgres://username:password@localhost/database_name -->

# Coderverse API

Bu uygulamada Coderverse API'si geliştirilmeye çalışılıyor. Bu API, Coderverse uygulaması için gerekli olan verileri sağlayacak olan bir REST API servisidir. Bu API, Postgresql veritabanında saklanan verileri dışarıya REST Api ile vermektedir.

## Ön Hazırlıklar

Docker üzerinde Postgresql veritabanı oluşturulması.

```bash
docker run --name postgresql -e POSTGRES_USER=scoth -e POSTGRES_PASSWORD=tiger -p 5432:5432 -d postgres
export DATABASE_URL=postgres://scoth:tiger@localhost/postgres
```

## Uygulama Çalıştırma

```bash
cargo run
```

## API Servisleri

```bash
// Üye Ekleme
curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe", "email": "john@doe.com"}' http://localhost:8000/members

// Üye Listeleme
curl -X GET http://localhost:8000/members

// Üye Detay
curl -X GET http://localhost:8000/members/1

// Üye Güncelleme
curl -X PUT -H "Content-Type: application/json" -d '{"name": "John Doe", "email": "new@email.com"}' http://localhost:8000/members/1

// Üye Silme
curl -X DELETE http://localhost:8000/members/1

// Üye Fotoğraf Ekleme
curl -X POST -H "Content-Type: application/json" -d '{"photo_url": "http://example.com/image.jpg"}' http://localhost:8000/members/1/photo
```

## Veritabanı Yapısı

- members
  - id: UUID
  - name: String
  - email: String
  - photo_url: String
