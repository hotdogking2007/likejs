# JavaScript syntax implementation library

```rs
likejs::var!(var str_text = "Hello, World!".to_string());
```
----------
```rs
let f = likejs::arrow!(() => {"Hello, World!".to_string()});
```
```rs
let f = likejs::arrow!(() => "Hello, World!".to_string());
```
```rs
likejs::normal_function!(function add(a, b){
   a+b
});
let int = add(12,13);
```