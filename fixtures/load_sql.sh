export PGPASSWORD=postgres
psql -h localhost -p 5432 -d cv_app_db -U postgres -a -f clientcompanies.sql
psql -h localhost -p 5432 -d cv_app_db -U postgres -a -f keywords.sql
psql -h localhost -p 5432 -d cv_app_db -U postgres -a -f jobfunctions.sql
psql -h localhost -p 5432 -d cv_app_db -U postgres -a -f users.sql
psql -h localhost -p 5432 -d cv_app_db -U postgres -a -f cvs.sql
psql -h localhost -p 5432 -d cv_app_db -U postgres -a -f coalesce.sql
