import mysql.connector

# TODO check if container is running and compose if not

conn = mysql.connector.connect(
    host="localhost",
    port=9999,
    user="rdke_user",
    password="rdke_password",
    database="rdke_parameters",
    auth_plugin="mysql_native_password",
)

cursor = conn.cursor()
# Schema
cursor.execute("""
CREATE TABLE IF NOT EXISTS simulations (
    sim_id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
""")

cursor.execute("""
CREATE TABLE IF NOT EXISTS parameters (
    sim_id INT,
    param_name VARCHAR(255),
    param_value DOUBLE,
    PRIMARY KEY (sim_id, param_name)
)
""")

try:
    # Commit
    cursor.execute("CALL dolt_commit('-Am', 'Initialize schema for rDke parameters')")
    conn.commit()
except mysql.connector.errors.DatabaseError as excp:
    print("⚠️  Error during commit: ", excp)

print("✅ RDKE parameter database initialised")

cursor.close()
conn.close()
