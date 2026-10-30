#[doc = "Register `TM_OFFSET` reader"]
pub type R = crate::R<TmOffsetSpec>;
#[doc = "Register `TM_OFFSET` writer"]
pub type W = crate::W<TmOffsetSpec>;
#[doc = "Field `tm_b` reader - This bit defines thermal meter offset. * Bit21: Signed bit * Bit\\[20:11\\]: Integer part * Bit\\[10:0\\]: Decimal part"]
pub type TmBR = crate::FieldReader<u32>;
#[doc = "Field `tm_b` writer - This bit defines thermal meter offset. * Bit21: Signed bit * Bit\\[20:11\\]: Integer part * Bit\\[10:0\\]: Decimal part"]
pub type TmBW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - This bit defines thermal meter offset. * Bit21: Signed bit * Bit\\[20:11\\]: Integer part * Bit\\[10:0\\]: Decimal part"]
    #[inline(always)]
    pub fn tm_b(&self) -> TmBR {
        TmBR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - This bit defines thermal meter offset. * Bit21: Signed bit * Bit\\[20:11\\]: Integer part * Bit\\[10:0\\]: Decimal part"]
    #[inline(always)]
    pub fn tm_b(&mut self) -> TmBW<'_, TmOffsetSpec> {
        TmBW::new(self, 0)
    }
}
#[doc = "Thermal Meter Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmOffsetSpec;
impl crate::RegisterSpec for TmOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_offset::R`](R) reader structure"]
impl crate::Readable for TmOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_offset::W`](W) writer structure"]
impl crate::Writable for TmOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_OFFSET to value 0x0037_1700"]
impl crate::Resettable for TmOffsetSpec {
    const RESET_VALUE: u32 = 0x0037_1700;
}
