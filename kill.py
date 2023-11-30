from databend_py import Client
import time

# Databend connection configuration
config = {
    'host': '127.0.0.1',
    'port': 8000,  # Adjust according to your setup
    'user': 'root',
    'password': '',  # Empty password
    'database': 'default'
}

# Initialize Databend client
client = Client(**config)

# Function to check and kill REPLACE INTO queries
def check_and_kill_replace_queries():
    try:
        # Query for executing REPLACE INTO statements
        processlist = client.execute('SHOW PROCESSLIST')

        for process_array in processlist:
            if not process_array or len(process_array) < 1:  # Skip empty arrays
                continue

            for process in process_array:

                if len(process) < 8:  # Skip if tuple does not have enough elements
                    continue
                query_id, query_text = process[1], process[7]  # Extract query ID and query text
                if query_text and 'REPLACE INTO' in query_text.upper():
                    # Kill REPLACE INTO statement using the modified format
                    kill_query = f"KILL QUERY '{query_id}'"
                    client.execute(kill_query)
                    print(f"Killed REPLACE INTO query with ID: {query_id}")
    except Exception as e:
        print(f"Error: {e}")

# Regularly execute the check
interval = 5  # Check frequency in seconds
while True:
    check_and_kill_replace_queries()
    time.sleep(interval)