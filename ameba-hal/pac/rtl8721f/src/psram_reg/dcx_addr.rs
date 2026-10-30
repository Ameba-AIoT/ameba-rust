#[doc = "Register `DCX_ADDR` reader"]
pub type R = crate::R<DcxAddrSpec>;
#[doc = "Register `DCX_ADDR` writer"]
pub type W = crate::W<DcxAddrSpec>;
#[doc = "Field `ADDR_HEAD_VAL` reader - Value of DCX header control bit in address phase. In 4-wire format address phase, spi_dcx = ADDR_HEAD_VAL\\[0\\]."]
pub type AddrHeadValR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_HEAD_VAL` writer - Value of DCX header control bit in address phase. In 4-wire format address phase, spi_dcx = ADDR_HEAD_VAL\\[0\\]."]
pub type AddrHeadValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDR_TAIL_VAL` reader - Value of DCX tail control bit in address phase."]
pub type AddrTailValR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_TAIL_VAL` writer - Value of DCX tail control bit in address phase."]
pub type AddrTailValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value of DCX header control bit in address phase. In 4-wire format address phase, spi_dcx = ADDR_HEAD_VAL\\[0\\]."]
    #[inline(always)]
    pub fn addr_head_val(&self) -> AddrHeadValR {
        AddrHeadValR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Value of DCX tail control bit in address phase."]
    #[inline(always)]
    pub fn addr_tail_val(&self) -> AddrTailValR {
        AddrTailValR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of DCX header control bit in address phase. In 4-wire format address phase, spi_dcx = ADDR_HEAD_VAL\\[0\\]."]
    #[inline(always)]
    pub fn addr_head_val(&mut self) -> AddrHeadValW<'_, DcxAddrSpec> {
        AddrHeadValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Value of DCX tail control bit in address phase."]
    #[inline(always)]
    pub fn addr_tail_val(&mut self) -> AddrTailValW<'_, DcxAddrSpec> {
        AddrTailValW::new(self, 16)
    }
}
#[doc = "LCDC address phase DCX control bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcx_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcx_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcxAddrSpec;
impl crate::RegisterSpec for DcxAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcx_addr::R`](R) reader structure"]
impl crate::Readable for DcxAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcx_addr::W`](W) writer structure"]
impl crate::Writable for DcxAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCX_ADDR to value 0"]
impl crate::Resettable for DcxAddrSpec {}
