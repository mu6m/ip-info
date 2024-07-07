import csv
import ipaddress
import json

# Function to read CSV and generate JSON
def csv_to_json(csv_file, output_file):
    ip_data = {}
    with open(csv_file, 'r') as file:
        reader = csv.DictReader(file)
        for row in reader:
            start_ip = int(ipaddress.ip_address(row['start']))
            asn = row['asn']
            ip_data[start_ip] = asn
    
    with open(output_file, 'w') as json_file:
        json.dump(ip_data, json_file, indent=4)


json_data = csv_to_json('asn_ip.csv','data.json')
print("DONE CONVERTING")
