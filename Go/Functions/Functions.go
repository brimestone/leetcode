package main

import "fmt"

func greet(name string) {
	fmt.Println("Hello, " + name)
}

func add(x, y int) int {
	return x + y
}

func double(x int) int {
	return x * 2
}

func isEven(x int) bool {
	return switchForOddOrEven(x)
	// if x%2 == 0 {
	// 	return true
	// }
	// return false
}

func switchForOddOrEven(x int) bool{
	switch x%2 {
		case 0: {
			return true
		}
		default: { 
			return false
		}
	}
}

type Person struct {
	name string
	age  int
	address string
}

func main() {
	greet(("World"))

	sum := add(4, 4)
	fmt.Println("Sum:", sum, "and is even:", isEven(sum), "and double:", double(sum))

	var person = Person{
		name:    "John Doe",
		age:     30,
		address: "123 Main St",
	}

	greet(person.name)
}