use borsh::{BorshDeserialize, BorshSerialize};
use group::GroupEncoding;
use std::io::Write;

use crate::transaction::components::sapling::{
    ConvertDescription, OutputDescription, SpendDescription,
};
use crate::{
    sapling::Nullifier,
    transaction::{components::sapling::Authorization, util::*},
};

impl<A: Authorization + PartialEq + BorshSerialize> BorshSerialize for SpendDescription<A>
where
    A::Proof: Clone + BorshSerialize,
    A::AuthSig: BorshSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
        BorshSerialize::serialize(&self.cv.to_bytes(), writer)?;
        BorshSerialize::serialize(&self.anchor.to_bytes(), writer)?;
        BorshSerialize::serialize(&self.nullifier.0, writer)?;
        BorshSerialize::serialize(&self.rk, writer)?;
        BorshSerialize::serialize(&self.zkproof, writer)?;
        BorshSerialize::serialize(&self.spend_auth_sig, writer)
    }
}

impl<A: Authorization + PartialEq + BorshDeserialize> BorshDeserialize for SpendDescription<A>
where
    A::Proof: Clone + BorshDeserialize,
    A::AuthSig: BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
        let cv = deserialize_extended_point(buf)?;
        let anchor = deserialize_scalar(buf)?;
        let nullifier_bytes: [u8; 32] = BorshDeserialize::deserialize(buf)?;
        let nullifier = Nullifier(nullifier_bytes);
        let rk = BorshDeserialize::deserialize(buf)?;
        let zkproof = BorshDeserialize::deserialize(buf)?;
        let spend_auth_sig = BorshDeserialize::deserialize(buf)?;
        Ok(Self {
            cv,
            anchor,
            nullifier,
            rk,
            zkproof,
            spend_auth_sig,
        })
    }
}

impl<Proof: Clone + PartialEq + BorshSerialize> BorshSerialize for ConvertDescription<Proof> {
    fn serialize<W: Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
        BorshSerialize::serialize(&self.cv.to_bytes(), writer)?;
        BorshSerialize::serialize(&self.anchor.to_bytes(), writer)?;
        BorshSerialize::serialize(&self.zkproof, writer)
    }
}

impl<Proof: Clone + PartialEq + BorshDeserialize> BorshDeserialize for ConvertDescription<Proof> {
    fn deserialize(buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
        let cv = deserialize_extended_point(buf)?;
        let anchor = deserialize_scalar(buf)?;
        let zkproof = BorshDeserialize::deserialize(buf)?;
        Ok(Self {
            cv,
            anchor,
            zkproof,
        })
    }
}

impl<Proof: Clone + BorshDeserialize> BorshDeserialize for OutputDescription<Proof> {
    fn deserialize(buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
        let cv = deserialize_extended_point(buf)?;
        let cmu = deserialize_scalar(buf)?;
        let ephemeral_key = BorshDeserialize::deserialize(buf)?;
        let enc_ciphertext = deserialize_array(buf)?;
        let out_ciphertext = deserialize_array(buf)?;
        let zkproof = BorshDeserialize::deserialize(buf)?;
        Ok(Self {
            cv,
            cmu,
            ephemeral_key,
            enc_ciphertext,
            out_ciphertext,
            zkproof,
        })
    }
}

impl<Proof: Clone + BorshSerialize> BorshSerialize for OutputDescription<Proof> {
    fn serialize<W: Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
        BorshSerialize::serialize(&self.cv.to_bytes(), writer)?;
        BorshSerialize::serialize(&self.cmu.to_bytes(), writer)?;
        BorshSerialize::serialize(&self.ephemeral_key, writer)?;
        writer.write_all(self.enc_ciphertext.as_ref())?;
        writer.write_all(self.out_ciphertext.as_ref())?;
        BorshSerialize::serialize(&self.zkproof, writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{
        components::sapling::{
            Authorized, ConvertDescription, OutputDescription, SpendDescription,
        },
        sapling::testing::{
            arb_convert_description, arb_output_description, arb_spend_description,
        },
        GrothProofBytes,
    };
    use borsh::{BorshDeserialize, BorshSerialize};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn spend_description_serialization(spend in arb_spend_description()) {
            // BorshSerialize
            let borsh = spend.try_to_vec().unwrap();
            // BorshDeserialize
            let de_code: SpendDescription<Authorized> = BorshDeserialize::deserialize(&mut borsh.as_ref()).unwrap();
            prop_assert_eq!(spend, de_code);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn output_description_serialization(output in arb_output_description()) {
            // BorshSerialize
            let borsh = output.try_to_vec().unwrap();
            // BorshDeserialize
            let de_code: OutputDescription<GrothProofBytes> = BorshDeserialize::deserialize(&mut borsh.as_ref()).unwrap();
            prop_assert_eq!(output, de_code);
        }
    }
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn convert_description_serialization(convert in arb_convert_description()) {
            // BorshSerialize
            let borsh = convert.try_to_vec().unwrap();
            // BorshDeserialize
            let de_code: ConvertDescription<GrothProofBytes> = BorshDeserialize::deserialize(&mut borsh.as_ref()).unwrap();
            prop_assert_eq!(convert, de_code);
        }
    }

    /*proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn bundle_description_serialization(bundle in arb_bundle()) {
            // BorshSerialize
            let borsh = bundle.try_to_vec().unwrap();
            // BorshDeserialize
            let de_code = BorshDeserialize::deserialize(&mut borsh.as_ref()).unwrap();
            prop_assert_eq!(bundle, de_code);
        }
    }*/
}
