# poc_cv_manager

Proof of Concept - CV Manager (Rust + Salvo + OpenApi + Sqlx + Postgres + Docker)

To run you only need Docker.

To start the VMs:
docker-compose up -d

To start the backend on the VM (creates OpenAPI interface at localhost:5800):
docker-compose exec app cargo run

The first time you run the commands above it will be slow... but afterwords it will be very fast.

The DB will be empty... but I have in the fixtures folder a script to populate the DB:
cd fixtures
sh load_sql.sh

After the DB is populated you can login with any of the users in the fixtures.
For username use the email.
For password just use password (it is the same for all users in the fixture).

The admin user is admin@admin.org.
For password just use password.
