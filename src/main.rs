pub mod scale;

use std::os::unix::thread;
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

#[derive(Debug, Copy, Clone)]
enum FourPattern {
    AAAA,
    AABB,
    ABAB,
    ABAC,
    AAAB,
}

impl FourPattern {
    fn indexes(&self) -> [usize; 4] {
        match self {
            FourPattern::AAAA => [0, 0, 0, 0],
            FourPattern::AABB => [0, 0, 1, 1],
            FourPattern::ABAB => [0, 1, 0, 1],
            FourPattern::ABAC => [0, 1, 0, 2],
            FourPattern::AAAB => [0, 0, 0, 1],
        }
    }
}

const PATTERNS: &[FourPattern] = &[
    FourPattern::AAAA,
    FourPattern::AABB,
    FourPattern::ABAB,
    FourPattern::ABAC,
    FourPattern::AAAB,
];

fn sixteen_bars_chords(
    chords: [(Semitones, &[Semitones]); 3],
    oct: &mut u8,
    sinks: [&Sink; 3],
) -> [Pitch; 16] {
    let mut rng = thread_rng();

    // Hokay, so.
    //
    // Steps here are to:
    //
    // 1. Figure out the MACRO level patterns, which arrange four-bar sets,
    //     which we will need three of.
    let outer_patterns = PATTERNS.choose(&mut rng).unwrap();

    let inner_patterns = [
        PATTERNS.choose(&mut rng).unwrap(),
        PATTERNS.choose(&mut rng).unwrap(),
        PATTERNS.choose(&mut rng).unwrap(),
    ];

    // * Walk through each bar. In each bar we will need to:
    //     * Determine which MACRO level pattern we are in, which will be one of three
    //         patterns we will use


    let mut pitches = [Pitch::C; 16];

    println!("chrd: outer {:?}", outer_patterns);
    for (inner_pattern, outer_idx) in outer_patterns.indexes().iter().zip(0..4) {
        // now we need to generate JUST four bars
        let inpat = inner_patterns[*inner_pattern];
        println!("chrd: inner {:?}", inpat);
        for (inner_idx, pat) in inpat.indexes().iter().enumerate() {
            pitches[(outer_idx * 4) + inner_idx] = gen_chord(
                oct,
                chords[*pat].0,
                chords[*pat].1, // &[Semitones],
                sinks, // [&Sink; 3]
            );
        }
    }

    pitches
}

fn sixteen_bars_melody(oct: &mut u8, scale: &[Semitones], pitch: &[Pitch]) -> [Bar; 16] {
    let mut rng = thread_rng();
    let mut output = [EMPTY_BAR; 16];

    let outer_patterns = PATTERNS.choose(&mut rng).unwrap();

    let inner_patterns = [
        PATTERNS.choose(&mut rng).unwrap(),
        PATTERNS.choose(&mut rng).unwrap(),
        PATTERNS.choose(&mut rng).unwrap(),
    ];

    let mut oct_semitoner = || {
        let mut notes = vec![];

        for _ in 0..rng.gen_range(1..5) {
            *oct = (((*oct as i8) + rng.gen_range(-1..=1)) as u8).max(2).min(6);
            let semi = scale.choose(&mut rng).unwrap();
            notes.push((*oct, semi));
        }

        notes
    };

    let semis = [
        oct_semitoner(),
        oct_semitoner(),
        oct_semitoner(),
    ];

    let length_opts = &[
        Lengths::Eighth,
        Lengths::EighthTriplets,
        Lengths::Quarter,
        Lengths::QuarterTriplets,
        Lengths::HalfTriplets,
        Lengths::Half,
    ];

    let lengths = [
        length_opts.choose(&mut rng).unwrap(),
        length_opts.choose(&mut rng).unwrap(),
        length_opts.choose(&mut rng).unwrap(),
    ];

    // * Walk through each bar. In each bar we will need to:
    //     * Determine which MACRO level pattern we are in, which will be one of three
    //         patterns we will use

    println!("mel outer: {:?}", outer_patterns);

    for (inner_pattern, outer_idx) in outer_patterns.indexes().iter().zip(0..4) {
        // now we need to generate JUST four bars
        let inpat = inner_patterns[*inner_pattern];
        println!("mel inner: {:?}", inpat);
        for (inner_idx, pat) in inpat.indexes().iter().enumerate() {

            let idx = (outer_idx * 4) + inner_idx;

            let mut notes = vec![];

            for (oct, semi) in semis[*pat].iter() {
                let note = Note { pitch: pitch[idx], octave: *oct } + **semi;
                notes.push(note);
            }

            output[idx] = if rng.gen() {
                println!("driving");
                patterns::driving_bar(*lengths[*pat], &notes)
            } else {
                println!("ones_threes");
                patterns::ones_threes(*lengths[*pat], &notes)
            };
        }
    }

    /////////////////



    output
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

    let mut rng = thread_rng();

    // if rng.gen() {
    // if true {
    if false {
        // MINOR
        chord_choices.extend_from_slice(MINOR_PRIMARY_CHORDS);
        chord_weights.push(8);
        for _ in 0..MINOR_PRIMARY_CHORDS.len() - 1 {
            chord_weights.push(4);
        }
        chord_choices.extend_from_slice(MINOR_SECONDARY_CHORDS);
        for _ in 0..MINOR_SECONDARY_CHORDS.len() {
            chord_weights.push(1);
        }

        // MINOR
        let scales = &[
            DORIAN_INTERVALS,
            PHRYGIAN_INTERVALS,
            AEOLIAN_INTERVALS,
            LOCRIAN_INTERVALS,
            HARMONIC_MINOR_INTERVALS,
            MELODIC_MINOR_ASCENDING_INTERVALS,
            MELODIC_MINOR_DESCENDING_INTERVALS,
            MINOR_TRIAD_INTERVALS,
            MINOR_7TH_TETRAD_INTERVALS,
            BLUES_MINOR_PENTATONIC_INTERVALS,
            MINOR_PENTATONIC_INTERVALS,
        ];
        scale_choices.extend_from_slice(scales);
        for _ in 0..scales.len() {
            scale_weights.push(1);
        }
    } else {
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
    }

    let chord_weights_idx = WeightedIndex::new(&chord_weights).unwrap();
    let scale_weights_idx = WeightedIndex::new(&scale_weights).unwrap();

    let mut chord_oct = rng.gen_range(4..6);
    let mut melod_oct = rng.gen_range(3..5);

    loop {
        let scale = scale_choices[scale_weights_idx.sample(&mut rng)];
        println!("Scale!");

        let scales = [
            chord_choices[chord_weights_idx.sample(&mut rng)],
            chord_choices[chord_weights_idx.sample(&mut rng)],
            chord_choices[chord_weights_idx.sample(&mut rng)],
        ];
        // sixteen_bars_chords(
        //     scales,
        //     &mut chord_oct,
        //     [&sink_2, &sink_3, &sink_4]
        // );

        let chord_pitches = sixteen_bars_chords(scales, &mut chord_oct, [&sink_2, &sink_3, &sink_4]);

        let bars = sixteen_bars_melody(&mut melod_oct, scale, &chord_pitches);

        for bar in bars {
            bar.gen_notes(&sink_1);
        }

        for sink in &[&sink_1, &sink_2, &sink_3, &sink_4] {
            sink.sleep_until_end();
        }

        // for _ in 0..4 {
        //     println!("Bar!");
        //     let (pitch, chords) = chord_choices[chord_weights_idx.sample(&mut rng)];
        //     let pitch = gen_chord(&mut chord_oct, pitch, chords, [&sink_2, &sink_3, &sink_4]);

        //     let bar = if let Some(bar) = keep_bar.take() {
        //         bar
        //     } else {
        //         one_bar_melody(&mut melod_oct, &sink_1, scale, [pitch; 4])
        //     };

        //     bar.gen_notes(&sink_1);

        //     if rng.gen_range(0..2) == 0 {
        //         println!("Keep!");
        //         keep_bar = Some(bar);
        //     }

        //     // The sound plays in a separate thread. This call will block the current thread until the sink
        //     // has finished playing all its queued sounds.
        //     for sink in &[&sink_1, &sink_2, &sink_3, &sink_4] {
        //         sink.sleep_until_end();
        //     }
        // }
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

        let bar = if rng.gen() {
            patterns::driving_bar(length, &[note])
        } else {
            patterns::ones_threes(length, &[note])
        };
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

#[derive(Debug, Clone, Default)]
pub struct Bar {
    notes: Vec<BarMember>,
}

const EMPTY_BAR: Bar = Bar { notes: vec![] };

impl Bar {
    pub fn gen_notes(&self, sink: &Sink) {
        // TODO: non constant bpm
        let bps = 2.0;

        for note in self.notes.iter() {
            let freq = note.note.map(|n| n.freq_f32()).unwrap_or(1.0) as u32;
            let dur = (bps * (note.fract.num as f32)) / (note.fract.den as f32);
            println!("dur: {:?}, {}", dur, freq);

            let amp = if freq < 250 {
                0.25
            } else if freq < 500 {
                0.225
            } else if freq < 750 {
                0.20
            } else if freq < 1000 {
                0.175
            } else if freq < 1500 {
                0.15
            } else {
                0.125
            };

            let source = SineWave::new(freq)
                .take_duration(Duration::from_secs_f32(
                    dur
                ))
                .amplify(amp)
                .buffered()
                .low_pass(4000);

            sink.append(source);
        }
    }
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
        let mut rng = thread_rng();
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

        for _ in 0..ct {
            let mut mem = mem.clone();
            mem.note = Some(*notes.choose(&mut rng).unwrap());
            bar.notes.push(mem);
        }

        bar
    }

    pub fn ones_threes(length: Lengths, notes: &[Note]) -> Bar {
        let mut rng = thread_rng();
        let mut bar = Bar { notes: vec![] };

        let fract = length.get_fraction();

        for half in 0..2 {
            match length {
                Lengths::Sixteenth => {
                    let notes_on = rng.gen_range(0..=8);
                    let notes_off = 8 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::SixteenthTriplets => {
                    let notes_on = rng.gen_range(0..=12);
                    let notes_off = 12 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::Eighth => {
                    let notes_on = rng.gen_range(0..=4);
                    let notes_off = 4 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::EighthTriplets => {
                    let notes_on = rng.gen_range(0..=6);
                    let notes_off = 6 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::Quarter => {
                    let notes_on = rng.gen_range(0..=2);
                    let notes_off = 2 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::QuarterTriplets => {
                    let notes_on = rng.gen_range(0..=3);
                    let notes_off = 1 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::Half => {
                    let notes_on = rng.gen_range(0..=1);
                    let notes_off = 1 - notes_on;

                    for _ in 0..notes_on {
                        bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                    }
                    for _ in 0..notes_off {
                        bar.notes.push(BarMember { fract, note: None });
                    }
                }
                Lengths::HalfTriplets if half == 0 => {
                    for _ in 0..3 {
                        bar.notes.push(BarMember {
                            fract, note: Some(*notes.choose(&mut rng).unwrap())
                        });
                    }
                }
                Lengths::Whole if half == 0 => {
                    bar.notes.push(BarMember { fract, note: Some(*notes.choose(&mut rng).unwrap()) });
                }
                _ => {
                    bar.notes.push(BarMember { fract: BeatFract { num: 1, den: 2 }, note: None });
                }
            };
        }

        bar
    }
}
