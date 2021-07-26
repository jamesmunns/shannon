pub mod scale;

use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use scale::{BLUES_MINOR_PENTATONIC_INTERVALS, MINOR_7TH_TETRAD_INTERVALS, MINOR_PENTATONIC_INTERVALS, MINOR_TRIAD_INTERVALS, Pitch, Semitones};
use crate::scale::{MINOR_PRIMARY_CHORDS, MINOR_SECONDARY_CHORDS, Note};
use rand::prelude::*;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink_1 = Sink::try_new(&stream_handle).unwrap();
    let sink_2 = Sink::try_new(&stream_handle).unwrap();
    let sink_3 = Sink::try_new(&stream_handle).unwrap();
    let sink_4 = Sink::try_new(&stream_handle).unwrap();

    std::thread::sleep(Duration::from_secs(1));

    loop {
        let bases = sixteen_bars_chords([&sink_2, &sink_3, &sink_4]);
        sixteen_bars_melody(&sink_1, bases);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        for sink in &[&sink_1, &sink_2, &sink_3, &sink_4] {
            sink.sleep_until_end();
        }
    }
}


// Okay, what do I need to do?
//
// * Set up the actors so that they can generate music
//     * Probably need some kind of function that can understand scales, keys
// * Set up the director to choose certain settings, like key, etc.
fn sixteen_bars_chords(sinks: [&Sink; 3]) -> [Pitch; 16] {
    const NOTE: Note = Note { pitch: Pitch::A, octave: 4 };
    let mut pitches = [Pitch::A; 16];
    let mut chords: [&[Semitones]; 16] = [scale::MINOR_PRIMARY_CHORDS[0].1; 16];

    let mut choices = MINOR_PRIMARY_CHORDS.iter()
        .chain(MINOR_PRIMARY_CHORDS.iter())
        .chain(MINOR_SECONDARY_CHORDS.iter())
        .chain(MINOR_PRIMARY_CHORDS.iter())
        .chain(MINOR_PRIMARY_CHORDS.iter())

        .cycle();

    let mut rng = rand::thread_rng();
    let skip = rng.gen_range(1..1000);

    // The first and last bar should be in the root
    for (pitch, chords) in (&mut pitches[1..][..15].iter_mut()).zip(&mut chords[1..][..15].iter_mut()) {
        for _ in 0..skip {
            let _ = choices.next();
        }
        let choice = choices.next().unwrap();
        *chords = choice.1;
        *pitch = (NOTE + choice.0).pitch;
    }

    for (pitch, chords) in pitches.iter_mut().zip(chords.iter_mut()) {
        // let oct = rng.gen_range(-1..2);
        // let cur_oct: i32 = NOTE.octave.into();
        // let new_oct: i32 = cur_oct + oct;
        // let octave = new_oct as u8;

        for (st, sink) in chords.iter().zip(sinks.iter()) {
            let note = Note { pitch: *pitch, octave: 3 } + *st;

            let freq = note.freq_f32() as u32;

            let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(2.0)).amplify(0.10).buffered().low_pass(10000);
            // println!("{:?} {:?} {:?}", note, 3, freq);
            sink.append(source);
        }
    }

    pitches
}

fn sixteen_bars_melody(sink: &Sink, pitches: [Pitch; 16]) {
    let mut rng = rand::thread_rng();

    for quarter_bar in 0..4 {
        let mut scale = match rng.gen_range(0..4) {
            0 => MINOR_PENTATONIC_INTERVALS,
            1 => BLUES_MINOR_PENTATONIC_INTERVALS,
            2 => MINOR_7TH_TETRAD_INTERVALS,
            _ => MINOR_TRIAD_INTERVALS,
        }.iter().cycle();

        let mut gen_notes = |ct, len, pitch| {
            for _beat in 0..ct {
                for _ in 0..rng.gen_range(0..100) { let _ = scale.next(); }
                let note = Note { pitch, octave: 4 } + *scale.next().unwrap();

                let freq = note.freq_f32() as u32;
                let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(len)).amplify(0.20).buffered().low_pass(10000);
                sink.append(source);
            }
        };

        let gen_rest = |len| {
            let source = SineWave::new(1).take_duration(Duration::from_secs_f32(len)).amplify(0.00001).buffered().low_pass(10000);
            sink.append(source);
        };

        for bar in 0..4 {
            let mut rng = rand::thread_rng();
            let pitch = pitches[(quarter_bar * 4) + bar];

            let mut taken = 0;
            'fill: while taken < 4 {
                let take = rng.gen_range(1..=4);
                let mode = rng.gen_range(0..8);

                // 0. rest(N)
                // 1. eighth
                // 2. eighth-triplets
                // 3. quarter
                // 4. quarter-triplets
                // 5. half
                // 6. half-triplets
                // 7. whole
                match take {
                    1 if mode <= 3 => {
                        match mode {
                            0 => gen_rest(2.0 / 4.0),
                            1 => gen_notes(2, 2.0 / 8.0, pitch),
                            2 => gen_notes(3, 0.1666, pitch),
                            3 => gen_notes(1, 2.0 / 4.0, pitch),
                            _ => panic!()
                        }
                        taken += 1;
                    }
                    2 if mode <= 5 => {
                        match mode {
                            0 => gen_rest(2.0 * 2.0 / 4.0),
                            1 => gen_notes(4, 2.0 / 8.0, pitch),
                            2 => gen_notes(6, 0.1666, pitch),
                            3 => gen_notes(2, 2.0 / 4.0, pitch),
                            4 => gen_notes(3, 0.3333, pitch),
                            5 => gen_notes(1, 2.0 / 2.0, pitch),
                            _ => panic!()
                        }
                        taken += 2;
                    }
                    4 => {
                        match mode {
                            0 => gen_rest(4.0 * 2.0 / 4.0),
                            1 => gen_notes(8, 2.0 / 8.0, pitch),
                            2 => gen_notes(12, 0.1666, pitch),
                            3 => gen_notes(4, 2.0 / 4.0, pitch),
                            4 => gen_notes(6, 0.3333, pitch),
                            5 => gen_notes(2, 2.0 / 2.0, pitch),
                            6 => gen_notes(3, 0.6666, pitch),
                            7 => gen_notes(1, 2.0, pitch),
                            _ => panic!()
                        }
                        taken += 4;
                    }
                    _ => continue 'fill,
                }
            }
        }
    }
}



