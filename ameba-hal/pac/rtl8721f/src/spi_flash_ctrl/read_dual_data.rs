#[doc = "Register `READ_DUAL_DATA` reader"]
pub type R = crate::R<ReadDualDataSpec>;
#[doc = "Register `READ_DUAL_DATA` writer"]
pub type W = crate::W<ReadDualDataSpec>;
#[doc = "Field `RD_DUAL_O_CMD` reader - Indicates SPI Flash command value of dual data read command."]
pub type RdDualOCmdR = crate::FieldReader;
#[doc = "Field `RD_DUAL_O_CMD` writer - Indicates SPI Flash command value of dual data read command."]
pub type RdDualOCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual data read command."]
    #[inline(always)]
    pub fn rd_dual_o_cmd(&self) -> RdDualOCmdR {
        RdDualOCmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual data read command."]
    #[inline(always)]
    pub fn rd_dual_o_cmd(&mut self) -> RdDualOCmdW<'_, ReadDualDataSpec> {
        RdDualOCmdW::new(self, 0)
    }
}
#[doc = "Dual Output Read Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`read_dual_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_dual_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadDualDataSpec;
impl crate::RegisterSpec for ReadDualDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_dual_data::R`](R) reader structure"]
impl crate::Readable for ReadDualDataSpec {}
#[doc = "`write(|w| ..)` method takes [`read_dual_data::W`](W) writer structure"]
impl crate::Writable for ReadDualDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READ_DUAL_DATA to value 0"]
impl crate::Resettable for ReadDualDataSpec {}
