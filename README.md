# Diff
Diff for text files.
A naive implementation of [Myers shortest edit distance](https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/) algorithm. 

Files are: 

- [main.rs](https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/) https://github.com/tigranmt/Diff/blob/master/src/main.rs - processing of command line arguments, difference invokation and eventual presentation 
- [difflib.rs](https://github.com/tigranmt/Diff/blob/master/src/diff/difflib.rs) - algorithm implementation 
- [presenter.rs](https://github.com/tigranmt/Diff/blob/master/src/diff/presenter.rs) - implementation of built-in diff presenter and definition of base presenter as well. 


### How to use:

```
-s1  :  first  string to be compared
-s2  :  second string to be compared
-f1  :  first  file to be compared
-f2  :  second file to be compared
-h   :  print help

Note : -str and -f options are mutually exclusive. You have to specify one _or_ another in both parameters

Example:
Diff.exe -s1 'hello!' -s2 'hola!'
```
This will produce output like 

![screenshot](https://github.com/tigranmt/Diff/blob/master/res/screen.png)
