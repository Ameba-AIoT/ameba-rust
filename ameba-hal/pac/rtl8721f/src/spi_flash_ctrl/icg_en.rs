#[doc = "Register `ICG_EN` reader"]
pub type R = crate::R<IcgEnSpec>;
#[doc = "Register `ICG_EN` writer"]
pub type W = crate::W<IcgEnSpec>;
#[doc = "Field `CGATE_EN_BOOT` reader - Boot module clock gating enable"]
pub type CgateEnBootR = crate::BitReader;
#[doc = "Field `CGATE_EN_BOOT` writer - Boot module clock gating enable"]
pub type CgateEnBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_NATIVE` reader - Native chop module clock gating enable"]
pub type CgateEnNativeR = crate::BitReader;
#[doc = "Field `CGATE_EN_NATIVE` writer - Native chop module clock gating enable"]
pub type CgateEnNativeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_USWR` reader - User write module clock gating enable"]
pub type CgateEnUswrR = crate::BitReader;
#[doc = "Field `CGATE_EN_USWR` writer - User write module clock gating enable"]
pub type CgateEnUswrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_USRD` reader - User read module clock gating enable"]
pub type CgateEnUsrdR = crate::BitReader;
#[doc = "Field `CGATE_EN_USRD` writer - User read module clock gating enable"]
pub type CgateEnUsrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_ATWR` reader - Auto write module clock gating enable"]
pub type CgateEnAtwrR = crate::BitReader;
#[doc = "Field `CGATE_EN_ATWR` writer - Auto write module clock gating enable"]
pub type CgateEnAtwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_ATRD` reader - Auto read module clock gating enable"]
pub type CgateEnAtrdR = crate::BitReader;
#[doc = "Field `CGATE_EN_ATRD` writer - Auto read module clock gating enable"]
pub type CgateEnAtrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_FRQC` reader - frqc module clock gating enable"]
pub type CgateEnFrqcR = crate::BitReader;
#[doc = "Field `CGATE_EN_FRQC` writer - frqc module clock gating enable"]
pub type CgateEnFrqcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_ATCK` reader - atck module clock gating enable"]
pub type CgateEnAtckR = crate::BitReader;
#[doc = "Field `CGATE_EN_ATCK` writer - atck module clock gating enable"]
pub type CgateEnAtckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_PDEX` reader - pdex module clock gating enable"]
pub type CgateEnPdexR = crate::BitReader;
#[doc = "Field `CGATE_EN_PDEX` writer - pdex module clock gating enable"]
pub type CgateEnPdexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_BPGM` reader - bpgm module clock gating enable"]
pub type CgateEnBpgmR = crate::BitReader;
#[doc = "Field `CGATE_EN_BPGM` writer - bpgm module clock gating enable"]
pub type CgateEnBpgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_BPGM_CR` reader - bpgm_cr module clock gating enable"]
pub type CgateEnBpgmCrR = crate::BitReader;
#[doc = "Field `CGATE_EN_BPGM_CR` writer - bpgm_cr module clock gating enable"]
pub type CgateEnBpgmCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_WRCR` reader - Write cr module clock gating enable"]
pub type CgateEnWrcrR = crate::BitReader;
#[doc = "Field `CGATE_EN_WRCR` writer - Write cr module clock gating enable"]
pub type CgateEnWrcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGATE_EN_ECC` reader - Nand ECC controller module clock gating enable. Exists: TAG >= 2.0.11"]
pub type CgateEnEccR = crate::BitReader;
#[doc = "Field `CGATE_EN_ECC` writer - Nand ECC controller module clock gating enable. Exists: TAG >= 2.0.11"]
pub type CgateEnEccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Boot module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_boot(&self) -> CgateEnBootR {
        CgateEnBootR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Native chop module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_native(&self) -> CgateEnNativeR {
        CgateEnNativeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User write module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_uswr(&self) -> CgateEnUswrR {
        CgateEnUswrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User read module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_usrd(&self) -> CgateEnUsrdR {
        CgateEnUsrdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto write module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_atwr(&self) -> CgateEnAtwrR {
        CgateEnAtwrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto read module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_atrd(&self) -> CgateEnAtrdR {
        CgateEnAtrdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - frqc module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_frqc(&self) -> CgateEnFrqcR {
        CgateEnFrqcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - atck module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_atck(&self) -> CgateEnAtckR {
        CgateEnAtckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pdex module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_pdex(&self) -> CgateEnPdexR {
        CgateEnPdexR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - bpgm module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_bpgm(&self) -> CgateEnBpgmR {
        CgateEnBpgmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - bpgm_cr module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_bpgm_cr(&self) -> CgateEnBpgmCrR {
        CgateEnBpgmCrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write cr module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_wrcr(&self) -> CgateEnWrcrR {
        CgateEnWrcrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Nand ECC controller module clock gating enable. Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn cgate_en_ecc(&self) -> CgateEnEccR {
        CgateEnEccR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_boot(&mut self) -> CgateEnBootW<'_, IcgEnSpec> {
        CgateEnBootW::new(self, 0)
    }
    #[doc = "Bit 1 - Native chop module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_native(&mut self) -> CgateEnNativeW<'_, IcgEnSpec> {
        CgateEnNativeW::new(self, 1)
    }
    #[doc = "Bit 2 - User write module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_uswr(&mut self) -> CgateEnUswrW<'_, IcgEnSpec> {
        CgateEnUswrW::new(self, 2)
    }
    #[doc = "Bit 3 - User read module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_usrd(&mut self) -> CgateEnUsrdW<'_, IcgEnSpec> {
        CgateEnUsrdW::new(self, 3)
    }
    #[doc = "Bit 4 - Auto write module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_atwr(&mut self) -> CgateEnAtwrW<'_, IcgEnSpec> {
        CgateEnAtwrW::new(self, 4)
    }
    #[doc = "Bit 5 - Auto read module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_atrd(&mut self) -> CgateEnAtrdW<'_, IcgEnSpec> {
        CgateEnAtrdW::new(self, 5)
    }
    #[doc = "Bit 6 - frqc module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_frqc(&mut self) -> CgateEnFrqcW<'_, IcgEnSpec> {
        CgateEnFrqcW::new(self, 6)
    }
    #[doc = "Bit 7 - atck module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_atck(&mut self) -> CgateEnAtckW<'_, IcgEnSpec> {
        CgateEnAtckW::new(self, 7)
    }
    #[doc = "Bit 8 - pdex module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_pdex(&mut self) -> CgateEnPdexW<'_, IcgEnSpec> {
        CgateEnPdexW::new(self, 8)
    }
    #[doc = "Bit 9 - bpgm module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_bpgm(&mut self) -> CgateEnBpgmW<'_, IcgEnSpec> {
        CgateEnBpgmW::new(self, 9)
    }
    #[doc = "Bit 10 - bpgm_cr module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_bpgm_cr(&mut self) -> CgateEnBpgmCrW<'_, IcgEnSpec> {
        CgateEnBpgmCrW::new(self, 10)
    }
    #[doc = "Bit 11 - Write cr module clock gating enable"]
    #[inline(always)]
    pub fn cgate_en_wrcr(&mut self) -> CgateEnWrcrW<'_, IcgEnSpec> {
        CgateEnWrcrW::new(self, 11)
    }
    #[doc = "Bit 12 - Nand ECC controller module clock gating enable. Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn cgate_en_ecc(&mut self) -> CgateEnEccW<'_, IcgEnSpec> {
        CgateEnEccW::new(self, 12)
    }
}
#[doc = "Clock gating enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icg_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icg_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcgEnSpec;
impl crate::RegisterSpec for IcgEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg_en::R`](R) reader structure"]
impl crate::Readable for IcgEnSpec {}
#[doc = "`write(|w| ..)` method takes [`icg_en::W`](W) writer structure"]
impl crate::Writable for IcgEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICG_EN to value 0x1fff"]
impl crate::Resettable for IcgEnSpec {
    const RESET_VALUE: u32 = 0x1fff;
}
