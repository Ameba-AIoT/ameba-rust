#[doc = "Register `REG_BACAM_WD` reader"]
pub type R = crate::R<RegBacamWdSpec>;
#[doc = "Register `REG_BACAM_WD` writer"]
pub type W = crate::W<RegBacamWdSpec>;
#[doc = "Field `BIT_BA_CONTENT_L` reader - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the lower 32 bits of CAM content."]
pub type BitBaContentLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BA_CONTENT_L` writer - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the lower 32 bits of CAM content."]
pub type BitBaContentLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the lower 32 bits of CAM content."]
    #[inline(always)]
    pub fn bit_ba_content_l(&self) -> BitBaContentLR {
        BitBaContentLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the lower 32 bits of CAM content."]
    #[inline(always)]
    pub fn bit_ba_content_l(&mut self) -> BitBaContentLW<'_, RegBacamWdSpec> {
        BitBaContentLW::new(self, 0)
    }
}
#[doc = "Block ACK CAM Content Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam_wd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam_wd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacamWdSpec;
impl crate::RegisterSpec for RegBacamWdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam_wd::R`](R) reader structure"]
impl crate::Readable for RegBacamWdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam_wd::W`](W) writer structure"]
impl crate::Writable for RegBacamWdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM_WD to value 0"]
impl crate::Resettable for RegBacamWdSpec {}
