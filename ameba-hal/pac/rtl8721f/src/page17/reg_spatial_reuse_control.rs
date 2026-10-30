#[doc = "Register `REG_SPATIAL_REUSE_CONTROL` reader"]
pub type R = crate::R<RegSpatialReuseControlSpec>;
#[doc = "Register `REG_SPATIAL_REUSE_CONTROL` writer"]
pub type W = crate::W<RegSpatialReuseControlSpec>;
#[doc = "Field `BIT_TXPWR_FAST_MODE_EN` reader - fast mode enable for txpwr_ctrl,tie 1 for AMEBAD2"]
pub type BitTxpwrFastModeEnR = crate::BitReader;
#[doc = "Field `BIT_TXPWR_FAST_MODE_EN` writer - fast mode enable for txpwr_ctrl,tie 1 for AMEBAD2"]
pub type BitTxpwrFastModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXPWR_CH20_DATA` reader - indicate which 20M channel is punctured in RU"]
pub type BitTxpwrCh20DataR = crate::FieldReader;
#[doc = "Field `BIT_TXPWR_CH20_DATA` writer - indicate which 20M channel is punctured in RU"]
pub type BitTxpwrCh20DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - fast mode enable for txpwr_ctrl,tie 1 for AMEBAD2"]
    #[inline(always)]
    pub fn bit_txpwr_fast_mode_en(&self) -> BitTxpwrFastModeEnR {
        BitTxpwrFastModeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - indicate which 20M channel is punctured in RU"]
    #[inline(always)]
    pub fn bit_txpwr_ch20_data(&self) -> BitTxpwrCh20DataR {
        BitTxpwrCh20DataR::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - fast mode enable for txpwr_ctrl,tie 1 for AMEBAD2"]
    #[inline(always)]
    pub fn bit_txpwr_fast_mode_en(
        &mut self,
    ) -> BitTxpwrFastModeEnW<'_, RegSpatialReuseControlSpec> {
        BitTxpwrFastModeEnW::new(self, 0)
    }
    #[doc = "Bits 1:8 - indicate which 20M channel is punctured in RU"]
    #[inline(always)]
    pub fn bit_txpwr_ch20_data(&mut self) -> BitTxpwrCh20DataW<'_, RegSpatialReuseControlSpec> {
        BitTxpwrCh20DataW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spatial_reuse_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spatial_reuse_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpatialReuseControlSpec;
impl crate::RegisterSpec for RegSpatialReuseControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spatial_reuse_control::R`](R) reader structure"]
impl crate::Readable for RegSpatialReuseControlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spatial_reuse_control::W`](W) writer structure"]
impl crate::Writable for RegSpatialReuseControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPATIAL_REUSE_CONTROL to value 0x01"]
impl crate::Resettable for RegSpatialReuseControlSpec {
    const RESET_VALUE: u32 = 0x01;
}
