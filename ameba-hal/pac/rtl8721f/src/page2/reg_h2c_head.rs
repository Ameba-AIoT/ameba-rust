#[doc = "Register `REG_H2C_HEAD` reader"]
pub type R = crate::R<RegH2cHeadSpec>;
#[doc = "Register `REG_H2C_HEAD` writer"]
pub type W = crate::W<RegH2cHeadSpec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegH2cHeadSpec> {
        RsvdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_h2c_head::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_h2c_head::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegH2cHeadSpec;
impl crate::RegisterSpec for RegH2cHeadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_h2c_head::R`](R) reader structure"]
impl crate::Readable for RegH2cHeadSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_h2c_head::W`](W) writer structure"]
impl crate::Writable for RegH2cHeadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_H2C_HEAD to value 0"]
impl crate::Resettable for RegH2cHeadSpec {}
