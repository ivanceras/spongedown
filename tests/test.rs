#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate spongedown;

#[test]
fn test_parser() {
    let arg = "
```bob
.---.      .--.
|   |  -> (    )
`---'      `--'
```
";
    let html =  r#"<div class='bob_container' style='width:128px;height:48px;'><svg class="bob" font-family="arial" font-size="14" height="48" width="128" xmlns="http://www.w3.org/2000/svg">
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
    }
    circle.solid {
      fill:black;
    }
    circle.open {
      fill:transparent;
    }
    tspan.head{
        fill: none;
        stroke: none;
    }
    
</style>
<path d=" M 8 8 A 4 4 0 0 0 4 12 L 4 16 M 8 8 L 16 8 M 8 8 L 16 8 L 24 8 M 16 8 L 24 8 L 32 8 M 24 8 L 32 8 M 36 12 A 4 4 0 0 0 32 8 M 36 12 L 36 16 M 4 16 L 4 32 M 4 16 L 4 32 M 36 16 L 36 32 M 36 16 L 36 32 M 96 8 A 16 16 0 0 0 88 32 M 88 16 A 16 16 0 0 0 96 40 M 4 36 L 4 32 M 4 36 A 4 4 0 0 0 8 40 L 16 40 M 8 40 L 16 40 L 24 40 M 16 40 L 24 40 L 32 40 M 24 40 L 32 40 M 36 36 L 36 32 M 32 40 A 4 4 0 0 0 36 36" fill="none"/>
<path d="" fill="none" stroke-dasharray="3 3"/>
<line marker-end="url(#triangle)" x1="56" x2="68" y1="24" y2="24"/>
<line marker-end="url(#triangle)" x1="56" x2="68" y1="24" y2="24"/>
<line marker-end="url(#triangle)" x1="64" x2="68" y1="24" y2="24"/>
<circle class="open" cx="104" cy="24" r="20"/>
</svg></div>
"#;
    println!("---------");
    println!("{}", spongedown::parse(arg).unwrap());
    println!("---------");
    assert_eq!(spongedown::parse(arg).unwrap(), html);
}
