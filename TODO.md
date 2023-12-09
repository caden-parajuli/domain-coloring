# Todo (implementation steps)

- Currently there is an issue parsing `z + 1 - 1` vs. `z - 1 + 1`. It thinks the negative applies to the whole rest of the equation (right-associative instead of left?).
- Add a production for unary minus
- Fix the color rotation (not sure if this is actually a problem)
- Add options (ranges, color function, etc.) to form
