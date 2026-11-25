use clip_api::ClipEngine;

#[cfg(test)]
const FERRO_SHAPE: f32 = 0.2;
#[cfg(test)]
const FERRO_CHARACTER: f32 = 0.1;
#[cfg(test)]
const RAIL_SHAPE: f32 = 0.65;
#[cfg(test)]
const RAIL_CHARACTER: f32 = 0.6;

pub const ENGINES: &[(&str, fn() -> Box<dyn ClipEngine>)] = &[("Ferro", ferro), ("Rail", rail)];

fn flush(value: f32) -> f32 {
    if value.abs() < 1e-25 {
        0.0
    } else {
        value
    }
}

fn core_mean(current: f32, previous: f32, scale: f32) -> f32 {
    let current_norm = current.hypot(scale);
    let previous_norm = previous.hypot(scale);
    scale * (current + previous) / (current_norm + previous_norm)
}

struct Core {
    previous: f32,
    allpass: f32,
    held: f32,
    release: f32,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            previous: 0.0,
            allpass: 0.0,
            held: 0.0,
            release: 0.0,
        }
    }
}

impl Core {
    fn prepare(&mut self, sample_rate: f32) {
        self.release = (-1.0 / (0.0005 * sample_rate)).exp();
        self.reset();
    }

    fn reset(&mut self) {
        self.previous = 0.0;
        self.allpass = 0.0;
        self.held = 0.0;
    }

    fn process(&mut self, input: f32, scale: f32) -> f32 {
        let mean = core_mean(input, self.previous, scale);
        let average = 0.5 * (input + self.previous);
        let allpass = input / 3.0 + self.previous - self.allpass / 3.0;
        self.allpass = flush(allpass);
        let compression = 1.0 - scale / input.hypot(scale);
        self.held = compression.max(self.held * self.release);
        if self.held < 1e-20 {
            self.held = 0.0;
        }
        let held2 = self.held * self.held;
        let held4 = held2 * held2;
        let held8 = held4 * held4;
        let weight = held8 / (held8 + 0.3f32.powi(8));
        self.previous = input;
        flush(mean + (1.0 - weight) * (allpass - average))
    }
}

trait Analog: Send + Default {
    type Params: Copy;
    const NAME: &'static str;

    fn prepare(&mut self, sample_rate: f32);
    fn reset(&mut self);
    fn parameters(&self, shape: f32, character: f32) -> Self::Params;
    fn process(&mut self, input: f32, params: Self::Params) -> f32;
}

struct AnalogStage<A: Analog> {
    models: [A; 2],
    ceiling: f32,
    shape: f32,
    character: f32,
    current_ceiling: f32,
    current_shape: f32,
    current_character: f32,
    smoothing: f32,
    primed: bool,
}

impl<A: Analog> AnalogStage<A> {
    fn new() -> Self {
        Self {
            models: std::array::from_fn(|_| A::default()),
            ceiling: 1.0,
            shape: 0.5,
            character: 0.5,
            current_ceiling: 1.0,
            current_shape: 0.5,
            current_character: 0.5,
            smoothing: 1.0,
            primed: false,
        }
    }
}

impl<A: Analog + 'static> ClipEngine for AnalogStage<A> {
    fn name(&self) -> &'static str {
        A::NAME
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        let sample_rate = if sample_rate.is_finite() {
            sample_rate.max(1.0)
        } else {
            48_000.0
        };
        self.smoothing = 1.0 - (-1.0 / (0.005 * sample_rate)).exp();
        for model in &mut self.models {
            model.prepare(sample_rate);
        }
        self.current_ceiling = self.ceiling;
        self.current_shape = self.shape;
        self.current_character = self.character;
        self.primed = false;
    }

    fn reset(&mut self) {
        for model in &mut self.models {
            model.reset();
        }
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        self.ceiling = ceiling.clamp(0.01, 1.0);
        self.shape = shape.clamp(0.0, 1.0);
        self.character = character.clamp(0.0, 1.0);
        if !self.primed {
            self.current_ceiling = self.ceiling;
            self.current_shape = self.shape;
            self.current_character = self.character;
            self.primed = true;
        }
    }

    fn latency(&self) -> usize {
        0
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let n = left.len().min(right.len());
        for i in 0..n {
            self.current_ceiling += (self.ceiling - self.current_ceiling) * self.smoothing;
            self.current_shape += (self.shape - self.current_shape) * self.smoothing;
            self.current_character += (self.character - self.current_character) * self.smoothing;
            let params = self.models[0].parameters(self.current_shape, self.current_character);
            let inverse_ceiling = self.current_ceiling.recip();
            let y_l = self.models[0].process(left[i] * inverse_ceiling, params);
            let y_r = self.models[1].process(right[i] * inverse_ceiling, params);
            left[i] = (y_l * self.current_ceiling).clamp(-self.ceiling, self.ceiling);
            right[i] = (y_r * self.current_ceiling).clamp(-self.ceiling, self.ceiling);
        }
    }
}

#[derive(Clone, Copy)]
struct FerroParams {
    scale: f32,
    emphasis: f32,
}

struct FerroChannel {
    core: Core,
    low_input: f32,
    low_output: f32,
    lowpass: f32,
}

impl Default for FerroChannel {
    fn default() -> Self {
        Self {
            core: Core::default(),
            low_input: 0.0,
            low_output: 0.0,
            lowpass: 1.0,
        }
    }
}

impl Analog for FerroChannel {
    type Params = FerroParams;
    const NAME: &'static str = "Ferro";

    fn prepare(&mut self, sample_rate: f32) {
        self.lowpass = 1.0 - (-2.0 * std::f32::consts::PI * 1_800.0 / sample_rate).exp();
        self.core.prepare(sample_rate);
        self.reset();
    }

    fn reset(&mut self) {
        self.core.reset();
        self.low_input = 0.0;
        self.low_output = 0.0;
    }

    fn parameters(&self, shape: f32, character: f32) -> Self::Params {
        Self::Params {
            scale: 1.0 - 0.75 * shape,
            emphasis: 2.0 * character,
        }
    }

    fn process(&mut self, input: f32, params: Self::Params) -> f32 {
        let driven = (1.0 + params.emphasis) * input - params.emphasis * self.low_input;
        let clipped = self.core.process(driven, params.scale);
        let output = (clipped + params.emphasis * self.low_output) / (1.0 + params.emphasis);
        self.low_input = flush(self.low_input + self.lowpass * (input - self.low_input));
        self.low_output = flush(self.low_output + self.lowpass * (output - self.low_output));
        output.clamp(-1.0, 1.0)
    }
}

#[derive(Clone, Copy)]
struct RailParams {
    scale: f32,
    slew: f32,
    release: f32,
}

struct RailChannel {
    core: Core,
    sample_rate: f32,
    output: f32,
    overload: f32,
}

impl Default for RailChannel {
    fn default() -> Self {
        Self {
            core: Core::default(),
            sample_rate: 48_000.0,
            output: 0.0,
            overload: 0.0,
        }
    }
}

impl Analog for RailChannel {
    type Params = RailParams;
    const NAME: &'static str = "Rail";

    fn prepare(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.core.prepare(sample_rate);
        self.reset();
    }

    fn reset(&mut self) {
        self.core.reset();
        self.output = 0.0;
        self.overload = 0.0;
    }

    fn parameters(&self, shape: f32, character: f32) -> Self::Params {
        let slew_hz = 8_000.0 + 72_000.0 * character;
        let release_seconds = 0.03 - 0.028 * character;
        Self::Params {
            scale: 1.0 - 0.65 * shape,
            slew: 2.0 * std::f32::consts::PI * slew_hz / self.sample_rate,
            release: (-1.0 / (self.sample_rate * release_seconds)).exp(),
        }
    }

    fn process(&mut self, input: f32, params: Self::Params) -> f32 {
        let excess = (input.abs() - 1.0).clamp(0.0, 10_000.0);
        self.overload = excess.max(self.overload * params.release);
        if self.overload < 1e-20 {
            self.overload = 0.0;
        }
        let driven = input / (1.0 + 0.18 * self.overload);
        let target = self.core.process(driven, params.scale);
        let step = (target - self.output).clamp(-params.slew, params.slew);
        self.output = flush((self.output + step).clamp(-1.0, 1.0));
        self.output
    }
}

fn ferro() -> Box<dyn ClipEngine> {
    Box::new(AnalogStage::<FerroChannel>::new())
}

fn rail() -> Box<dyn ClipEngine> {
    Box::new(AnalogStage::<RailChannel>::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clip_api::bench;

    fn make(index: usize) -> Box<dyn ClipEngine> {
        ENGINES[index].1()
    }

    #[test]
    fn ferro_contract() {
        bench::assert_contract(&ferro);
    }

    #[test]
    fn rail_contract() {
        bench::assert_contract(&rail);
    }

    #[test]
    fn export_labels_and_latency_are_valid() {
        assert!((1..=3).contains(&ENGINES.len()));
        for (label, make) in ENGINES {
            assert!(!label.is_empty() && label.len() <= 12);
            let mut engine = make();
            engine.prepare(48_000.0, 512);
            assert_eq!(engine.name(), *label);
            assert_eq!(engine.latency(), 0);
        }
    }

    #[test]
    fn core_interval_mean_matches_its_antiderivative() {
        for scale in [0.25, 0.6, 1.0] {
            for current in [-4.0, -0.2, 0.0, 0.2, 4.0] {
                let repeated = core_mean(current, current, scale);
                let curve = current / (1.0 + (current / scale).powi(2)).sqrt();
                assert!((repeated - curve).abs() < 2e-6);
                for previous in [-3.0, -0.1, 0.0, 0.3, 3.0] {
                    let actual = core_mean(current, previous, scale) as f64;
                    let difference = (current - previous) as f64;
                    let expected = if difference == 0.0 {
                        curve as f64
                    } else {
                        let antiderivative = |x: f32| scale as f64 * x.hypot(scale) as f64;
                        (antiderivative(current) - antiderivative(previous)) / difference
                    };
                    assert!(
                        (actual - expected).abs() < 2e-5,
                        "mean {actual} vs {expected}, scale {scale}, {previous} -> {current}"
                    );
                }
            }
        }
    }

    #[test]
    fn ferro_is_odd_and_bounded() {
        for shape in [0.0, FERRO_SHAPE, 1.0] {
            for character in [0.0, FERRO_CHARACTER, 1.0] {
                let reference = FerroChannel::default();
                let params = reference.parameters(shape, character);
                let mut positive = FerroChannel::default();
                let mut negative = FerroChannel::default();
                positive.prepare(48_000.0);
                negative.prepare(48_000.0);
                for i in 0..4_096 {
                    let x = 4.0 * (0.13 * i as f32).sin();
                    let y = positive.process(x, params);
                    let opposite = negative.process(-x, params);
                    assert!(y.is_finite() && y.abs() <= 1.0);
                    assert!((y + opposite).abs() < 1e-6, "asymmetry at sample {i}");
                }
            }
        }
    }

    #[test]
    fn ferro_character_adds_frequency_dependent_saturation() {
        let rms = |frequency: f32, character: f32| {
            let mut channel = FerroChannel::default();
            channel.prepare(48_000.0);
            let params = channel.parameters(0.8, character);
            let mut power = 0.0;
            for i in 0..8_192 {
                let input =
                    1.5 * (2.0 * std::f32::consts::PI * frequency * i as f32 / 48_000.0).sin();
                let output = channel.process(input, params);
                if i >= 4_096 {
                    power += output * output;
                }
            }
            (power / 4_096.0).sqrt()
        };
        let low_plain = rms(100.0, 0.0);
        let low_emphasised = rms(100.0, 1.0);
        let high_plain = rms(10_000.0, 0.0);
        let high_emphasised = rms(10_000.0, 1.0);
        assert!((low_emphasised / low_plain - 1.0).abs() < 0.05);
        assert!(
            high_emphasised < 0.95 * high_plain,
            "high-frequency RMS {high_plain} -> {high_emphasised}; low {low_plain} -> {low_emphasised}"
        );
    }

    #[test]
    fn rail_limits_slew_and_recovers_from_overload() {
        let mut slew = RailChannel::default();
        slew.prepare(48_000.0);
        let slow = slew.parameters(0.5, 0.0);
        for _ in 0..100 {
            slew.process(4.0, slow);
        }
        let before = slew.output;
        let after = slew.process(-4.0, slow);
        assert!((before - after) <= slow.slew + 1e-6);
        assert!(before - after < 1.9, "unlimited polarity flip");

        let mut recovery = RailChannel::default();
        recovery.prepare(48_000.0);
        let fast = recovery.parameters(0.5, 1.0);
        for _ in 0..100 {
            recovery.process(4.0, fast);
        }
        assert!(recovery.overload > 0.0);
        for _ in 0..4_800 {
            recovery.process(0.0, fast);
        }
        assert_eq!(recovery.output, 0.0);
        assert_eq!(recovery.overload, 0.0);
    }

    #[test]
    fn small_signal_response_is_flat() {
        for (index, recommended) in [
            (0, (FERRO_SHAPE, FERRO_CHARACTER)),
            (1, (RAIL_SHAPE, RAIL_CHARACTER)),
        ] {
            for settings in [recommended, (0.0, 0.0), (1.0, 1.0)] {
                for sample_rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0] {
                    let mut engine = make(index);
                    engine.prepare(sample_rate, 512);
                    engine.set(1.0, settings.0, settings.1);
                    let error = bench::flatness_db(engine.as_mut(), sample_rate as f64);
                    assert!(
                        error <= 0.05,
                        "{} at {sample_rate} Hz, {settings:?}: {error} dB",
                        engine.name()
                    );
                }
            }
        }
    }

    #[test]
    fn recommended_alias_beats_reference_by_ten_db() {
        for (index, settings) in [
            (0, (FERRO_SHAPE, FERRO_CHARACTER)),
            (1, (RAIL_SHAPE, RAIL_CHARACTER)),
        ] {
            let mut engine = make(index);
            engine.prepare(48_000.0, 512);
            engine.set(1.0, settings.0, settings.1);
            for hz in [1_000.0, 2_500.0, 5_000.0, 8_000.0, 12_000.0] {
                let alias = bench::alias_dbc(engine.as_mut(), 48_000.0, hz, 12.0);
                assert!(alias <= -22.7, "{} at {hz} Hz: {alias} dBc", engine.name());
            }
        }
    }

    #[test]
    fn output_is_symmetric_and_has_no_dc() {
        for index in 0..ENGINES.len() {
            let mut engine = make(index);
            engine.prepare(48_000.0, 256);
            engine.set(1.0, 0.5, 0.5);
            let mut left = vec![0.0; 9_600];
            let mut right = vec![0.0; 9_600];
            for i in 0..left.len() {
                left[i] = 2.0 * (2.0 * std::f32::consts::PI * 1_000.0 * i as f32 / 48_000.0).sin();
                right[i] = -left[i];
            }
            engine.process(&mut left, &mut right);
            for (&l, &r) in left.iter().zip(&right) {
                assert!((l + r).abs() < 2e-5, "{}: channel asymmetry", engine.name());
            }
            let dc_window = &left[1_024..1_024 + 7_200];
            let mean = dc_window.iter().sum::<f32>() / dc_window.len() as f32;
            assert!(mean.abs() < 1e-5, "{}: DC {mean}", engine.name());
        }
    }

    #[test]
    fn sample_ceiling_holds_during_settings_changes() {
        for index in 0..ENGINES.len() {
            let mut engine = make(index);
            engine.prepare(48_000.0, 64);
            for block in 0..96 {
                let ceiling = if block == 48 { 0.25 } else { 0.75 };
                let setting = block as f32 / 95.0;
                engine.set(ceiling, setting, 1.0 - setting);
                let mut left = vec![0.0; 64];
                let mut right = vec![0.0; 64];
                for i in 0..64 {
                    let t = (block * 64 + i) as f32;
                    left[i] = if (t as usize / 9) % 2 == 0 { 8.0 } else { -8.0 };
                    right[i] = -left[i];
                }
                engine.process(&mut left, &mut right);
                assert!(left.iter().chain(&right).all(|v| v.abs() <= ceiling + 1e-6));
            }
        }
    }

    #[test]
    fn macro_changes_are_smooth_on_a_held_signal() {
        for index in 0..ENGINES.len() {
            let mut engine = make(index);
            engine.prepare(48_000.0, 64);
            engine.set(1.0, 0.0, 0.0);
            let mut left = vec![0.5; 1_024];
            let mut right = left.clone();
            engine.process(&mut left, &mut right);
            engine.set(1.0, 1.0, 1.0);
            let mut left = vec![0.5; 256];
            let mut right = left.clone();
            engine.process(&mut left, &mut right);
            let largest = left
                .windows(2)
                .map(|pair| (pair[1] - pair[0]).abs())
                .fold(0.0f32, f32::max);
            assert!(
                largest < 0.002,
                "{}: parameter step {largest}",
                engine.name()
            );
        }
    }

    #[test]
    #[ignore]
    fn report_all() {
        for (index, (label, make)) in ENGINES.iter().enumerate() {
            let recommended = if index == 0 {
                (FERRO_SHAPE, FERRO_CHARACTER)
            } else {
                (RAIL_SHAPE, RAIL_CHARACTER)
            };
            println!("Engine {label}: recommended settings");
            println!("{}", bench::report(make, recommended.0, recommended.1));
            println!("Engine {label}: zero settings");
            println!("{}", bench::report(make, 0.0, 0.0));
            println!("Engine {label}: full settings");
            println!("{}", bench::report(make, 1.0, 1.0));
        }
    }
}
