import csv
import io

def main():
    # Example tuple with special characters (a comma and a quote)
    my_tuple = (101, 'Alice Smith', 'Sales, Marketing', 'She said "Hello!"')

    # 1. Create an in-memory text buffer
    output = io.StringIO()

    # 2. Create a CSV writer that writes to the buffer
    csv_writer = csv.writer(output)

    # 3. Write the tuple as a single row
    csv_writer.writerow(my_tuple)

    # 4. Get the resulting string and remove the trailing newline
    csv_string = output.getvalue().strip()

    print(csv_string)
    # Expected Output: 101,"Alice Smith","Sales, Marketing","She said ""Hello!"""

if __name__ == "__main__":
    main()
