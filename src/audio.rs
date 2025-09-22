use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::SineWave;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Audio {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    bgm_sink: Sink,
    win_sink: Option<Sink>,
    // volumenes base y mute
    bgm_volume: f32,
    sfx_volume: f32,
    muted: bool,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("No hay dispositivo de audio");
        let bgm_sink = Sink::try_new(&handle).unwrap();

        // Música de fondo (loop)
        if let Ok(file) = File::open("assets/steps.mp3") {
            let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
            bgm_sink.append(src.repeat_infinite());
        } else {
            let src = SineWave::new(220.0).amplify(0.02).repeat_infinite();
            bgm_sink.append(src);
        }
        bgm_sink.set_volume(0.7); // volumen inicial
        bgm_sink.play();

        Self {
            _stream: stream,
            handle,
            bgm_sink,
            win_sink: None,
            bgm_volume: 0.7,
            sfx_volume: 0.9,
            muted: false,
        }
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
        let vol = if muted { 0.0 } else { self.bgm_volume };
        self.bgm_sink.set_volume(vol);
        // si había fanfarria sonando, también la silenciamos
        if let Some(ref s) = self.win_sink {
            s.set_volume(if muted { 0.0 } else { self.sfx_volume });
        }
    }

    pub fn toggle_muted(&mut self) -> bool {
        let new_state = !self.muted;
        self.set_muted(new_state);
        new_state
    }

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
            sink.set_volume(if self.muted { 0.0 } else { self.sfx_volume });
            sink.detach();
        }
    }

    pub fn play_win(&mut self) {
        if let Some(s) = self.win_sink.take() { s.stop(); }
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Ok(file) = File::open("assets/victoria.mp3") {
                let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(src);
            } else {
                let a = SineWave::new(523.0).amplify(0.05).take_duration(Duration::from_millis(120));
                let b = SineWave::new(659.0).amplify(0.05).take_duration(Duration::from_millis(120));
                let c = SineWave::new(784.0).amplify(0.05).take_duration(Duration::from_millis(220));
                sink.append(a); sink.append(b); sink.append(c);
            }
            sink.set_volume(if self.muted { 0.0 } else { self.sfx_volume });
            sink.play();
            self.win_sink = Some(sink);
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        self.bgm_sink.stop();
        if let Some(s) = self.win_sink.take() { s.stop(); }
    }
}
