# portal 1 plugin in rust (template)
## general info
use this if you want to write a portal 1 plugin using rust

this is also adaptable to other source games (see comment in src/stubs.rs) (you may need to adjust build settings for that but you're on your own at that point)

## resources used
https://woz.blue/articles/non-cpp-plugin.html this article explains the basic things you need to do to get up and running

## build
only windows build available
1. install cargo
2. clone repo
3. `cargo build --release`
4. resulting dll is inside `./target/i686-pc-windows-msvc/release`
5. profit

unfortunately rust produces a huge dll even with no_std (around 300kb); currently i haven't found a way to get around that