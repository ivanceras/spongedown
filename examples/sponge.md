

## Spongedown


| 中文处理 | Data | CJK |
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


And a text
with abbr

```rust
fn main(){
    println!("hello world!");
}

```

