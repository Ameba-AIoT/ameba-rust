#[doc = "Register `SR_TX_BCLK` reader"]
pub type R = crate::R<SrTxBclkSpec>;
#[doc = "Register `SR_TX_BCLK` writer"]
pub type W = crate::W<SrTxBclkSpec>;
#[doc = "Field `tx_mi` reader - "]
pub type TxMiR = crate::FieldReader<u16>;
#[doc = "Field `tx_mi` writer - "]
pub type TxMiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tx_ni` reader - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
pub type TxNiR = crate::FieldReader<u16>;
#[doc = "Field `tx_ni` writer - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
pub type TxNiW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `tx_mi_ni_update` reader - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
pub type TxMiNiUpdateR = crate::BitReader;
#[doc = "Field `tx_mi_ni_update` writer - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
pub type TxMiNiUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tx_mi(&self) -> TxMiR {
        TxMiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
    #[inline(always)]
    pub fn tx_ni(&self) -> TxNiR {
        TxNiR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
    #[inline(always)]
    pub fn tx_mi_ni_update(&self) -> TxMiNiUpdateR {
        TxMiNiUpdateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tx_mi(&mut self) -> TxMiW<'_, SrTxBclkSpec> {
        TxMiW::new(self, 0)
    }
    #[doc = "Bits 16:30 - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
    #[inline(always)]
    pub fn tx_ni(&mut self) -> TxNiW<'_, SrTxBclkSpec> {
        TxNiW::new(self, 16)
    }
    #[doc = "Bit 31 - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
    #[inline(always)]
    pub fn tx_mi_ni_update(&mut self) -> TxMiNiUpdateW<'_, SrTxBclkSpec> {
        TxMiNiUpdateW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_tx_bclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_tx_bclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrTxBclkSpec;
impl crate::RegisterSpec for SrTxBclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_tx_bclk::R`](R) reader structure"]
impl crate::Readable for SrTxBclkSpec {}
#[doc = "`write(|w| ..)` method takes [`sr_tx_bclk::W`](W) writer structure"]
impl crate::Writable for SrTxBclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR_TX_BCLK to value 0x0030_0271"]
impl crate::Resettable for SrTxBclkSpec {
    const RESET_VALUE: u32 = 0x0030_0271;
}
