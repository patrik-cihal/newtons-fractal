# Newton's fractal
## characteristics:
- created when using newton-rapson method to find roots of complex function
- each pixel is represented as complex number and then colored according to which root it converges to

## target:
1. let user define root count
2. display fractal and allow changing the polynomial function by shifting roots
3. make one iteration further when space is pressed

## requirements:
1. roots 
        -> coefficients -> derivative
        -> function
2. space translation
3. colors pallete

## procedure:
- get length of polynom from user 
- generate roots
- find the coefficients and derivative
- display the fractal with shiftable roots
- when root is clicked on, shift it with mouse
 
## model: 
- roots: Complex[]
- colors pallete: Colors[]
- camera: Camera

### how to find coefficients:
- go from 0..n
- find unique combinations of n-i-1 roots (eg. [[C1, C2], [C2, C3], [C1, C3]])
- multiply the inside (eg. [C1*C2, C2*C3, C1*C3])
- sum them together (eg. C1*C2+C2*C3+C1*C3)

