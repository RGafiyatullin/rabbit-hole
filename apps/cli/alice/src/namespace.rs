use crate::curve::Curve;

#[derive(Debug, Clone, Default)]
pub struct Namespace;

impl Namespace {
    pub fn dkg_csi_rashi_own_share(&self, curve: Curve) -> String {
        format!("dkg/csi-rashi/{}", curve)
    }

    pub fn key_share_s4(&self) -> String {
        "key-share/s4".to_owned()
    }

    pub fn key_share_s4_for_curve(&self, curve: Curve) -> String {
        format!("{}/{}", self.key_share_s4(), curve)
    }

    pub fn tss_frost_nonce(&self, curve: Curve) -> String {
        format!("tss/frost/nonce/{}", curve)
    }
}
