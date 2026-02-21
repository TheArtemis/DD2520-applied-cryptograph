use rand::RngCore;

/// Security parameter k (bits) for wire labels.
pub const LABEL_BITS: usize = 128;
pub const LABEL_BYTES: usize = LABEL_BITS / 8;

/// A label for a single Boolean value on a wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WireLabel(pub [u8; LABEL_BYTES]);

impl WireLabel {
    pub fn random<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        let mut bytes = [0u8; LABEL_BYTES];
        rng.fill_bytes(&mut bytes);
        WireLabel(bytes)
    }
}

/// The pair of labels representing 0 and 1 on a wire.
#[derive(Clone, Debug)]
pub struct WireLabels {
    pub zero: WireLabel,
    pub one: WireLabel,
}

impl WireLabels {
    pub fn random<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        Self {
            zero: WireLabel::random(rng),
            one: WireLabel::random(rng),
        }
    }
}

