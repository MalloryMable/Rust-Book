# Mini Grep
This is a tool that mimics the classic grep functionality poorly but practices applying tools such as:
- Lifetimes
- Results and errors
- Modular project design
- Tests
- Interacting with the commandline and the user enviroment

## How to Use
minigrep PATTERN FILE

NOTE: rather than proper options the one option(CASE_INSENSITIVE) is set as an enviroment variable.
This value may be 0(case sensitive) or 1(case insensitive).
Set as its own command or with the following syntax:
CASE_INSENSITIVE=1 minigrep PATTERN FILE
