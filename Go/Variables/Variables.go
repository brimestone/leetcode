//Summary:
// Print basic information to the terminal using various variable
// creation techniques. The information may be printed using any
// formatting you like.
//
//Requirements:
//* Store your favorite color in a variable using the 'var' keyword //* Store your birth year and age (in years) in two variables using
// compound assignment
//* Store your first & last initials in two variables using block as.
//* Declare (but don't assign!) a variable for your age (in days),
// then assign it on the next line by multiplying 365 with the age
// variable created earlier
//
//Notes:
//* Use fmt.Println() to print out information
//* Basic math operations are:
//

package main

import "fmt"

func main() {
	fmt.Println("Hello World!.")

	name := "Ted Butler"

	var birthYear, age, favColor = 1982, 42, "Blue"

	fmt.Println("My name is", name, "and I was born in", birthYear, "and I am", (age * 365), "days old. My favorite color is", favColor)

	var partingMessage string

	partingMessage = "Goodbye World!"

	fmt.Println(partingMessage)

}
