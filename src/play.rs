use rodio::{source::SineWave, OutputStream, Sample, Sink, Source};
use std::time::Duration;

use crate::ast::*;

pub fn play(music: Music) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for i in music.sounds {
        sink.set_volume((i.volume as f32) / 255.0);
        let duration = Duration::from_millis(i.duration as u64);

        let notes = i
            .chord
            .iter()
            .map(|s| SineWave::new(s.hertz()).take_duration(duration))
            .collect();

        if let Some(chord) = mix_all(notes) {
            sink.append(chord.amplify(1.0 / i.chord.len() as f32));
        }

        sink.sleep_until_end();
    }
}

fn mix_all<S>(sources: Vec<S>) -> Option<Box<dyn Source<Item = S::Item> + Send>>
where
    S: Source + Clone + 'static + std::marker::Send,
    S::Item: Sample + Send,
{
    let mut iter = sources.into_iter();
    let first = iter.next()?;

    let mixed = iter.fold(
        Box::new(first) as Box<dyn Source<Item = S::Item> + Send>,
        |acc, s| Box::new(acc.mix(s)) as Box<dyn Source<Item = S::Item> + Send>,
    );

    Some(mixed)
}
