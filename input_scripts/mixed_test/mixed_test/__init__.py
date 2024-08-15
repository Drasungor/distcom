import csv
import base64
import random

def generate_miller_rabin_inputs(number, iterations_limit):
    # Convert number to 4-byte big endian
    big_endian_bytes = number.to_bytes(4, byteorder='big')
    byte_array = bytearray(big_endian_bytes)

    byte_array.extend(iterations_limit.to_bytes(4, byteorder='big'))

    while len(byte_array) < 1024:
        byte_array.extend(b'\x00')

    base64_data = base64.b64encode(byte_array).decode('utf-8')
    return base64_data

def generate_fermat_inputs(number):
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
    return base64_data

numbers_list = [561, 562]
for number in numbers_list:
    file_path = str(number) + '_mixed_big_endian.csv'
    with open(file_path, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow([generate_fermat_inputs(number)])
        writer.writerow([generate_miller_rabin_inputs(number, 1000)])

    print(f"Number {number} written in big endian format to {file_path} with padding.")
