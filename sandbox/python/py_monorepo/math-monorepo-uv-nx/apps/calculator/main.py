from add import add
from sub import subtract
from mult import multiply

def main():
    print("Welcome to the Monorepo Math Calculator!")
    a, b = 10, 5
    
    print(f"{a} + {b} = {add(a, b)}")
    print(f"{a} - {b} = {subtract(a, b)}")
    print(f"{a} * {b} = {multiply(a, b)}")

if __name__ == "__main__":
    main()
