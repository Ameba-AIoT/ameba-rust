#[doc = "Register `REG_SEQUENCE_CHECK0` reader"]
pub type R = crate::R<RegSequenceCheck0Spec>;
#[doc = "Register `REG_SEQUENCE_CHECK0` writer"]
pub type W = crate::W<RegSequenceCheck0Spec>;
#[doc = "Field `r_seq_chk0` reader - sequence check, when this field first write 0x69696969, then write 0x96969696, tirgger led/i2c work"]
pub type RSeqChk0R = crate::FieldReader<u32>;
#[doc = "Field `r_seq_chk0` writer - sequence check, when this field first write 0x69696969, then write 0x96969696, tirgger led/i2c work"]
pub type RSeqChk0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sequence check, when this field first write 0x69696969, then write 0x96969696, tirgger led/i2c work"]
    #[inline(always)]
    pub fn r_seq_chk0(&self) -> RSeqChk0R {
        RSeqChk0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sequence check, when this field first write 0x69696969, then write 0x96969696, tirgger led/i2c work"]
    #[inline(always)]
    pub fn r_seq_chk0(&mut self) -> RSeqChk0W<'_, RegSequenceCheck0Spec> {
        RSeqChk0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sequence_check0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sequence_check0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSequenceCheck0Spec;
impl crate::RegisterSpec for RegSequenceCheck0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sequence_check0::R`](R) reader structure"]
impl crate::Readable for RegSequenceCheck0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sequence_check0::W`](W) writer structure"]
impl crate::Writable for RegSequenceCheck0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SEQUENCE_CHECK0 to value 0"]
impl crate::Resettable for RegSequenceCheck0Spec {}
