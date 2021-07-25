pub mod scale;

use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

use crate::scale::Note;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink_1 = Sink::try_new(&stream_handle).unwrap();
    let sink_2 = Sink::try_new(&stream_handle).unwrap();
    let sink_3 = Sink::try_new(&stream_handle).unwrap();
    let sink_4 = Sink::try_new(&stream_handle).unwrap();

    std::thread::sleep(Duration::from_secs(1));

    const BASE_CHORD: Note = Note { pitch: scale::Pitch::A, octave: 3 };
    const BASE_MELODY: Note = Note { pitch: scale::Pitch::A, octave: 4 };

    loop {
        for (base_semi, chord_semis) in scale::MINOR_PRIMARY_CHORDS.iter().cycle().take(4) {
            let note_chord = BASE_CHORD + *base_semi;
            let note_melody = BASE_MELODY + *base_semi;

            // Build chord on sink 2/3/4
            println!("\nCHORD:");
            for (st, sink) in chord_semis.iter().zip(&[&sink_2, &sink_3, &sink_4]) {
                let note = note_chord + *st;

                let freq = note.freq_f32() as u32;

                let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(2.0)).amplify(0.10);
                println!("{:?} {:?} {:?}", note, 3, freq);
                sink.append(source);
            }

            // Melody - build on sink 1
            println!("\nMelody:");
            for st in chord_semis.iter().cycle().take(6) {
                let note = note_melody + *st;

                let freq = note.freq_f32() as u32;

                let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.33333)).amplify(0.20);
                println!("{:?} {:?} {:?}", note, 3, freq);
                sink_1.append(source);
            }

            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            for sink in &[&sink_1, &sink_2, &sink_3, &sink_4] {
                sink.sleep_until_end();
            }
        }
    }
}


// Okay, what do I need to do?
//
// * Set up the actors so that they can generate music
//     * Probably need some kind of function that can understand scales, keys
// * Set up the director to choose certain settings, like key, etc.
