#[doc = "Register `SP_MCLK` reader"]
pub type R = crate::R<SpMclkSpec>;
#[doc = "Register `SP_MCLK` writer"]
pub type W = crate::W<SpMclkSpec>;
#[doc = "Field `mclk_mi` reader - "]
pub type MclkMiR = crate::FieldReader<u16>;
#[doc = "Field `mclk_mi` writer - "]
pub type MclkMiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `mclk_ni` reader - mclk_out = 40MHz*(ni/mi) For example: mclk_out=3.072MHz=40MHz*(48/625)"]
pub type MclkNiR = crate::FieldReader<u16>;
#[doc = "Field `mclk_ni` writer - mclk_out = 40MHz*(ni/mi) For example: mclk_out=3.072MHz=40MHz*(48/625)"]
pub type MclkNiW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `mclk_mi_ni_update` reader - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
pub type MclkMiNiUpdateR = crate::BitReader;
#[doc = "Field `mclk_mi_ni_update` writer - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
pub type MclkMiNiUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mclk_mi(&self) -> MclkMiR {
        MclkMiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - mclk_out = 40MHz*(ni/mi) For example: mclk_out=3.072MHz=40MHz*(48/625)"]
    #[inline(always)]
    pub fn mclk_ni(&self) -> MclkNiR {
        MclkNiR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
    #[inline(always)]
    pub fn mclk_mi_ni_update(&self) -> MclkMiNiUpdateR {
        MclkMiNiUpdateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mclk_mi(&mut self) -> MclkMiW<'_, SpMclkSpec> {
        MclkMiW::new(self, 0)
    }
    #[doc = "Bits 16:30 - mclk_out = 40MHz*(ni/mi) For example: mclk_out=3.072MHz=40MHz*(48/625)"]
    #[inline(always)]
    pub fn mclk_ni(&mut self) -> MclkNiW<'_, SpMclkSpec> {
        MclkNiW::new(self, 16)
    }
    #[doc = "Bit 31 - 1'b1: to update \"mi\" and \"ni\" to get the new clock rate. This bit will be reset automatically when the update is done"]
    #[inline(always)]
    pub fn mclk_mi_ni_update(&mut self) -> MclkMiNiUpdateW<'_, SpMclkSpec> {
        MclkMiNiUpdateW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_mclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_mclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpMclkSpec;
impl crate::RegisterSpec for SpMclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_mclk::R`](R) reader structure"]
impl crate::Readable for SpMclkSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_mclk::W`](W) writer structure"]
impl crate::Writable for SpMclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_MCLK to value 0x00c0_0271"]
impl crate::Resettable for SpMclkSpec {
    const RESET_VALUE: u32 = 0x00c0_0271;
}
