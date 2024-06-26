# Solutions by Abigail

## [Challenge 1](https://perlweeklychallenge.org/blog/perl-weekly-challenge-004/#challenge-1)

Write a script to output the same number of PI digits as the size
of your script. Say, if your script size is `10`, it should print
`3.141592653`.

### Notes
We're going to encode the digits (after the decimal dot) in groups
of 9 in base-91. 9 base-10 digits need 5 base-91 digits. We represent
the base-91 digits as ASCII characters, from '#' (0) to '~' (90).
Now, '#' to '~' covers a range of 92; however, when encoding, we
will be skipping '\'. Choosing a range from '#' to '~' while skipping
'\' means we can easily use this in double quoted strings (for languages
which don't do interpolation).

This gives a reduction of 44% (9 decimals encoded in 5 characters). By 
using 3242 digits of Pi, we decidate 1800 characters to the encoding
of Pi, which gives us plenty of room to write code to do the decoding
and printing. We may even need to filler comments to make it all work out.

### Solutions
* [AWK](awk/ch-1.awk)
* [Bash](bash/ch-1.sh)
* [bc](bc/ch-1.bc)
* [C](c/ch-1.c)
* [Go](go/ch-1.go)
* [Java](java/ch-1.java)
* [Lua](lua/ch-1.lua)
* [Node.js](node/ch-1.node)
* [Pascal](pascal/ch-1.p)
* [Perl](perl/ch-1.pl)
* [Python](python/ch-1.py)
* [R](r/ch-1.r)
* [Ruby](ruby/ch-1.rb)
* [Scheme](scheme/ch-1.scm)
* [Tcl](tcl/ch-1.tcl)


## [Challenge 2](https://perlweeklychallenge.org/blog/perl-weekly-challenge-004/#challenge-2)

You are given a file containing a list of words (case insensitive
1 word per line) and a list of letters. Print each word from the
file that can be made using only letters from the list. You can use
each letter only once (though there can be duplicates and you can
use each of them once), you don't have to use all the letters.
(Disclaimer: The challenge was proposed by Scimon Proctor)

### Notes
We will assume the word list is passed in with a '-f' parameter.
The sets of letters are read from standard input.

### Solutions
* [GNU AWK](awk/ch-2.gawk)
* [Bash](bash/ch-2.sh)
* [C](c/ch-2.c)
* [Lua](lua/ch-2.lua)
* [Node.js](node/ch-2.js)
* [Perl](perl/ch-2.pl)
* [Pyton](python/ch-2.py)
* [Ruby](ruby/ch-2.rb)
