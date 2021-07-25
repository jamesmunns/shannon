pub mod scale;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink_1 = Sink::try_new(&stream_handle).unwrap();
    let sink_2 = Sink::try_new(&stream_handle).unwrap();
    let sink_3 = Sink::try_new(&stream_handle).unwrap();
    let sink_4 = Sink::try_new(&stream_handle).unwrap();

    std::thread::sleep(Duration::from_secs(1));

    // for oct in [3, 5, 4].iter() {
    //     for st in scale::AEOLIAN_INTERVALS {
    //         let note = scale::Note {
    //             pitch: scale::Pitch::ASharp,
    //             octave: *oct,
    //         };
    //         let note = note + *st;

    //         // Add a dummy source of the sake of the example.
    //         let oct = if st.0 == 12 {
    //             1
    //         } else {
    //             0
    //         } + *oct;

    //         let freq = note.freq_f32() as u32;

    //         let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
    //         println!("{:?} {:?} {:?}", note, oct, freq);
    //         sink.append(source);

    //         // The sound plays in a separate thread. This call will block the current thread until the sink
    //         // has finished playing all its queued sounds.
    //         sink.sleep_until_end();
    //     }

    //     for st in scale::HARMONIC_MINOR_INTERVALS {
    //         let note = scale::Note {
    //             pitch: scale::Pitch::ASharp,
    //             octave: *oct,
    //         };
    //         let note = note + *st;

    //         // Add a dummy source of the sake of the example.
    //         let oct = if st.0 == 12 {
    //             1
    //         } else {
    //             0
    //         } + *oct;

    //         let freq = note.freq_f32() as u32;

    //         let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
    //         println!("{:?} {:?} {:?}", note, oct, freq);
    //         sink.append(source);

    //         // The sound plays in a separate thread. This call will block the current thread until the sink
    //         // has finished playing all its queued sounds.
    //         sink.sleep_until_end();
    //     }

    //     for st in scale::MELODIC_MINOR_ASCENDING_INTERVALS {
    //         let note = scale::Note {
    //             pitch: scale::Pitch::ASharp,
    //             octave: *oct,
    //         };
    //         let note = note + *st;

    //         // Add a dummy source of the sake of the example.
    //         let oct = if st.0 == 12 {
    //             1
    //         } else {
    //             0
    //         } + *oct;

    //         let freq = note.freq_f32() as u32;

    //         let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
    //         println!("{:?} {:?} {:?}", note, oct, freq);
    //         sink.append(source);

    //         // The sound plays in a separate thread. This call will block the current thread until the sink
    //         // has finished playing all its queued sounds.
    //         sink.sleep_until_end();
    //     }

    //     for st in scale::MELODIC_MINOR_DESCENDING_INTERVALS {
    //         let note = scale::Note {
    //             pitch: scale::Pitch::ASharp,
    //             octave: *oct,
    //         };
    //         let note = note + *st;

    //         // Add a dummy source of the sake of the example.
    //         let oct = if st.0 == 12 {
    //             1
    //         } else {
    //             0
    //         } + *oct;

    //         let freq = note.freq_f32() as u32;

    //         let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
    //         println!("{:?} {:?} {:?}", note, oct, freq);
    //         sink.append(source);

    //         // The sound plays in a separate thread. This call will block the current thread until the sink
    //         // has finished playing all its queued sounds.
    //         sink.sleep_until_end();
    //     }
    //     println!("=-=-=-=-=-=-=-=-=")
    // }


    for note_semi in scale::PHRYGIAN_INTERVALS {
        let note = scale::Note {
            pitch: scale::Pitch::C,
            octave: 4,
        };
        let note = note + *note_semi;

        for (st, sink) in scale::MAJOR_TRIAD_INTERVALS.iter().zip(&[&sink_2, &sink_3, &sink_4]) {
            let note = note + *st;

            let freq = note.freq_f32() as u32;

            let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(1.0)).amplify(0.30);
            println!("{:?} {:?} {:?}", note, 3, freq);
            sink.append(source);
        }

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        for sink in &[&sink_2, &sink_3, &sink_4] {
            sink.sleep_until_end();
        }
    }




    std::thread::sleep(Duration::from_secs(1));
}


// Okay, what do I need to do?
//
// * Set up the actors so that they can generate music
//     * Probably need some kind of function that can understand scales, keys
// * Set up the director to choose certain settings, like key, etc.
