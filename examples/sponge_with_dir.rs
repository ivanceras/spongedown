extern crate spongedown;

use std::fs::File;
use std::error::Error;
use std::io::Write;

fn main() {
    let arg = r#"
[Link to](./README.md)

[Linux notes](/home/lee/PersonalBooks/notes/src/LINUX_NOTES.md)

[Link to parent readme](../README.md)

[Link to github](https://raw.githubusercontent.com/ivanceras/svgbob/master/TODO.md)

## Spongedown


| table    | Data | here|
|----------|------|-----|
| 1        | 2    | 3   |

```bob

                                        .--> Base::Class::Derived_A
                                       /
                                      .----> Base::Class::Derived_B    
      Something -------.             /         \
                        \           /           .---> Base::Class::Derived
      Something::else    \         /             \
            \             \       /               '--> Base::Class::Derived
             \             \     /
              \             \   .-----------> Base::Class::Derived_C 
               \             \ /
                '------ Base::Class
                       /  \ \ \
                      '    \ \ \  
                      |     \ \ \
                      .      \ \ '--- The::Latest
                     /|       \ \      \
 With::Some::fantasy  '        \ \      '---- The::Latest::Greatest
                     /|         \ \
         More::Stuff  '          \ '- I::Am::Running::Out::Of::Ideas
                     /|           \
         More::Stuff  '            \
                     /              '--- Last::One
       More::Stuff  V 

             .---.  .---. .---.  .---.    .---.  .---.
    OS API   '---'  '---' '---'  '---'    '---'  '---'
               |      |     |      |        |      |
               v      v     |      v        |      v
             .------------. | .-----------. |  .-----.
             |   Some     | | | Diagrams  | |  | here|
             '------------' | '-----------' |  '-----'
                    |       |      |        |
                    v       |      |        v
                 .----.     |      |    .---------.
                 | IO |<----'      |    |         |
                 '----'            |    '---------'
                    |              |         |
                    v              v         v
             .---------------------------------------.
             |                    Output             |
             '---------------------------------------'

```


And a text
with abbr

```rust
fn main(){
    println!("hello world!");
}

```

    "#;
    let html = spongedown::parse_with_base_dir(arg, "md").unwrap();
    println!("{}",html);
}

