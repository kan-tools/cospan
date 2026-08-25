def greet(name):
    print("hello", name)


def add(a, b):
    return a + b


def subtract(a, b):
    return a - b


def main():
    greet("world")
    print(add(2, 3))
    print(subtract(5, 1))


if __name__ == "__main__":
    main()
