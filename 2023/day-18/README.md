https://adventofcode.com/2023/day/18

# Notes for part2:

Let's consider the area for the folllowing:

```
R 2
D 2
L 1
D 1
L 1
U 3
```

```
xxx
x x
xxx
xx
```

Area is 11.

But area of the polygon with above instructions is only 5, why?
Let zoom the above matrix

```
xxx xxx xxx
xxx xxx xxx
xxx xxx xxx

xxx     xxx
xxx     xxx
xxx     xxx

xxx xxx xxx
xxx xxx xxx
xxx xxx xxx

xxx xxx
xxx xxx
xxx xxx
```

Let's draw the bounds of the area you're calculating:

```
xxx xxx xxx
xo- --- -ox
x|x xxx x|x

x|x     x|x
x|x     x|x
x|x     x|x

x|x xxx x|x
x|x xo- -ox
x|x x|x xxx

x|x x|x
xo- -ox
xxx xxx
```

```
1** *** **2
*o- --- -o*
*|x xxx x|*

*|x     x|*
*|x     x|*
*|x     x|*

*|x xxx x|*
*|x xo- -o*
*|x x|4 **3

*|x x|*
*o- -o*
6** **5
```

So first observation: alogn the perimeter the line cuts each pixel into half, and right half is in the polygon, left is outside not counted. 

So to get the area we'll need to add the half of the perimeter to the area of the polygon bounded by the lines.

Then ther are the corners - for each CW corner we'll need to ad 1/4 of a pixel, and for each CCW we'll need to remvoe 1/4 of a pixel. So (#(CW)+#(CCW))/4. #(CW) + #(CCW) = 4 So that is 1.

A = A inner + perimeter / 2 + 1.

In the above example it's
11 = 5 + 10/2 + 1

See https://en.wikipedia.org/wiki/Pick%27s_theorem for general version.




