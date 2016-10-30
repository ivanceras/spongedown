extern crate pulldown_cmark;
extern crate svgbob;

use pulldown_cmark::Parser;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;
use std::fs::File;
use std::error::Error;
use std::io::Write;

fn main(){
    let arg = r#"

中文处理

## Hello

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

    "#;

    let parser = Parser::new(arg);
    let mut start_bob = false;
    let mut bob_text = String::new();
    let parser = parser.map(|event| match event {
        Event::Text(text) => {
            if start_bob {
                bob_text.push_str(&text);
                Event::Text("".into())
            }else{
                Event::Text(text)
            }
        }
        Event::Start(tag) => {
            match tag{
                Tag::CodeBlock(ref lang) => {
                    if lang == "bob" {
                        start_bob = true;
                        Event::Text("".into())
                    }else{
                        Event::Start(tag.clone())
                    }
                }
                _ => Event::Start(tag)
            }
        }
        Event::End(ref tag) => {
            match *tag{
                Tag::CodeBlock(ref lang) => {
                    if lang == "bob" {
                        start_bob = false;
                        let svg = svgbob::to_svg(&bob_text).to_string();
                        Event::Html(svg.into())
                    }else{
                        Event::End(tag.clone())
                    }
                }
                _ => Event::End(tag.clone())
            }
        }
        _ => event
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    save_to_file(&html);

}

fn save_to_file(html: &str)->Result<(),Box<Error>>{
    let mut f = try!(File::create("file.html"));
    try!(f.write_all(html.as_bytes()));
    Ok(())
}
