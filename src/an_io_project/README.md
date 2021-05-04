## An IO Project

```bash
  cargo run <query_string> <file_name>
```
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; - CASE_INSENSITIVE: Set to search insensive.
### Samples:
```bash
  CASE_INSENSITIVE=1 cargo run oThEr hello.txt
```
#### To print result to file:
```bash
  cargo run orther hello.txt > results.txt
```