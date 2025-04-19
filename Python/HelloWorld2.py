# a = "Ril3y"

# b = "Butler"

# age = 15

# print(f"hi My name is {a} {b} and I am {age * 365} days old, thats {age} in years")


def main_1():
    """ Main program """
    # Code goes over here.
    print("Hello fro Main_1")

def main_2():
    """ Main program """
    # Code goes over here.
    print("Hello fro Main_2")



if __name__ == "__main__":
    for i in range(1, 3):
        eval(f"main_{i}()")
