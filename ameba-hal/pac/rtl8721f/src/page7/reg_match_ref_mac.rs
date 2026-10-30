#[doc = "Register `REG_MATCH_REF_MAC` reader"]
pub type R = crate::R<RegMatchRefMacSpec>;
#[doc = "Register `REG_MATCH_REF_MAC` writer"]
pub type W = crate::W<RegMatchRefMacSpec>;
#[doc = "Field `BIT_MATCH_REF_MAC` reader - used for compare with debug port"]
pub type BitMatchRefMacR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MATCH_REF_MAC` writer - used for compare with debug port"]
pub type BitMatchRefMacW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - used for compare with debug port"]
    #[inline(always)]
    pub fn bit_match_ref_mac(&self) -> BitMatchRefMacR {
        BitMatchRefMacR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used for compare with debug port"]
    #[inline(always)]
    pub fn bit_match_ref_mac(&mut self) -> BitMatchRefMacW<'_, RegMatchRefMacSpec> {
        BitMatchRefMacW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_match_ref_mac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_match_ref_mac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMatchRefMacSpec;
impl crate::RegisterSpec for RegMatchRefMacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_match_ref_mac::R`](R) reader structure"]
impl crate::Readable for RegMatchRefMacSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_match_ref_mac::W`](W) writer structure"]
impl crate::Writable for RegMatchRefMacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MATCH_REF_MAC to value 0"]
impl crate::Resettable for RegMatchRefMacSpec {}
