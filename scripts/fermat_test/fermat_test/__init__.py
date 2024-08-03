import csv
import base64
import random

def write_big_endian_inputs_to_csv(number, file_path):
    # Convert number to 4-byte big endian
    big_endian_bytes = number.to_bytes(4, byteorder='big')
    
    byte_array = bytearray(big_endian_bytes)
    random_numbers = []


    while len(random_numbers) < 20 and (len(random_numbers) < (number - 2)):
        generated_number = random.randint(2, number - 1)
        if not (generated_number in random_numbers):
            random_numbers.append(generated_number)
            byte_array.extend(generated_number.to_bytes(4, byteorder='big'))

    print(len(random_numbers))

    while len(byte_array) < 1024:
        byte_array.extend(b'\x00')

    base64_data = base64.b64encode(byte_array).decode('utf-8')

    # Write to CSV file
    with open(file_path, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow([base64_data])

# Example usage
number = 7841
file_path = 'fermat_big_endian.csv'
write_big_endian_inputs_to_csv(number, file_path)

print(f"Number {number} written in big endian format to {file_path} with padding.")
