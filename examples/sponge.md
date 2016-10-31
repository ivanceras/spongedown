
# Spongedown

```bob
                 _____________
 +---------+     \            \        +------------+
 | md+bob  |----->) spongedown )------>| html + svg |
 +---------+     /____________/        +------------+
```

Spongedown converts markdown to html with support for 
[svgbob diagrams](https://github.com/ivanceras/svgbobrus)



| 中文处理 | Data | CJK |
|----------|------|-----|
|**Table**  | `are`|supported  |
| as     | well  | |

The next `code block` fenced with `bob` will be rendered into an svg

```bob

                                        .--> Base::Class::Derived_A
                                       /
            .-.                       .----> Base::Class::Derived_B    
           (x1y)-------.             /         \
            '-'         \           /           .---> Base::Class::Derived
        Alice            \         /             \
            \             \       /               '--> Base::Class::Derived
             \             \     /
              \             \   .-----------> SVG                    
               \             \ /
                \    .-----------.
                 '--(    BOB      )
                     '-----------'
                       /  \ \ \
                      '    \ \ \  
                      |     \ \ \
                      .      \ \ '--- The::Latest
                     /|       \ \      \
                 Foo  '        \ \      '---- The::Latest::Greatest
                     /|         \ \
                 Bar  '          \ '- I::Am::Running::Out::Of::Ideas
                     /|           \
                 Bar  '            \
                     /              '--- Last::One
               Quux V 

+----------------------+
|                      |
|       中文处理       |
|       12345678       |
|                      |
+----------------------+

             .---.  .---. .---.  .---.    .---.  .---.
    OS API   '---'  '---' '---'  '---'    '---'  '---'
               |      |     |      |        |      |
               v      v     |      v        |      v
             .------------. | .-----------. |  .-----.
             |  文件系统  | | |   调度器  | |  | MMU |
             '------------' | '-----------' |  '-----'
                    |       |      |        |
                    v       |      |        v
                 .----.     |      |    .---------.
                 | IO |<----'      |    |   网络  |
                 '----'            |    '---------'
                    |              |         |
                    v              v         v
             .---------------------------------------.
             |              硬件抽象层               |
             '---------------------------------------'

```

## Improvements in svgbob
- CJK is now supported
- Supports a wide array of diagram element combinations


Supports normal code blocks too.


```rust
fn main(){
    println!("Hello world!");
}
```

### Links
* [Spongedown repo](https://github.com/ivanceras/spongedown)
* [Svgbob demo](https://ivanceras.github.io/svgbobrus/) 
    - [repo](https://github.com/ivanceras/svgbobrus)
* [Svgbob in hackernews](https://news.ycombinator.com/item?id=12621680)
* [pulldown-cmark](https://github.com/google/pulldown-cmark)

