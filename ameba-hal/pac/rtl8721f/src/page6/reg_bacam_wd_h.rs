#[doc = "Register `REG_BACAM_WD_H` reader"]
pub type R = crate::R<RegBacamWdHSpec>;
#[doc = "Register `REG_BACAM_WD_H` writer"]
pub type W = crate::W<RegBacamWdHSpec>;
#[doc = "Field `BIT_BA_CONTENT_H` reader - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the upper 32 bits of CAM content."]
pub type BitBaContentHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BA_CONTENT_H` writer - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the upper 32 bits of CAM content."]
pub type BitBaContentHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the upper 32 bits of CAM content."]
    #[inline(always)]
    pub fn bit_ba_content_h(&self) -> BitBaContentHR {
        BitBaContentHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Used for CAM content read and write access register. Because BA CAM is 64 bit access, the double word (32-bit) responds to the upper 32 bits of CAM content."]
    #[inline(always)]
    pub fn bit_ba_content_h(&mut self) -> BitBaContentHW<'_, RegBacamWdHSpec> {
        BitBaContentHW::new(self, 0)
    }
}
#[doc = "Block ACK CAM Content Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam_wd_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam_wd_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacamWdHSpec;
impl crate::RegisterSpec for RegBacamWdHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam_wd_h::R`](R) reader structure"]
impl crate::Readable for RegBacamWdHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam_wd_h::W`](W) writer structure"]
impl crate::Writable for RegBacamWdHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM_WD_H to value 0"]
impl crate::Resettable for RegBacamWdHSpec {}
