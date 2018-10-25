extern crate spongedown;

fn main() {
    let arg = r#"

![an image](image.jpg)

[Link to](./README.md)

[Linux notes](/home/lee/PersonalBooks/notes/src/LINUX_NOTES.md)


![{display:hidden}](records.csv)

![{id:records,display:hidden}](records.csv)

or

![][records]

```csv
col1,col2,col3
1, 2, 3
4, 5, 6
banana, batman, orange
```
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

[records]: ./records.csv

    "#;
    let html = spongedown::parse(arg).unwrap();
    println!("{}", html.content);
}
