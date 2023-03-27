# madpinger
[![CI](https://github.com/calculub/madpinger/actions/workflows/ci.yml/badge.svg)](https://github.com/calculub/madpinger/actions/workflows/ci.yml)

A utility to poll the state of courses in UW-Madison's [Course Search & Enroll](https://public.enroll.wisc.edu) site.

## Commands

### `madpinger section`
List information about a section with the (1) term code, (2) subject code, and (3) course ID.

#### Arguments & Flags
- `<SUBJECT_CODE>`: The subject code of the course
- `<COURSE_ID>`: The course ID of the course
- `-t, --term-code <TERM_CODE>`: Use sections from a specific term; defaults to Spring '23 (`1234`)

#### Examples
```bash
# the following are equivalent
madpinger section 266 022784
madpinger section 266 022784 -t 1234
madpinger section 266 022784 --term-code 1234
```

### `madpinger search`
Search for courses that match a given query.


#### Arguments & Flags
- `-s <SIZE>`: Return only the first `SIZE` results; defaults to 10
- `-t, --term-code <TERM_CODE>`: Use courses from a specific term; defaults to Spring '23 (`1234`)
- `-o`: Include open sections; defaults to false
- `-w`: Include waitlisted sections; defaults to false
- `-c`: Include closed sections; defaults to false

#### Examples
```bash
# search for 10 open or waitlisted courses that match "comp sci" in term `1234`
madpinger search -o -w "comp sci" 

# search for the first 5 open, waitlisted, or closed courses that match "calculus" in term `1234` 
madpinger search -t 1234 -s 5 -o -w -c calculus` 
```
