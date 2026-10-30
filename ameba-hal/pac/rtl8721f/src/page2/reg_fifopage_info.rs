#[doc = "Register `REG_FIFOPAGE_INFO` reader"]
pub type R = crate::R<RegFifopageInfoSpec>;
#[doc = "Field `BIT_HPQ_AVAL_PG` reader - Available High Priority Queue Page Number"]
pub type BitHpqAvalPgR = crate::FieldReader;
#[doc = "Field `BIT_LPQ_AVAL_PG` reader - Available Low Priority Queue Page Number"]
pub type BitLpqAvalPgR = crate::FieldReader;
#[doc = "Field `BIT_PUBQ_AVAL_PG` reader - Available Public Queue Page Number"]
pub type BitPubqAvalPgR = crate::FieldReader;
#[doc = "Field `BIT_TXPKTNUM` reader - Packet number in TXPKTBUF (Normal/Low)"]
pub type BitTxpktnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Available High Priority Queue Page Number"]
    #[inline(always)]
    pub fn bit_hpq_aval_pg(&self) -> BitHpqAvalPgR {
        BitHpqAvalPgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Available Low Priority Queue Page Number"]
    #[inline(always)]
    pub fn bit_lpq_aval_pg(&self) -> BitLpqAvalPgR {
        BitLpqAvalPgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Available Public Queue Page Number"]
    #[inline(always)]
    pub fn bit_pubq_aval_pg(&self) -> BitPubqAvalPgR {
        BitPubqAvalPgR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Packet number in TXPKTBUF (Normal/Low)"]
    #[inline(always)]
    pub fn bit_txpktnum(&self) -> BitTxpktnumR {
        BitTxpktnumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fifopage_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFifopageInfoSpec;
impl crate::RegisterSpec for RegFifopageInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fifopage_info::R`](R) reader structure"]
impl crate::Readable for RegFifopageInfoSpec {}
#[doc = "`reset()` method sets REG_FIFOPAGE_INFO to value 0"]
impl crate::Resettable for RegFifopageInfoSpec {}
