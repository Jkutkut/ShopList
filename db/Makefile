include ../.env

SQL_FILE=./init.sql

reset_db:
	cat ${SQL_FILE} | docker exec -i shoplist-db psql -U ${DB_USER} -d ${DB_NAME}
