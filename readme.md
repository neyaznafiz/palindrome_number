## Problem

**Given an integer num, return true if num is a palindrome, and false otherwise.**

</br>

Example 1: </br>
```
Input: num = 121
Output: true
```
Explanation: 121 reads as 121 from left to right and from right to left.

</br>

Example 2: </br>
```
Input: num = -121
Output: false
```
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

</br>

Example 3: </br>
```
Input: num = 10
Output: false
```
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

</br>

Constraints: </br>

$-2^{31} <= num <= 2^{31} - 1$
 
 </br>

**Follow up:** Could you solve it without converting the integer to a string?

 </br>
 </br>

## Solution

 - Check the `main.rs` in `src/main.rs` directory.