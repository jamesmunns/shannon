pub mod scale;

use std::time::Duration;
use rand::distributions::WeightedIndex;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use scale::*;
use rand::prelude::*;

struct Context {
    note: Option<Note>,
    notes: Vec<Note>,
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink_1 = Sink::try_new(&stream_handle).unwrap();
    let sink_2 = Sink::try_new(&stream_handle).unwrap();
    let sink_3 = Sink::try_new(&stream_handle).unwrap();
    let sink_4 = Sink::try_new(&stream_handle).unwrap();

    std::thread::sleep(Duration::from_secs(1));

    let mut chord_choices = vec![];
    let mut chord_weights = vec![];
    let mut scale_choices = vec![];
    let mut scale_weights = vec![];

    // // MINOR
    // chord_choices.extend_from_slice(MINOR_PRIMARY_CHORDS);
    // chord_weights.push(8);
    // for _ in 0..MINOR_PRIMARY_CHORDS.len() - 1 {
    //     chord_weights.push(4);
    // }
    // chord_choices.extend_from_slice(MINOR_SECONDARY_CHORDS);
    // for _ in 0..MINOR_SECONDARY_CHORDS.len() {
    //     chord_weights.push(1);
    // }

    // // MINOR
    // let scales = &[
    //     DORIAN_INTERVALS,
    //     PHRYGIAN_INTERVALS,
    //     AEOLIAN_INTERVALS,
    //     LOCRIAN_INTERVALS,
    //     HARMONIC_MINOR_INTERVALS,
    //     MELODIC_MINOR_ASCENDING_INTERVALS,
    //     MELODIC_MINOR_DESCENDING_INTERVALS,
    //     MINOR_TRIAD_INTERVALS,
    //     MINOR_7TH_TETRAD_INTERVALS,
    //     BLUES_MINOR_PENTATONIC_INTERVALS,
    //     MINOR_PENTATONIC_INTERVALS,
    // ];
    // scale_choices.extend_from_slice(scales);
    // for _ in 0..scales.len() {
    //     scale_weights.push(1);
    // }

    // MAJOR
    chord_choices.extend_from_slice(MAJOR_PRIMARY_CHORDS);
    chord_weights.push(8);
    for _ in 0..MAJOR_PRIMARY_CHORDS.len() - 1 {
        chord_weights.push(4);
    }
    chord_choices.extend_from_slice(MAJOR_SECONDARY_CHORDS);
    for _ in 0..MAJOR_SECONDARY_CHORDS.len() {
        chord_weights.push(1);
    }

    // MAJOR
    let scales = &[
        IONIAN_INTERVALS,
        LYDIAN_INTERVALS,
        MIXOLYDIAN_INTERVALS,
        NATURAL_MAJOR_INTERVALS,
        MAJOR_TRIAD_INTERVALS,
        DOMINANT_7TH_TETRAD_INTERVALS,
        MAJOR_7TH_TETRAD_INTERVALS,
        AUGMENTED_7TH_TETRAD_INTERVALS,
        AUGMENTED_MAJOR_7TH_TETRAD_INTERVALS,
        MAJOR_PENTATONIC_INTERVALS,
        BLUES_MAJOR_PENTATONIC_INTERVALS,
    ];
    scale_choices.extend_from_slice(scales);
    for _ in 0..scales.len() {
        scale_weights.push(1);
    }


    let chord_weights_idx = WeightedIndex::new(&chord_weights).unwrap();
    let scale_weights_idx = WeightedIndex::new(&scale_weights).unwrap();
    let mut rng = thread_rng();

    let mut chord_oct = rng.gen_range(4..6);
    let mut melod_oct = rng.gen_range(3..5);

    loop {
        let scale = scale_choices[scale_weights_idx.sample(&mut rng)];
        println!("Scale!");

        for _ in 0..4 {
            println!("Bar!");
            let (pitch, chords) = chord_choices[chord_weights_idx.sample(&mut rng)];
            let pitch = gen_chord(&mut chord_oct, pitch, chords, [&sink_2, &sink_3, &sink_4]);
            sixteen_bars_melody(&mut melod_oct, &sink_1, scale, [pitch; 16]);

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
fn gen_chord(oct: &mut u8, semi: Semitones, chords: &[Semitones], sinks: [&Sink; 3]) -> Pitch {
    let mut rng = thread_rng();
    *oct = (((*oct as i8) + rng.gen_range(-1..=1)) as u8).max(3).min(5);
    let note: Note = Note { pitch: Pitch::C, octave: *oct } + semi;

    for (st, sink) in chords.iter().zip(sinks.iter()) {
        if rng.gen::<u8>() < 64u8 {
            continue;
        }
        let note = note + *st;

        let freq = note.freq_f32() as u32;

        let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(2.0)).amplify(0.10).buffered().low_pass(10000);
        // println!("{:?} {:?} {:?}", note, 3, freq);
        sink.append(source);
    }

    note.pitch
}

fn sixteen_bars_melody(oct: &mut u8, sink: &Sink, scale: &[Semitones], pitches: [Pitch; 16]) {
    let mut rng = rand::thread_rng();
    *oct = (((*oct as i8) + rng.gen_range(-1..=1)) as u8).max(2).min(6);

    // for quarter_bar in 0..4 {
    for quarter_bar in 0..1 {
        let mut gen_notes = |ct, len, pitch| {
            for _beat in 0..ct {
                let semi = scale.choose(&mut rng).unwrap();

                let note = Note { pitch, octave: *oct } + *semi;

                let freq = note.freq_f32() as u32;
                let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(len)).amplify(0.20).buffered().low_pass(10000);
                sink.append(source);
            }
        };

        let gen_rest = |len| {
            let source = SineWave::new(1).take_duration(Duration::from_secs_f32(len)).amplify(0.00001).buffered().low_pass(10000);
            sink.append(source);
        };

        for bar in 0..1 {
            let mut rng = rand::thread_rng();
            let pitch = pitches[(quarter_bar * 4) + bar];

            let mut taken = 0;
            'fill: while taken < 4 {
                let take = rng.gen_range(1..=4);
                let mode = rng.gen_range(0..14);

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
                            1 => gen_notes(2, 2.0 / 8.0, pitch),
                            2 => gen_notes(3, 0.1666, pitch),
                            3 => gen_notes(1, 2.0 / 4.0, pitch),
                            _ => gen_rest(2.0 / 4.0),
                        }
                        taken += 1;
                    }
                    2 if mode <= 5 => {
                        match mode {
                            1 => gen_notes(4, 2.0 / 8.0, pitch),
                            2 => gen_notes(6, 0.1666, pitch),
                            3 => gen_notes(2, 2.0 / 4.0, pitch),
                            4 => gen_notes(3, 0.3333, pitch),
                            5 => gen_notes(1, 2.0 / 2.0, pitch),
                            _ => gen_rest(2.0 * 2.0 / 4.0),
                        }
                        taken += 2;
                    }
                    4 => {
                        match mode {
                            1 => gen_notes(8, 2.0 / 8.0, pitch),
                            2 => gen_notes(12, 0.1666, pitch),
                            3 => gen_notes(4, 2.0 / 4.0, pitch),
                            4 => gen_notes(6, 0.3333, pitch),
                            5 => gen_notes(2, 2.0 / 2.0, pitch),
                            6 => gen_notes(3, 0.6666, pitch),
                            7 => gen_notes(1, 2.0, pitch),
                            _ => gen_rest(4.0 * 2.0 / 4.0),
                        }
                        taken += 4;
                    }
                    _ => continue 'fill,
                }
            }
        }
    }
}



