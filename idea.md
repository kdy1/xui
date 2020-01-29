
  - We need to use css to make it fast on web.
  - API of flutter is cleaner than one of web
  


## Styles api

### TODOs
 - Support scss

### 1. styles!() like react jss
```
let classes = `styles!("foo.css")`

// Requires hot-reloading of rust code.
```
 
 
 ### 2. 
 
 ```rust
 
// Loads template and stylesheet
#[component]
struct Column;

// Loads template and stylesheet
#[component(templater = "row.html")]
struct Row;
```