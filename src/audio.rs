use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::SineWave;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Audio {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    bgm_sink: Sink,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("No hay dispositivo de audio");
        let bgm_sink = Sink::try_new(&handle).unwrap();

        // Música de fondo (loop). Si no hay archivo, usa un zumbido suave.
        if let Ok(file) = File::open("assets/steps.mp3") {
            let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
            bgm_sink.append(src.repeat_infinite());
        } else {
            let src = SineWave::new(220.0).amplify(0.02).repeat_infinite();
            bgm_sink.append(src);
        }
        bgm_sink.play();
        Self { _stream: stream, handle, bgm_sink }
    }

    pub fn play_step(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Ok(file) = File::open("assets/pasos.mp3") {
                let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(src);
            } else {
                // “tap” cortito
                let src = SineWave::new(440.0)
                    .amplify(0.02)
                    .take_duration(Duration::from_millis(90));
                sink.append(src);
            }
            sink.detach(); // reproducir en background
        }
    }

    pub fn play_win(&self) {
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
            sink.detach();
        }
    }
}
