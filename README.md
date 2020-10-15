#intersect

Prints intersection of two files line by line.
For comparison lines are trimmed.
Empty lines are skipped.
Intersection detection is done via hashset intersection, so the output is unordered.

##### Usage

```shell script
intersect <file a> <file b>
```

##### Example

Given the file a:
```shell script
a
b
```
And the file b:
```shell script
  b
 
c

```
The output is:
```shell script
$ intersect file_a file_b
b
```