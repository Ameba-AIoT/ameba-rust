#[doc = "Register `REG_LPSD` reader"]
pub type R = crate::R<RegLpsdSpec>;
#[doc = "Register `REG_LPSD` writer"]
pub type W = crate::W<RegLpsdSpec>;
#[doc = "Field `POW_SD` reader - Power on signal detector"]
pub type PowSdR = crate::BitReader;
#[doc = "Field `POW_SD` writer - Power on signal detector"]
pub type PowSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_POSEDGE` reader - 1: vin>vref output high"]
pub type SdPosedgeR = crate::BitReader;
#[doc = "Field `SD_POSEDGE` writer - 1: vin>vref output high"]
pub type SdPosedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_VREFH` reader - adjust VREFH"]
pub type SelVrefhR = crate::FieldReader;
#[doc = "Field `SEL_VREFL` reader - adjust VREFL"]
pub type SelVreflR = crate::FieldReader;
#[doc = "Field `EN_VREF0_H` reader - set VREFH to 0"]
pub type EnVref0HR = crate::BitReader;
#[doc = "Field `EN_VREF0_H` writer - set VREFH to 0"]
pub type EnVref0HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_VREF0_L` reader - set VREFL to 0"]
pub type EnVref0LR = crate::BitReader;
#[doc = "Field `EN_VREF0_L` writer - set VREFL to 0"]
pub type EnVref0LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPSD_DMY` reader - dummy"]
pub type LpsdDmyR = crate::FieldReader;
#[doc = "Field `LPSD_DMY` writer - dummy"]
pub type LpsdDmyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Power on signal detector"]
    #[inline(always)]
    pub fn pow_sd(&self) -> PowSdR {
        PowSdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: vin>vref output high"]
    #[inline(always)]
    pub fn sd_posedge(&self) -> SdPosedgeR {
        SdPosedgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - adjust VREFH"]
    #[inline(always)]
    pub fn sel_vrefh(&self) -> SelVrefhR {
        SelVrefhR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - adjust VREFL"]
    #[inline(always)]
    pub fn sel_vrefl(&self) -> SelVreflR {
        SelVreflR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - set VREFH to 0"]
    #[inline(always)]
    pub fn en_vref0_h(&self) -> EnVref0HR {
        EnVref0HR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - set VREFL to 0"]
    #[inline(always)]
    pub fn en_vref0_l(&self) -> EnVref0LR {
        EnVref0LR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:19 - dummy"]
    #[inline(always)]
    pub fn lpsd_dmy(&self) -> LpsdDmyR {
        LpsdDmyR::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power on signal detector"]
    #[inline(always)]
    pub fn pow_sd(&mut self) -> PowSdW<'_, RegLpsdSpec> {
        PowSdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: vin>vref output high"]
    #[inline(always)]
    pub fn sd_posedge(&mut self) -> SdPosedgeW<'_, RegLpsdSpec> {
        SdPosedgeW::new(self, 1)
    }
    #[doc = "Bit 12 - set VREFH to 0"]
    #[inline(always)]
    pub fn en_vref0_h(&mut self) -> EnVref0HW<'_, RegLpsdSpec> {
        EnVref0HW::new(self, 12)
    }
    #[doc = "Bit 13 - set VREFL to 0"]
    #[inline(always)]
    pub fn en_vref0_l(&mut self) -> EnVref0LW<'_, RegLpsdSpec> {
        EnVref0LW::new(self, 13)
    }
    #[doc = "Bits 14:19 - dummy"]
    #[inline(always)]
    pub fn lpsd_dmy(&mut self) -> LpsdDmyW<'_, RegLpsdSpec> {
        LpsdDmyW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpsd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpsd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpsdSpec;
impl crate::RegisterSpec for RegLpsdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpsd::R`](R) reader structure"]
impl crate::Readable for RegLpsdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpsd::W`](W) writer structure"]
impl crate::Writable for RegLpsdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPSD to value 0"]
impl crate::Resettable for RegLpsdSpec {}
