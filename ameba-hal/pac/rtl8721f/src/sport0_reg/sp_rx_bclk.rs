#[doc = "Register `SP_RX_BCLK` reader"]
pub type R = crate::R<SpRxBclkSpec>;
#[doc = "Register `SP_RX_BCLK` writer"]
pub type W = crate::W<SpRxBclkSpec>;
#[doc = "Field `rx_mi` reader - "]
pub type RxMiR = crate::FieldReader<u16>;
#[doc = "Field `rx_mi` writer - "]
pub type RxMiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rx_ni` reader - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
pub type RxNiR = crate::FieldReader<u16>;
#[doc = "Field `rx_ni` writer - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
pub type RxNiW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `rx_mi_ni_update` reader - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
pub type RxMiNiUpdateR = crate::BitReader;
#[doc = "Field `rx_mi_ni_update` writer - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
pub type RxMiNiUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_mi(&self) -> RxMiR {
        RxMiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
    #[inline(always)]
    pub fn rx_ni(&self) -> RxNiR {
        RxNiR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
    #[inline(always)]
    pub fn rx_mi_ni_update(&self) -> RxMiNiUpdateR {
        RxMiNiUpdateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_mi(&mut self) -> RxMiW<'_, SpRxBclkSpec> {
        RxMiW::new(self, 0)
    }
    #[doc = "Bits 16:30 - BCLK = 40MHz*(ni/mi) For example: BCLK=3.072MHz=40MHz*(48/625)"]
    #[inline(always)]
    pub fn rx_ni(&mut self) -> RxNiW<'_, SpRxBclkSpec> {
        RxNiW::new(self, 16)
    }
    #[doc = "Bit 31 - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
    #[inline(always)]
    pub fn rx_mi_ni_update(&mut self) -> RxMiNiUpdateW<'_, SpRxBclkSpec> {
        RxMiNiUpdateW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_rx_bclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_rx_bclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpRxBclkSpec;
impl crate::RegisterSpec for SpRxBclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_rx_bclk::R`](R) reader structure"]
impl crate::Readable for SpRxBclkSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_rx_bclk::W`](W) writer structure"]
impl crate::Writable for SpRxBclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_RX_BCLK to value 0x0030_0271"]
impl crate::Resettable for SpRxBclkSpec {
    const RESET_VALUE: u32 = 0x0030_0271;
}
