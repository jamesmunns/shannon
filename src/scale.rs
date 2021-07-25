use core::ops::Add;

pub const PITCHES_PER_OCTAVE: u32 = 12;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Pitch {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Add<Semitones> for Note {
    type Output = Note;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Semitones) -> Self::Output {
        let pitch: u8 = self.pitch.into();
        let pitch: u32 = pitch.into();
        let pitch = pitch.wrapping_add(rhs.0);

        let new_oct = self.octave + (pitch / PITCHES_PER_OCTAVE) as u8;
        let new_pitch = (pitch % PITCHES_PER_OCTAVE) as u8;
        Note {
            pitch: new_pitch.into(),
            octave: new_oct,
        }
    }
}

impl From<Pitch> for u8 {
    fn from(val: Pitch) -> Self {
        match val {
            Pitch::C => 0,
            Pitch::CSharp => 1,
            Pitch::D => 2,
            Pitch::DSharp => 3,
            Pitch::E => 4,
            Pitch::F => 5,
            Pitch::FSharp => 6,
            Pitch::G => 7,
            Pitch::GSharp => 8,
            Pitch::A => 9,
            Pitch::ASharp => 10,
            Pitch::B => 11,
        }
    }
}

impl From<u8> for Pitch {
    fn from(val: u8) -> Self {
        match val {
             0 => Pitch::C,
             1 => Pitch::CSharp,
             2 => Pitch::D,
             3 => Pitch::DSharp,
             4 => Pitch::E,
             5 => Pitch::F,
             6 => Pitch::FSharp,
             7 => Pitch::G,
             8 => Pitch::GSharp,
             9 => Pitch::A,
             10 => Pitch::ASharp,
             11 => Pitch::B,
             _ => {
                debug_assert!(false, "what?");
                // lol
                Pitch::C
            }
        }
    }
}

impl Pitch {
    // Note: frequencies taken from
    // https://pages.mtu.edu/~suits/notefreqs.html
    pub const fn root_frequency(&self) -> f32 {
        match self {
            Pitch::C => 16.35,
            Pitch::CSharp => 17.32,
            Pitch::D => 18.35,
            Pitch::DSharp => 19.45,
            Pitch::E => 20.60,
            Pitch::F => 21.83,
            Pitch::FSharp => 23.12,
            Pitch::G => 24.50,
            Pitch::GSharp => 25.96,
            Pitch::A => 27.50,
            Pitch::ASharp => 29.14,
            Pitch::B => 30.87,
        }
    }

    pub fn freq_with_octave(&self, octave: u8) -> f32 {
        let base = self.root_frequency();
        let mult = f32::from(octave).exp2();
        base * mult
    }
}

/// A note.
#[derive(Debug, Clone, Copy)]
pub struct Note {
    /// The pitch of the note (A, B, C#, etc).
    pub pitch: Pitch,
    /// The octave of the note in standard notation.
    pub octave: u8,
}

impl Note {
    pub fn freq_f32(&self) -> f32 {
        self.pitch.freq_with_octave(self.octave)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Semitones(pub u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check_octave() {
        let tests = [
            (Pitch::C, 1, 32.70),
            (Pitch::C, 4, 261.63),
            (Pitch::C, 8, 4186.01),
            (Pitch::A, 1, 55.00),
            (Pitch::A, 3, 220.00),
            (Pitch::A, 7, 3520.00),
        ];

        for (note, octave, exp_freq) in tests {
            let freq = note.freq_with_octave(octave);
            f32_compare(freq, exp_freq, exp_freq * 0.001);
        }
    }

    #[test]
    fn sanity_check_semitone() {
        let tests = [
            (Pitch::C, Semitones(0), Pitch::C),
            (Pitch::C, Semitones(12), Pitch::C),
            (Pitch::C, Semitones(1), Pitch::CSharp),
            (Pitch::C, Semitones(3), Pitch::DSharp),
        ];

        for (note, semis, exp_note) in tests {
            let new_note = note + semis;
            assert_eq!(new_note, exp_note);
        }
    }

    fn f32_compare(lhs: f32, rhs: f32, tol: f32) {
        let abs_diff = (rhs - lhs).abs();
        if abs_diff > tol.abs() {
            panic!(
                "Value out of tolerance! lhs: {} rhs: {} diff: {} tol: {}",
                lhs,
                rhs,
                abs_diff,
                tol,
            );
        }
    }
}

// --------------------------
// Diatonic Scale Sequences
//
// REF: https://en.wikipedia.org/wiki/Diatonic_scale#Theory
// --------------------------
pub const IONIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),  // 1
    Semitones(2),
    Semitones(4),  // 3
    Semitones(5),
    Semitones(7),  // 5
    Semitones(9),
    Semitones(11), // 7
    Semitones(12),
];

pub const DORIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(2),
    Semitones(3),
    Semitones(5),
    Semitones(7),
    Semitones(9),
    Semitones(10),
    Semitones(12),
];

pub const PHRYGIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(1),
    Semitones(3),
    Semitones(5),
    Semitones(7),
    Semitones(8),
    Semitones(10),
    Semitones(12),
];

pub const LYDIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(2),
    Semitones(4),
    Semitones(6),
    Semitones(7),
    Semitones(9),
    Semitones(11),
    Semitones(12),
];

pub const MIXOLYDIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(2),
    Semitones(4),
    Semitones(5),
    Semitones(7),
    Semitones(9),
    Semitones(10),
    Semitones(12),
];

pub const AEOLIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(2),
    Semitones(3),
    Semitones(5),
    Semitones(7),
    Semitones(8),
    Semitones(10),
    Semitones(12),
];

pub const LOCRIAN_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(1),
    Semitones(3),
    Semitones(5),
    Semitones(6),
    Semitones(8),
    Semitones(10),
    Semitones(12),
];

// --------------------------
// Other Scale Sequences
// --------------------------

pub const NATURAL_MAJOR_INTERVALS: &[Semitones] = IONIAN_INTERVALS;
pub const NATURAL_MINOR_INTERVALS: &[Semitones] = AEOLIAN_INTERVALS;

pub const HARMONIC_MINOR_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(2),
    Semitones(3),
    Semitones(5),
    Semitones(7),
    Semitones(8),
    Semitones(11),
    Semitones(12),
];

pub const MELODIC_MINOR_ASCENDING_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(2),
    Semitones(3),
    Semitones(5),
    Semitones(7),
    Semitones(9),
    Semitones(11),
    Semitones(12),
];

pub const MELODIC_MINOR_DESCENDING_INTERVALS: &[Semitones] = &[
    Semitones(12),
    Semitones(10),
    Semitones(8),
    Semitones(7),
    Semitones(5),
    Semitones(3),
    Semitones(2),
    Semitones(0),
];

// --------------------------
// Chord sequences
//
// NOTE:
//   https://www.bellandcomusic.com/chord-structure.html
//   https://www.bellandcomusic.com/music-chords.html
// --------------------------

// Triads

pub const MAJOR_TRIAD_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(4),
    Semitones(7),
];

pub const MINOR_TRIAD_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(3),
    Semitones(7),
];

pub const DIMINISHED_TRIAD_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(3),
    Semitones(6),
];

pub const AUGMENTED_TRIAD_INTERVALS: &[Semitones] = &[
    Semitones(0),
    Semitones(4),
    Semitones(8),
];

// Tetrads

pub const DOMINANT_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const MINOR_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const MAJOR_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const MINOR_MAJOR_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const AUGMENTED_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const AUGMENTED_MAJOR_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const DIMINISHED_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];

pub const DIMINISHED_HALF_TETRAD_INTERVALS: &[Semitones] = &[
    // TODO
];
