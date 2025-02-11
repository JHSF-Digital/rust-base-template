# Estutura de pastas

```yml
my_project/
│── app/
│   ├── api/             # API Axum
│   │   ├── routes/
│   │   │   ├── health.rs
│   │   │   ├── users.rs
│   │   │   └── mod.rs
│   │   ├── server.rs
│   │   └── mod.rs
│   ├── worker/          # Worker para consumo de filas
│   │   ├── jobs/
│   │   │   ├── email_job.rs
│   │   │   ├── data_sync.rs
│   │   │   └── mod.rs
│   │   ├── consumer.rs
│   │   ├── producer.rs
│   │   └── mod.rs
│
│── core/                # Camada central (serviços, repositórios e contratos)
│   ├── contracts/       # Interfaces (traits)
│   │   ├── services/
│   │   │   ├── user_service.rs
│   │   │   ├── order_service.rs
│   │   │   └── mod.rs
│   │   ├── repositories/
│   │   │   ├── user_repo.rs
│   │   │   ├── order_repo.rs
│   │   │   └── mod.rs
│   │   ├── messaging/
│   │   │   ├── queue.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── services/        # Implementações dos serviços
│   │   ├── user_service.rs
│   │   ├── order_service.rs
│   │   └── mod.rs
│   ├── repositories/    # Implementações dos repositórios
│   │   ├── user_repo.rs
│   │   ├── order_repo.rs
│   │   └── mod.rs
│   ├── messaging/       # Implementação da fila
│   │   ├── rabbitmq.rs
│   │   └── mod.rs
│   ├── domain/          # Definição das entidades do sistema
│   │   ├── user.rs
│   │   ├── order.rs
│   │   └── mod.rs
│   ├── database/        # Conexão e migrações do banco
│   │   ├── connection.rs
│   │   ├── migrations/
│   │   └── mod.rs
│   └── mod.rs
│
│── config/              # Configurações do projeto
│   ├── api.rs
│   ├── worker.rs
│   ├── database.rs
│   ├── environment.rs
│   └── mod.rs
│
│── bin/                 # Entradas do sistema
│   ├── api.rs
│   ├── worker.rs
│
│── Cargo.toml
│── README.md
```