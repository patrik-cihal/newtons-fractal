# Newton's fractal
students: Patrik Cihal
teacher: 


## anotation (project intentions):
Visualize how newton's method works for real numbers and visualize it's effects when applied in complex plane by showing colors depending on which root individual pixel converges to.


## introduction:
Newton's method is used for finding roots by iterating over points of the function and taking derivative that way taking the problem into linear function problem.


## theoretical part:
For each pixel on screen which is represented as complex number depending on its position I will calculated n iterations of newton's method and then color it accordingly to which root it converges.


## practical part:
The entire code is written in Rust, because of the speed that is required. 

**real plane:**
Choose random starting point. Take the first derivative of the function at that point and turn it into linear function 

**complex plane:**
In order to use complex numbers I use a library called num.
User specifies the number of roots he wants and program places the roots x1, x2, x3 in a cirle (function is in a form (x-x1)(x-x2)(x-x3))
Then it calculates coefficients using permutation method.
From these coefficients we can easily construct derivative of this function. (a*x^A+b*x^B) -> (a*A*x^(A-1)+b*B*x^(B-1))


## what have we learned:
1. Managing time when dealing with longer projects
2. Complex numbers
3. Newton's method, fractal

## thanks:
