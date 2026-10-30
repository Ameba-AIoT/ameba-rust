#[doc = "Register `REG_CKD_GRP0` reader"]
pub type R = crate::R<RegCkdGrp0Spec>;
#[doc = "Register `REG_CKD_GRP0` writer"]
pub type W = crate::W<RegCkdGrp0Spec>;
#[doc = "Field `ckd_rom` reader - rom clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
pub type CkdRomR = crate::FieldReader;
#[doc = "Field `ckd_rom` writer - rom clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
pub type CkdRomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ckd_sram` reader - sram clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
pub type CkdSramR = crate::FieldReader;
#[doc = "Field `ckd_sram` writer - sram clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
pub type CkdSramW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ckd_xtal_adc` reader - xtal 4m divider for adc Default is /10 which output 4M"]
pub type CkdXtalAdcR = crate::FieldReader;
#[doc = "Field `ckd_xtal_adc` writer - xtal 4m divider for adc Default is /10 which output 4M"]
pub type CkdXtalAdcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - rom clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_rom(&self) -> CkdRomR {
        CkdRomR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - sram clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sram(&self) -> CkdSramR {
        CkdSramR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - xtal 4m divider for adc Default is /10 which output 4M"]
    #[inline(always)]
    pub fn ckd_xtal_adc(&self) -> CkdXtalAdcR {
        CkdXtalAdcR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - rom clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_rom(&mut self) -> CkdRomW<'_, RegCkdGrp0Spec> {
        CkdRomW::new(self, 0)
    }
    #[doc = "Bits 4:5 - sram clock divider, it is based on hp_clk 0: div1 1: div2 2: div3 3/other: divider by this value + 1 DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sram(&mut self) -> CkdSramW<'_, RegCkdGrp0Spec> {
        CkdSramW::new(self, 4)
    }
    #[doc = "Bits 8:11 - xtal 4m divider for adc Default is /10 which output 4M"]
    #[inline(always)]
    pub fn ckd_xtal_adc(&mut self) -> CkdXtalAdcW<'_, RegCkdGrp0Spec> {
        CkdXtalAdcW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdGrp0Spec;
impl crate::RegisterSpec for RegCkdGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_grp0::R`](R) reader structure"]
impl crate::Readable for RegCkdGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_grp0::W`](W) writer structure"]
impl crate::Writable for RegCkdGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_GRP0 to value 0x0900"]
impl crate::Resettable for RegCkdGrp0Spec {
    const RESET_VALUE: u32 = 0x0900;
}
