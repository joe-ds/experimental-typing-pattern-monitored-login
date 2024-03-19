# Experimental Typing Pattern Monitored Login

An experiment with a login system that looks not just at what your password is,
but how you type it.

## Usage
`experimental-login [ms]`  
where `ms` is the number of milliseconds to use for the threshold.

## Requirements
*  [ncurses](https://www.gnu.org/software/ncurses/ncurses.html)

## How It Works
*  Let the user enter a password.
*  Store the delay in milliseconds for each character.
*  Let the first character have a delay of 0, since how long the user takes
before they start entering their password is irrelevant.
*  Let the user enter this password again.
*  Store the delays like the first time around.
*  Compare the passwords to see if they're the same.
*  If they are, then take the absolute respective difference in the delays in
both attempts.
*  Take the arithmetic mean of these differences.
*  Compare this mean with a determined threshold.