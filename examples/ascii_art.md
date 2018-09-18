

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

Generated files use .atxt extension. ("a" as ASCII art : to NOT overide any existing .txt files!). Here is the result:

```bob
                      ,-.
                      `-'
                      /|\
     ,---.             |
     |Bob|            / \
     `-+-'           Alice
       |    hello      |
       |-------------->|
       |               |
       |  Is it ok?    |
       |<- - - - - - - |
     ,-+-.           Alice
     |Bob|            ,-.
     `---'            `-'
                      /|\
                       |
                      / \

```

Unicode
The default txt format uses only plain ASCII characters. It is possible to use few extended Unicode characters to have a slightly better result. You should use the -utxt flag in the command line, or the utxt format in the ANT task.

```bob

                      ┌─┐
                      ║"│
                      └┬┘
                      ┌┼┐
     ┌───┐             │
     │Bob│            ┌┴┐
     └─┬─┘           Alice
       │    hello      │
       │──────────────>│
       │               │
       │  Is it ok?    │
       │<─ ─ ─ ─ ─ ─ ─ │
     ┌─┴─┐           Alice
     │Bob│            ┌─┐
     └───┘            ║"│
                      └┬┘
                      ┌┼┐
                       │
                      ┌┴┐

```

They are two drawbacks of using utxt:

    The result is UTF-8 encoded, and sometimes this is an issue (mail gateway, editors...)
    The used font must have the used extended characters (like Courier, Courier New...)

Complexe Diagram
You can even try complex example if you wish.

@startuml
'hide footbox

participant "Bob on\nseveral lines" as Bob
actor Alice

Bob -> Alice : hello
note right of Alice
  this is a note
end note

Alice -> Bob : Is it ok\nwith a message that is\non several lines?

note right
  This other note
  should work
  on several lines
end note

== This is a separation ==

Bob -> Last : Yes it works!
Last -> Last : working in progress
note left : this is\nanother note

Last --> Last : working in progress

Last --> Bob : done

opt dummy comment
  Bob -> Last : Error\nOn\nSeveral\nLine
  Last --> Bob : None
else
  Last --> Bob : None
  Last -> Bob : None
else other
  Last -> Bob : None
  note over Alice, Last
    This is a long note
    over Alice and Last
  end note
  Last -> Bob : None
  Last -> Bob : None
end


@enduml


Example

```bob

                                             ,-.
                                             `-'
               ,-------------.               /|\
               |Bob on       |                |             ,----.
               |several lines|               / \            |Last|
               `------+------'              Alice           `-+--'
                      |        hello          |               |
                      |---------------------->|               |
                      |                       |               |
                      |                       | ,--------------!.
                      |                       | |this is a note|_\
                      |                       | `----------------'
                      |Is it ok               | ,----------------!.
                      |with a message that is | |This other note |_\
                      |on several lines?      | |should work       |
                      |<----------------------| |on several lines  |
                      |                       | `------------------'
                      |                       |               |
                      |              ======================== |
====================================== This is a separation =======================================
                      |              ======================== |
                      |                       |               |
                      |            Yes it works!              |
                      |-------------------------------------->|
                      |                       |               |
                      |                       ,------------!. |----.
                      |                       |this is     |_\|    | working in progress
                      |                       |another note  ||<---'
                      |                       `--------------'|
                      |                       |               |- - .
                      |                       |               |    | working in progress
                      |                       |               |< - '
                      |                       |               |
                      |                 done  |               |
                      |<- - - - - - - - - - - - - - - - - - - |
                      |                       |               |
     ______________________________________________________________________
     ! OPT  /  dummy comment                  |               |            !
     !_____/          |                       |               |            !
     !                |                       |               |            !
     !                |               Error   |               |            !
     !                |               On      |               |            !
     !                |               Several |               |            !
     !                |               Line    |               |            !
     !                |-------------------------------------->|            !
     !                |                       |               |            !
     !                |                 None  |               |            !
     !                |<- - - - - - - - - - - - - - - - - - - |            !
     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~!
     !                |                       |               |            !
     !                |                 None  |               |            !
     !                |<- - - - - - - - - - - - - - - - - - - |            !
     !                |                       |               |            !
     !                |                 None  |               |            !
     !                |<--------------------------------------|            !
     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~!
     ! [other]        |                       |               |            !
     !                |                 None  |               |            !
     !                |<--------------------------------------|            !
     !                |                       |               |            !
     !                |                    ,-------------------!.          !
     !                |                    |This is a long note|_\         !
     !                |                    |over Alice and Last  |         !
     !                |                    `---------------------'         !
     !                |                 None  |               |            !
     !                |<--------------------------------------|            !
     !                |                       |               |            !
     !                |                 None  |               |            !
     !                |<--------------------------------------|            !
     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~!
               ,------+------.              Alice           ,-+--.
               |Bob on       |               ,-.            |Last|
               |several lines|               `-'            `----'
               `-------------'               /|\
                                              |
                                             / \
```

Same example using Unicode

```bob
                                             ┌─┐
                                             ║"│
                                             └┬┘
               ┌─────────────┐               ┌┼┐
               │Bob on       │                │             ┌────┐
               │several lines│               ┌┴┐            │Last│
               └──────┬──────┘              Alice           └─┬──┘
                      │        hello          │               │
                      │──────────────────────>│               │
                      │                       │               │
                      │                       │ ╔═════════════╧══╗
                      │                       │ ║this is a note ░║
                      │                       │ ╚═════════════╤══╝
                      │Is it ok               │ ╔═════════════╧════╗
                      │with a message that is │ ║This other note  ░║
                      │on several lines?      │ ║should work       ║
                      │<──────────────────────│ ║on several lines  ║
                      │                       │ ╚═════════════╤════╝
                      │                       │               │
                      │              ╔════════╧═════════════╗ │
══════════════════════╪══════════════╣ This is a separation ╠═╪════════════════════════════════════
                      │              ╚════════╤═════════════╝ │
                      │                       │               │
                      │            Yes it works!              │
                      │──────────────────────────────────────>│
                      │                       │               │
                      │                       ╔══════════════╗│────┐
                      │                       ║this is      ░║│    │ working in progress
                      │                       ║another note  ║│<───┘
                      │                       ╚══════════════╝│
                      │                       │               │─ ─ ┐
                      │                       │               │    | working in progress
                      │                       │               │< ─ ┘
                      │                       │               │
                      │                 done  │               │
                      │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │
                      │                       │               │
     ╔══════╤═════════╪═══════════════════════╪═══════════════╪════════════╗
     ║ OPT  │  dummy comment                  │               │            ║
     ╟──────┘         │                       │               │            ║
     ║                │                       │               │            ║
     ║                │               Error   │               │            ║
     ║                │               On      │               │            ║
     ║                │               Several │               │            ║
     ║                │               Line    │               │            ║
     ║                │──────────────────────────────────────>│            ║
     ║                │                       │               │            ║
     ║                │                 None  │               │            ║
     ║                │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │            ║
     ╠════════════════╪═══════════════════════╪═══════════════╪════════════╣
     ║                │                       │               │            ║
     ║                │                 None  │               │            ║
     ║                │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │            ║
     ║                │                       │               │            ║
     ║                │                 None  │               │            ║
     ║                │<──────────────────────────────────────│            ║
     ╠════════════════╪═══════════════════════╪═══════════════╪════════════╣
     ║ [other]        │                       │               │            ║
     ║                │                 None  │               │            ║
     ║                │<──────────────────────────────────────│            ║
     ║                │                       │               │            ║
     ║                │                    ╔══╧═══════════════╧══╗         ║
     ║                │                    ║This is a long note ░║         ║
     ║                │                    ║over Alice and Last  ║         ║
     ║                │                    ╚══╤═══════════════╤══╝         ║
     ║                │                 None  │               │            ║
     ║                │<──────────────────────────────────────│            ║
     ║                │                       │               │            ║
     ║                │                 None  │               │            ║
     ║                │<──────────────────────────────────────│            ║
     ╚════════════════╪═══════════════════════╪═══════════════╪════════════╝
               ┌──────┴──────┐              Alice           ┌─┴──┐
               │Bob on       │               ┌─┐            │Last│
               │several lines│               ║"│            └────┘
               └─────────────┘               └┬┘
                                             ┌┼┐
                                              │
                                             ┌┴┐
```
