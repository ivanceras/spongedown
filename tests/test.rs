extern crate spongedown;
#[cfg(test)] 
#[macro_use] 
extern crate pretty_assertions;

#[test]
fn test_parser(){
    let arg = "
```bob
.---.      .--.
|   |  -> (    )
`---'      `--'
```
";
    let html =  r#"<svg font-family="arial" font-size="14" height="80" width="160" xmlns="http://www.w3.org/2000/svg">
<defs>
<marker id="triangle" markerHeight="10" markerUnits="strokeWidth" markerWidth="10" orient="auto" refX="15" refY="10" viewBox="0 0 50 20">
<path d="M 0 0 L 30 10 L 0 20 z"/>
</marker>
</defs>
<style>

    line, path {
      stroke: black;
      stroke-width: 2;
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
    }
    circle {
      stroke: black;
      stroke-width: 2;
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
      fill:white;
    }
    tspan.head{
        fill: none;
        stroke: none;
    }
    
</style>
<path d=" M 4 12 L 4 32 M 8 8 A 4 4 0 0 0 4 12 M 8 8 L 32 8 M 36 12 L 36 32 M 36 12 A 4 4 0 0 0 32 8 M 96 8 A 16 16 0 0 0 86 16 M 96 8 L 112 8 M 122 16 A 16 16 0 0 0 112 8 M 4 16 L 4 32 M 36 16 L 36 32 M 86 16 A 16 16 0 0 0 86 32 M 122 32 A 16 16 0 0 0 122 16 M 4 32 L 4 36 A 4 4 0 0 0 8 40 L 32 40 M 36 32 L 36 36 M 32 40 A 4 4 0 0 0 36 36 M 86 32 A 16 16 0 0 0 96 40 L 112 40 A 16 16 0 0 0 122 32" fill="none"/>
<path d="" fill="none" stroke-dasharray="3 3"/>
<line marker-end="url(#triangle)" x1="56" x2="68" y1="24" y2="24"/>
</svg>"#;
    println!("---------");
    println!("{}", spongedown::parse(arg));
    println!("---------");
    assert_eq!(spongedown::parse(arg), html);
}
