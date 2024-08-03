import csv
import base64
import random

def write_big_endian_inputs_to_csv(number, iterations_limit, file_path):
    # Convert number to 4-byte big endian
    big_endian_bytes = number.to_bytes(4, byteorder='big')
    byte_array = bytearray(big_endian_bytes)

    byte_array.extend(iterations_limit.to_bytes(4, byteorder='big'))

    while len(byte_array) < 1024:
        byte_array.extend(b'\x00')

    base64_data = base64.b64encode(byte_array).decode('utf-8')

    # Write to CSV file
    with open(file_path, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow([base64_data])

# Example usage
number = 2373
file_path = 'miller_rabin_big_endian.csv'
write_big_endian_inputs_to_csv(number, 1000, file_path)

print(f"Number {number} written in big endian format to {file_path} with padding.")
