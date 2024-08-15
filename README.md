# Rental car API
Clean architecture for api rest in actix-web framework

## Pasos para comenzar

- Iniciar docker `docker-compose up -d`
- Instalar diesel ORM `cargo install diesel_cli --no-default-features --features "postgres"`
- Crear el archivo de variables de entorno copiando el archivo `.env.example`
- Iniciar diesel ORM `diesel setup`
- Ejecutar las migraciones `diesel migration run` 