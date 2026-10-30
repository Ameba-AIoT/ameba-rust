#[doc = "Register `REG_TEST_KEY_REG` reader"]
pub type R = crate::R<RegTestKeyRegSpec>;
#[doc = "Register `REG_TEST_KEY_REG` writer"]
pub type W = crate::W<RegTestKeyRegSpec>;
#[doc = "Field `HW_PDB_0P9` reader - // This signal must have the correct default value // Digital sends this signal through an analog levelshift to HW_PDB_33V_OUT // level shift gating to low by POR_AON"]
pub type HwPdb0p9R = crate::BitReader;
#[doc = "Field `HW_PDB_0P9` writer - // This signal must have the correct default value // Digital sends this signal through an analog levelshift to HW_PDB_33V_OUT // level shift gating to low by POR_AON"]
pub type HwPdb0p9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow_sw` reader - power swr"]
pub type PowSwR = crate::BitReader;
#[doc = "Field `pow_sw` writer - power swr"]
pub type PowSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - // This signal must have the correct default value // Digital sends this signal through an analog levelshift to HW_PDB_33V_OUT // level shift gating to low by POR_AON"]
    #[inline(always)]
    pub fn hw_pdb_0p9(&self) -> HwPdb0p9R {
        HwPdb0p9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - power swr"]
    #[inline(always)]
    pub fn pow_sw(&self) -> PowSwR {
        PowSwR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - // This signal must have the correct default value // Digital sends this signal through an analog levelshift to HW_PDB_33V_OUT // level shift gating to low by POR_AON"]
    #[inline(always)]
    pub fn hw_pdb_0p9(&mut self) -> HwPdb0p9W<'_, RegTestKeyRegSpec> {
        HwPdb0p9W::new(self, 0)
    }
    #[doc = "Bit 3 - power swr"]
    #[inline(always)]
    pub fn pow_sw(&mut self) -> PowSwW<'_, RegTestKeyRegSpec> {
        PowSwW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_test_key_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_test_key_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTestKeyRegSpec;
impl crate::RegisterSpec for RegTestKeyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_test_key_reg::R`](R) reader structure"]
impl crate::Readable for RegTestKeyRegSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_test_key_reg::W`](W) writer structure"]
impl crate::Writable for RegTestKeyRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TEST_KEY_REG to value 0x01"]
impl crate::Resettable for RegTestKeyRegSpec {
    const RESET_VALUE: u32 = 0x01;
}
