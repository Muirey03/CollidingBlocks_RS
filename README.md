#### Comically inefficient algorithm for cumputing digits of pi

This works using the "colliding blocks" phenomenon. A good explanation of this can be found here: https://youtu.be/HEfHFsfGXjs.

The algorithm is extremely inefficient, made worse by the fact that a decimal library must be used for floating point arithmetic to avoid floating point errors.