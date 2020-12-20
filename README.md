# tts-rs
Uma crate básica para utilização de espeak em rust lang.

## Como utilizar em Cargo.toml

```toml
tts = { git = "https://github.com/JoseCarlosSkar/tts-rs", branch = "main" }
```

## Exemplo em linux
```rust
extern crate tts;

fn main() {
    let speaker = tts::Linux::new();
    speaker.set_language("pt-br");
    speaker.speak("Olá Mundo!");
}
```
