extern crate tts;

fn main() {
    let speaker = tts::Linux::new();
    speaker.set_language("pt-br");
    println!("Meu id: {}", speaker.id().unwrap());
    println!("Meu idioma: {}", speaker.get_language());
    println!("Meu rate: {}", speaker.get_rate());
    println!("Meu volume: {}", speaker.get_volume());
    speaker.speak("Eu utilizo o espeak como engine!");
}
