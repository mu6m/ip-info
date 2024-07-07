import json
import psycopg2
from psycopg2 import sql

with open('data.json') as data_file:
    data = json.load(data_file)

db_params = {
    #YOUR_DB_PARAMS
}

conn = psycopg2.connect(**db_params)
cur = conn.cursor()

values = []
for i in data:
    values.append(i,data[i])

insert_query = """
INSERT INTO ip_info (ip, asn) VALUES %s
"""
psycopg2.extras.execute_values(
    cur, insert_query, values
)

conn.commit()

cur.close()
conn.close()