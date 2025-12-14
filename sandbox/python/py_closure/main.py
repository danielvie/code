def build_counter():
    cont = 0

    def counter():
        nonlocal cont
        cont += 1
        return cont

    return counter


def main():
    ca = build_counter()
    cb = build_counter()
    print(ca())
    print(ca())
    print(ca())
    print(ca())
    print(cb())
    print(cb())


if __name__ == "__main__":
    main()
