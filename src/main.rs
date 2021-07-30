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

    let mut keep_bar = None;

    loop {
        let scale = scale_choices[scale_weights_idx.sample(&mut rng)];
        println!("Scale!");

        for _ in 0..4 {
            println!("Bar!");
            let (pitch, chords) = chord_choices[chord_weights_idx.sample(&mut rng)];
            let pitch = gen_chord(&mut chord_oct, pitch, chords, [&sink_2, &sink_3, &sink_4]);

            let bar = if let Some(bar) = keep_bar.take() {
                bar
            } else {
                sixteen_bars_melody(&mut melod_oct, &sink_1, scale, [pitch; 4])
            };

            bar.gen_notes(&sink_1);

            if rng.gen_range(0..2) == 0 {
                println!("Keep!");
                keep_bar = Some(bar);
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
fn gen_chord(oct: &mut u8, semi: Semitones, chords: &[Semitones], sinks: [&Sink; 3]) -> Pitch {
    let mut rng = thread_rng();
    *oct = (((*oct as i8) + rng.gen_range(-1..=1)) as u8).max(3).min(5);
    let note: Note = Note { pitch: Pitch::C, octave: *oct } + semi;

    for (st, sink) in chords.iter().zip(sinks.iter()) {
        if rng.gen::<u8>() < 64u8 {
            continue;
        }
        let note = note + *st;

        let length_opts = &[
            Lengths::Quarter,
            Lengths::Half,
            Lengths::Whole,
        ];
        let x: usize = rng.gen_range(0..length_opts.len());
        let length = length_opts[x];

        let bar = patterns::driving_bar(length, &[note]);
        bar.gen_notes(sink);
        // let freq = note.freq_f32() as u32;

        // let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(2.0)).amplify(0.10).buffered().low_pass(10000);
        // // println!("{:?} {:?} {:?}", note, 3, freq);
        // sink.append(source);
    }

    note.pitch
}

#[derive(Debug, Copy, Clone)]
pub struct BeatFract {
    num: usize,
    den: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct BarMember {
    fract: BeatFract,
    note: Option<Note>,
}

#[derive(Debug, Clone)]
pub struct Bar {
    notes: Vec<BarMember>,
}

impl Bar {
    pub fn gen_notes(&self, sink: &Sink) {
        // TODO: non constant bpm
        let bps = 2.0;

        for note in self.notes.iter() {
            let freq = note.note.map(|n| n.freq_f32()).unwrap_or(1.0) as u32;
            let dur = (bps * (note.fract.num as f32)) / (note.fract.den as f32);
            println!("dur: {:?}", dur);
            let source = SineWave::new(freq)
                .take_duration(Duration::from_secs_f32(
                    dur
                ))
                .amplify(0.20)
                .buffered()
                .low_pass(10000);

            sink.append(source);
        }
    }
}

fn sixteen_bars_melody(oct: &mut u8, sink: &Sink, scale: &[Semitones], pitches: [Pitch; 4]) -> Bar {
    let mut rng = rand::thread_rng();
    let mut notes = vec![];

    for _ in 0..rng.gen_range(1..5) {
        *oct = (((*oct as i8) + rng.gen_range(-1..=1)) as u8).max(2).min(6);
        let semi = scale.choose(&mut rng).unwrap();
        let note = Note { pitch: pitches[0], octave: *oct } + *semi;
        notes.push(note);
    }


    let length_opts = &[
        Lengths::Eighth,
        Lengths::EighthTriplets,
        Lengths::Quarter,
        Lengths::QuarterTriplets,
        Lengths::HalfTriplets,
        Lengths::Half,
    ];
    let x: usize = rng.gen_range(0..length_opts.len());
    let length = length_opts[x];

    patterns::driving_bar(length, &notes)

    // // for quarter_bar in 0..4 {
    // for quarter_bar in 0..1 {
    //     let mut gen_notes = |ct, len, pitch| {
    //         for _beat in 0..ct {
    //             let semi = scale.choose(&mut rng).unwrap();

    //             let note = Note { pitch, octave: *oct } + *semi;

    //             let freq = note.freq_f32() as u32;
    //             let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(len)).amplify(0.20).buffered().low_pass(10000);
    //             sink.append(source);
    //         }
    //     };

    //     let gen_rest = |len| {
    //         let source = SineWave::new(1).take_duration(Duration::from_secs_f32(len)).amplify(0.00001).buffered().low_pass(10000);
    //         sink.append(source);
    //     };

    //     for bar in 0..1 {
    //         let mut rng = rand::thread_rng();
    //         let pitch = pitches[(quarter_bar * 4) + bar];

    //         let mut taken = 0;
    //         'fill: while taken < 4 {
    //             let take = rng.gen_range(1..=4);
    //             let mode = rng.gen_range(0..14);

    //             // 0. rest(N)
    //             // 1. eighth
    //             // 2. eighth-triplets
    //             // 3. quarter
    //             // 4. quarter-triplets
    //             // 5. half
    //             // 6. half-triplets
    //             // 7. whole
    //             match take {
    //                 1 if mode <= 3 => {
    //                     match mode {
    //                         1 => gen_notes(2, 2.0 / 8.0, pitch),
    //                         2 => gen_notes(3, 0.1666, pitch),
    //                         3 => gen_notes(1, 2.0 / 4.0, pitch),
    //                         _ => gen_rest(2.0 / 4.0),
    //                     }
    //                     taken += 1;
    //                 }
    //                 2 if mode <= 5 => {
    //                     match mode {
    //                         1 => gen_notes(4, 2.0 / 8.0, pitch),
    //                         2 => gen_notes(6, 0.1666, pitch),
    //                         3 => gen_notes(2, 2.0 / 4.0, pitch),
    //                         4 => gen_notes(3, 0.3333, pitch),
    //                         5 => gen_notes(1, 2.0 / 2.0, pitch),
    //                         _ => gen_rest(2.0 * 2.0 / 4.0),
    //                     }
    //                     taken += 2;
    //                 }
    //                 4 => {
    //                     match mode {
    //                         1 => gen_notes(8, 2.0 / 8.0, pitch),
    //                         2 => gen_notes(12, 0.1666, pitch),
    //                         3 => gen_notes(4, 2.0 / 4.0, pitch),
    //                         4 => gen_notes(6, 0.3333, pitch),
    //                         5 => gen_notes(2, 2.0 / 2.0, pitch),
    //                         6 => gen_notes(3, 0.6666, pitch),
    //                         7 => gen_notes(1, 2.0, pitch),
    //                         _ => gen_rest(4.0 * 2.0 / 4.0),
    //                     }
    //                     taken += 4;
    //                 }
    //                 _ => continue 'fill,
    //             }
    //         }
    //     }
    // }
}

#[derive(Debug, Copy, Clone)]
pub enum Lengths {
    Sixteenth,
    SixteenthTriplets,
    Eighth,
    EighthTriplets,
    Quarter,
    QuarterTriplets,
    Half,
    HalfTriplets,
    Whole,
}

impl Lengths {
    pub fn get_fraction(&self) -> BeatFract {
        match self {
            Lengths::Sixteenth => BeatFract { num: 1, den: 16 },
            Lengths::SixteenthTriplets => BeatFract { num: 1, den: 24 },
            Lengths::Eighth => BeatFract { num: 1, den: 8 },
            Lengths::EighthTriplets => BeatFract { num: 1, den: 12 },
            Lengths::Quarter => BeatFract { num: 1, den: 4 },
            Lengths::QuarterTriplets => BeatFract { num: 1, den: 6 },
            Lengths::Half => BeatFract { num: 1, den: 2 },
            Lengths::HalfTriplets => BeatFract { num: 1, den: 3 },
            Lengths::Whole => BeatFract { num: 1, den: 1 },
        }
    }
}

pub mod patterns {
    use super::*;
    pub fn driving_bar(length: Lengths, notes: &[Note]) -> Bar {
        let mut bar = Bar { notes: vec![] };

        let fract = length.get_fraction();

        let (ct, mem) = match length {
            Lengths::Sixteenth => (16, BarMember { fract, note: None }),
            Lengths::SixteenthTriplets => (24, BarMember { fract, note: None }),
            Lengths::Eighth => (8, BarMember { fract, note: None }),
            Lengths::EighthTriplets => (12, BarMember { fract, note: None }),
            Lengths::Quarter => (4, BarMember { fract, note: None }),
            Lengths::QuarterTriplets => (6, BarMember { fract, note: None }),
            Lengths::Half => (2, BarMember { fract, note: None }),
            Lengths::HalfTriplets => (3, BarMember { fract, note: None }),
            Lengths::Whole => (1, BarMember { fract, note: None }),

        };

        let notecy = notes.iter().cycle();

        for (_, note) in (0..ct).zip(notecy) {
            let mut mem = mem.clone();
            mem.note = Some(*note);
            bar.notes.push(mem);
        }

        bar
    }
}
