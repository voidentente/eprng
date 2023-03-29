<br>
<p align="center">
    <img src="assets/light.svg#gh-dark-mode-only" height="50"/><img src="assets/dark.svg#gh-light-mode-only" height="50"/>
</p>

## What

This is not a pseudorandom number generator.

This is an _extremely_ pseudorandom number generator.

It trades in equal distribution with compactness, portability, and performance.

It is `#![no_std]` by nature with only `core` as dependency, and `alloc` if you
use the feature `distribution`.

EPRNG can fill buffers with bytes and digit chars in bases up to 36.

You should take a look at the generator's distributions to see if this works for you.

## How

The generators are seeded with a pointer that is taken by value.

The "source of entropy" is therefore implicitly given by the runtime.

Basically, the generators count the zeros in the pointer, and count the pointer up.

The pointer is never dereferenced however, which is why calling it a pointer is misleading.

This is why it is called "offset", as it continues to travel down the stack.

In order to continuously get new entropy, the offset must be mutated in some way;

Often, it is not enough to call `initial_offset()` again and call it a day, since

it might very likely return the exact same offset.

In fact, it is very possible that the same offset is returned when restarting the

same process! As such, it is a very naive PRNG.