# eqt 0.1.0 

Compare numbers or text values

## Usage

```
eqt  <VALUE1> <VALUE2>
```

## Arguments
`<VALUE1>` - First value to compare (number or text)

`<VALUE2>` - Second value to compare (number or text)
    
## Comparison Rules
- Numbers are compared mathematically (`>`, `<`, `==`)
- Text is compared lexicographically (`==`, `!=`)
- Mixed types are converted to strings and compared
    
## Examples 
```
eqt 10 20
eqt 'hello' 'world'
eqt 5 '5'
```
