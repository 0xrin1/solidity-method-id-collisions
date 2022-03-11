# solidity-method-id-collisions
Finds method id collissions by permutating the alphabet in opposing directions and halting when the bytes4 of the keccak256 hash of the names are equal.

Not exactly useful since contracts with colliding method ids dont compile, but a fun experiment nevertheless.

Took about a day to find collision: `d()` and `mbrysd()`
