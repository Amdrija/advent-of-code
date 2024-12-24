# Part 2

For part 2, I manually went through the gates and what they were supposed to do. 
Basically, I added the two numbers in the input to see which digits are different. 
These outputs were the candidates for swapping. Sometimes, it would be multiple digits in a row,
which means that probably the first digit is the culprit and then it messes with the results afterwards. 
Then, I manually check which gates do not conform to the full adder:
```
z = x ^ y ^ carry;
carry_next = (x & y) | ((x ^ y) & carry));
```