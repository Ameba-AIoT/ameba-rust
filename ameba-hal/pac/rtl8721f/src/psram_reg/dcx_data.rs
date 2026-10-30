#[doc = "Register `DCX_DATA` reader"]
pub type R = crate::R<DcxDataSpec>;
#[doc = "Register `DCX_DATA` writer"]
pub type W = crate::W<DcxDataSpec>;
#[doc = "Field `DATA_HEAD_VAL` reader - Value of DCX header control bit in data phase. In 4-wire format data phase, spi_dcx = DATA_HEAD_VAL\\[0\\]."]
pub type DataHeadValR = crate::FieldReader<u16>;
#[doc = "Field `DATA_HEAD_VAL` writer - Value of DCX header control bit in data phase. In 4-wire format data phase, spi_dcx = DATA_HEAD_VAL\\[0\\]."]
pub type DataHeadValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATA_TAIL_VAL` reader - Value of DCX tail control bit in data phase."]
pub type DataTailValR = crate::FieldReader<u16>;
#[doc = "Field `DATA_TAIL_VAL` writer - Value of DCX tail control bit in data phase."]
pub type DataTailValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value of DCX header control bit in data phase. In 4-wire format data phase, spi_dcx = DATA_HEAD_VAL\\[0\\]."]
    #[inline(always)]
    pub fn data_head_val(&self) -> DataHeadValR {
        DataHeadValR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Value of DCX tail control bit in data phase."]
    #[inline(always)]
    pub fn data_tail_val(&self) -> DataTailValR {
        DataTailValR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of DCX header control bit in data phase. In 4-wire format data phase, spi_dcx = DATA_HEAD_VAL\\[0\\]."]
    #[inline(always)]
    pub fn data_head_val(&mut self) -> DataHeadValW<'_, DcxDataSpec> {
        DataHeadValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Value of DCX tail control bit in data phase."]
    #[inline(always)]
    pub fn data_tail_val(&mut self) -> DataTailValW<'_, DcxDataSpec> {
        DataTailValW::new(self, 16)
    }
}
#[doc = "LCDC data phase DCX control bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcxDataSpec;
impl crate::RegisterSpec for DcxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcx_data::R`](R) reader structure"]
impl crate::Readable for DcxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`dcx_data::W`](W) writer structure"]
impl crate::Writable for DcxDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCX_DATA to value 0"]
impl crate::Resettable for DcxDataSpec {}
