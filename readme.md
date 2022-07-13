# Newton's fractal
author: Patrik Cihal

 ![Newtons fractal](screenshot_newtons_fractal.png)



## annotation:
Visualize how newton's method works for real numbers and it's effects when applied in complex plane by displaying colors depending on which root individual pixel (represented as number in complex plane) converges to.


## Newton's method:
- used for finding roots where 


## theoretical part:

**real plane:**

1. Take last saved x-position (if first take random). Find the first derivative of the function at that point and turn it into linear function. 

$$f(x) = ax+b$$

$$a = f_0'(x_0)$$

$$b = f_0(x_0)-f_0'(x_0)*x_0$$

$$f(x) = f_0'(x_0)*x + f_0(x_0)-f_0'(x_0)*x_0$$

$$f(x) = f_0'(x_0)*(x-x_0) + f_0(x_0)$$

2. Find next x-position where this linear function crosses x-axis (root) and save that point.

$$f_0'(x_0)*x + f_0(x_0)-f_0'(x_0)*x_0 = 0$$

$$x = (f_0'(x_0)*x_0-f_0(x_0))/f_0'(x_0)$$

$$x = x_0-f_0(x_0)/f_0'(x_0)$$

**complex plane**:

Each pixel depending on it's position on screen is represented as a complex number.

Each complex root is represented as a point in 2d space and with random color.

Program for each pixel calculates $n$ iterations of newton's method (same equation as for real plane) and then colors it with the color of closest root to the newly calculated position.


## practical part:
The entire code is written in Rust, a compiled language which provides the required computational speed. 

**real plane:**



**complex plane:**
In order to use complex numbers I use a library called num.

User specifies the number of roots he wants and program places the roots $x_1, x_2, .., x_n$ in a cirle, resulting function becomes:
$(x-x_1) * (x-x_2) * (x-x_3) .. * (x-x_n)$
.

Then it calculates function coefficients from roots using permutation method.

From these coefficients we can easily construct derivative of this function.

$f(x) = a * x ^ A + b * x ^ B$

$f'(x) = a * A * x ^ {A - 1} + b * B * x ^ {B-1}$


User can then press arrows to decrease/increase the number of iterations applied to each pixel or shift roots of the function using mouse.


## what have I learned:
1. Managing time when dealing with longer projects
2. Complex numbers
3. Newton's method, fractal
