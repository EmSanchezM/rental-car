# Rental car API
Clean architecture for api rest in actix-web framework

## Pasos para comenzar

- Coonstruir app e iniciar docker `docker-compose up --build`
- Entrar al app desde docker `docker-compose exec app sh`
- Instalar diesel ORM `cargo install diesel_cli --no-default-features --features "postgres"`
- Crear el archivo de variables de entorno copiando el archivo `.env.example`
- Iniciar diesel ORM `diesel setup`
- Ejecutar las migraciones `diesel migration run`