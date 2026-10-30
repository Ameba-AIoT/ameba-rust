#[doc = "Register `REG_AMPDU_MAX_LENGTH` reader"]
pub type R = crate::R<RegAmpduMaxLengthSpec>;
#[doc = "Register `REG_AMPDU_MAX_LENGTH` writer"]
pub type W = crate::W<RegAmpduMaxLengthSpec>;
#[doc = "Field `BIT_AMPDU_MAX_LENGTH` reader - AMPDU length limit"]
pub type BitAmpduMaxLengthR = crate::FieldReader<u32>;
#[doc = "Field `BIT_AMPDU_MAX_LENGTH` writer - AMPDU length limit"]
pub type BitAmpduMaxLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AMPDU length limit"]
    #[inline(always)]
    pub fn bit_ampdu_max_length(&self) -> BitAmpduMaxLengthR {
        BitAmpduMaxLengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AMPDU length limit"]
    #[inline(always)]
    pub fn bit_ampdu_max_length(&mut self) -> BitAmpduMaxLengthW<'_, RegAmpduMaxLengthSpec> {
        BitAmpduMaxLengthW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ampdu_max_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ampdu_max_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAmpduMaxLengthSpec;
impl crate::RegisterSpec for RegAmpduMaxLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ampdu_max_length::R`](R) reader structure"]
impl crate::Readable for RegAmpduMaxLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ampdu_max_length::W`](W) writer structure"]
impl crate::Writable for RegAmpduMaxLengthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AMPDU_MAX_LENGTH to value 0xffff_ffff"]
impl crate::Resettable for RegAmpduMaxLengthSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
