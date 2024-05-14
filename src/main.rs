fn main() {
   let result: bool = is_palindrome(121);
   println!("Result: {}", result);
}

/**
 * Checks if the input integer is a palindrome.
 * 
 * The function first handles special cases where negative numbers and numbers ending with zero are not considered palindromes.
 * It then iteratively reverses the number and compares it with the original number to determine if it is a palindrome.
 * 
 * Parameters:
 * - num: The integer to be checked for palindrome.
 * 
 * Returns:
 * - True if the input number is a palindrome, false otherwise.
 */
fn is_palindrome(num:i32) -> bool {
    /*
    Checking if the input number is negative or if it ends with a zero, excluding the case when the input number is 0. If either condition is true, the function returns false.
    */
    if num < 0 || (num % 10 == 0 && num != 0) { return false; };

    /*
    Here creates two mutable variable one of them is num of type i32 and assigns the input number to it,
    and another is reverse of type i32 and initializes it to 0. This variable will store the reversed number.
    */
    let mut num: i32 = num;
    let mut reverse: i32 = 0;

    /*
		In this while loop, continues as long as the original number is greater than the reversed number.
		Updates the reverse variable by shifting it one position to the left and adding the last digit of the original number.
		Divides the original number by 10, effectively removing the last digit in preparation for the next iteration of the loop.
		*/
    while num > reverse {
      reverse = reverse * 10 + num % 10;
      num /= 10;
    };

    /*
    Checking if the original number is equal to the reversed number or the reversed number divided by 10 (to handle cases
    where the original number has an odd number of digits) and returning true or false.
    */
    num == reverse || num == reverse / 10
}