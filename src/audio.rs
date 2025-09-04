use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::SineWave;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Audio {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    bgm_sink: Sink,
    win_sink: Option<Sink>,    // <-- NUEVO: para no encimar victorias
}

impl Audio {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("No hay dispositivo de audio");

        // BGM dedicado y único
        let bgm_sink = Sink::try_new(&handle).unwrap();
        if let Ok(file) = File::open("assets/steps.mp3") {          // tu BGM aquí
            let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
            bgm_sink.append(src.repeat_infinite());
        } else {
            let src = SineWave::new(220.0).amplify(0.02).repeat_infinite();
            bgm_sink.append(src);
        }
        bgm_sink.play();

        Self { _stream: stream, handle, bgm_sink, win_sink: None }
    }

    /// Reproduce un paso corto (one-shot). No importa que se encimen: son cortitos.
    pub fn play_step(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Ok(file) = File::open("assets/pasos.mp3") {
                let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(src);
            } else {
                let src = SineWave::new(440.0)
                    .amplify(0.02)
                    .take_duration(Duration::from_millis(90));
                sink.append(src);
            }
            sink.detach(); // se libera solo cuando termina
        }
    }

    /// Reproduce la fanfarria de victoria. Siempre detiene la anterior primero.
    pub fn play_win(&mut self) {
        // Si había una victoria sonando, la paramos para evitar encimado.
        if let Some(s) = self.win_sink.take() {
            s.stop();
        }
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Ok(file) = File::open("assets/victoria.mp3") {
                let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(src);
            } else {
                // “ta-da” simple: do–mi–sol
                let a = SineWave::new(523.0).amplify(0.05).take_duration(Duration::from_millis(120));
                let b = SineWave::new(659.0).amplify(0.05).take_duration(Duration::from_millis(120));
                let c = SineWave::new(784.0).amplify(0.05).take_duration(Duration::from_millis(220));
                sink.append(a); sink.append(b); sink.append(c);
            }
            sink.play();
            self.win_sink = Some(sink);
        }
    }

    /// Útil si quieres silenciar o cambiar el BGM sin recrear `Audio`.
    pub fn restart_bgm(&mut self, path: Option<&str>) {
        self.bgm_sink.stop(); // detén el loop anterior
        self.bgm_sink = Sink::try_new(&self.handle).unwrap();
        if let Some(p) = path {
            if let Ok(file) = File::open(p) {
                let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
                self.bgm_sink.append(src.repeat_infinite());
            }
        } else {
            // fallback
            let src = SineWave::new(220.0).amplify(0.02).repeat_infinite();
            self.bgm_sink.append(src);
        }
        self.bgm_sink.play();
    }
}

/// Si por alguna razón recreas `Audio` al reiniciar el juego,
/// esto garantiza que no quede ningún loop previo sonando.
impl Drop for Audio {
    fn drop(&mut self) {
        self.bgm_sink.stop();
        if let Some(s) = self.win_sink.take() {
            s.stop();
        }
    }
}
