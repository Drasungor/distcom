import csv
import base64

def write_big_endian_to_csv(number, file_path):
    # Convert number to 4-byte big endian
    big_endian_bytes = number.to_bytes(4, byteorder='big')
    
    byte_array = bytearray(big_endian_bytes)

    while len(byte_array) < 1024:
        byte_array.extend(b'\x00')

    base64_data = base64.b64encode(byte_array)

    # Write to CSV file
    with open(file_path, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow([base64_data])

# Example usage
number = 5
file_path = 'big_endian.csv'
write_big_endian_to_csv(number, file_path)

print(f"Number {number} written in big endian format to {file_path} with padding.")
