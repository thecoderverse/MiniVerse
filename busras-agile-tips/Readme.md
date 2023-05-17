# Busra's Agile Tips

Bu .net çözümünde Agile metodolojilerle ilgili minik ipuçları veren bir uygulama geliştirilmeye çalışılıyor. İpuçlarını Postgresql'de saklayan ve dışarıya REST Api ile veren bir .Net 7 Web API servisi söz konusu. Postgresql maliyeti bu çözüm için biraz yüksek ama olsun :)

## Ön Hazırlıklar

Postgresql, Entity tarafları için yapılacak olanlar.

```bash
docker run --name postgresql -e POSTGRES_USER=scoth -e POSTGRES_PASSWORD=tiger -p 5432:5432 -v /data:/var/lib/postgresql/data -d postgres

# Api Projesine eklenen Nuget paketleri
dotnet add package Microsoft.EntityFrameworkCore
dotnet add package Microsoft.EntityFrameworkCore.Design
dotnet add package Npgsql.EntityFrameworkCore.PostgreSQL

# EF Migration Tool tarafında
# install için
dotnet tool install -g dotnet-ef
# güncelleme için
dotnet tool update -g dotnet-ef

# Migration planlarını çalıştırmak için
dotnet ef migrations add InitialCreate
dotnet ef database update
```