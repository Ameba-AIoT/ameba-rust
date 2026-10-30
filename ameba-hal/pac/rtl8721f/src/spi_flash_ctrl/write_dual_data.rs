#[doc = "Register `WRITE_DUAL_DATA` reader"]
pub type R = crate::R<WriteDualDataSpec>;
#[doc = "Register `WRITE_DUAL_DATA` writer"]
pub type W = crate::W<WriteDualDataSpec>;
#[doc = "Field `WR_DUAL_I_CMD` reader - Indicates SPI Flash command value of dual data write command."]
pub type WrDualICmdR = crate::FieldReader;
#[doc = "Field `WR_DUAL_I_CMD` writer - Indicates SPI Flash command value of dual data write command."]
pub type WrDualICmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual data write command."]
    #[inline(always)]
    pub fn wr_dual_i_cmd(&self) -> WrDualICmdR {
        WrDualICmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual data write command."]
    #[inline(always)]
    pub fn wr_dual_i_cmd(&mut self) -> WrDualICmdW<'_, WriteDualDataSpec> {
        WrDualICmdW::new(self, 0)
    }
}
#[doc = "Dual Data Input Program Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`write_dual_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_dual_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteDualDataSpec;
impl crate::RegisterSpec for WriteDualDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_dual_data::R`](R) reader structure"]
impl crate::Readable for WriteDualDataSpec {}
#[doc = "`write(|w| ..)` method takes [`write_dual_data::W`](W) writer structure"]
impl crate::Writable for WriteDualDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_DUAL_DATA to value 0"]
impl crate::Resettable for WriteDualDataSpec {}
