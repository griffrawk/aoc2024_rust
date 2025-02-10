https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/

# [2024 Day 13] An explanation of the mathematics

Tutorial
You've probably seen a ton of meming or talk about using linear algebra to solve today's question, 
so let's give quick refresher on how this works.

For today's question, we're given a claw machine with two buttons A and B both of which move the 
claw some distance horizontally and vertically, and a location of a prize, and we're interested in 
determining a) if its possible to reach the prize just by pressing A and B and b) what's the 
cheapest way to do so.

Initially this seems like a dynamic programming problem (and certainly you can solve it with DP), 
but then part 2 becomes infeasible as the numbers simply grow too large even for DP to be a good 
optimization strategy, luckily we can solve this purely with some good ol' mathematics.

If we let A be the number of times we press the A button, B be the number of times we press 
the B button, (a_x, a_y) be the claw's movement from pressing A, (b_x, b_y) be the claws movement 
from pressing the B button, and (p_x, p_y) be the location of the prize, we can model the machine 
using a system of two linear equations:

```
A*a_x + B*B_x = p_x
A*a_y + B*b_y = p_y
```

All these equations are saying is "the number of presses of A times how far A moves the claw in 
the X direction plus the number of presses of B times how far B moves the claw in the X direction 
is the prize's X coordinate", and this is analogous to Y.

To give a concrete example, for the first machine in the example input, we can model it with the 
two equations

```
94A + 22B = 8400
34A + 67B = 5400
```

We just have to solve these equations for A and B, the good news we have two equations with two 
unknowns so we can use whichever method for solving two equations with two unknowns we'd like, 
you may have even learned a few ways to do so in high school algebra!

One really nice way to solve a system of n equations with n unknowns is a really nice rule named 
Cramer's rule, a nice theorem from linear algebra. Cramer's Rule generally is honestly a kind of 
a bad way to solve a system of linear equations (it's more used in theoretical math for proofs 
instead of actually solving systems) compared to other methods like Gaussian elimination, but 
for a 2x2 system like this it ends up being fine and gives us a nice algebraic way to solve the 
system.

I'm not going to cover how Cramer's Rule works in this post, since it would require an explanation 
on matrices and how determinants work and I doubt I could reasonably cover that in a single Reddit 
post, but if you're interested in further details 3blue1brown has a [really beautiful video on 
Cramer's Rule](https://www.youtube.com/watch?v=jBsC34PxzoM) (and honestly his entire essence of 
linear algebra series is top tier and got me through linear algebra in my first year of uni so 
I'd highly reccomend the entire series) 
and The Organic Chemistry Teacher has [a solid video actually covering the calculation itself 
for a 2x2 system](https://www.youtube.com/watch?v=vXqlIOX2itM). All we need to know is that 
applying Cramer's Rule to this system gives us:

```
A = (p_x*b_y - prize_y*b_x) / (a_x*b_y - a_y*b_x)
B = (a_x*p_y - a_y*p_x) / (a_x*b_y - a_y*b_x)
```

As an example, for the first machine in the sample input we get:

```
A = (8400\*67 - 5400\*22) / (94\*67 - 34\*22) = 80
B = (8400\*34 - 5400\*94) / (94\*67 - 34\*22) = 40
```

Which is the exact solution given in the problem text!

This now give us an easy way to compute the solution for any given machine (assuming the system 
of equations has one solution, which all the machines in the sample and inputs do, as an aside 
this means that for all machines in the input there's exactly one way to reach the prize, so 
saying "find the minimum" is a bit of a red herring). All we need to do is plug the machine's 
values into our formulas for A and B and we have the number of button presses, and as long 
as A and B are both integers, we can reach the prize on this machine and can calculate the 
price (it's just 3A + B). For part 2, all we have to do is add the offset to the prize before 
doing the calculation.

As a concrete example we can do this with a function like:

```
fn solve_machine(machine: &Machine, offset: isize) -> isize {
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
    let a = (prize.0 * machine.b.1 - prize.1 * machine.b.0) / det;
    let b = (machine.a.0 * prize.1 - machine.a.1 * prize.0) / det;
    if (machine.a.0 * a + machine.b.0 * b, machine.a.1 * a + machine.b.1 * b) == (prize.0, prize.1) {
        a * 3 + b
    } else {
        0
    }
}
```