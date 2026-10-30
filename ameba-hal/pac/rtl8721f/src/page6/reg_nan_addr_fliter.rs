#[doc = "Register `REG_NAN_ADDR_FLITER` reader"]
pub type R = crate::R<RegNanAddrFliterSpec>;
#[doc = "Register `REG_NAN_ADDR_FLITER` writer"]
pub type W = crate::W<RegNanAddrFliterSpec>;
#[doc = "Field `r_RX_NAN_ADDR_FILTER` reader - nan address filter"]
pub type RRxNanAddrFilterR = crate::FieldReader<u32>;
#[doc = "Field `r_RX_NAN_ADDR_FILTER` writer - nan address filter"]
pub type RRxNanAddrFilterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - nan address filter"]
    #[inline(always)]
    pub fn r_rx_nan_addr_filter(&self) -> RRxNanAddrFilterR {
        RRxNanAddrFilterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - nan address filter"]
    #[inline(always)]
    pub fn r_rx_nan_addr_filter(&mut self) -> RRxNanAddrFilterW<'_, RegNanAddrFliterSpec> {
        RRxNanAddrFilterW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_addr_fliter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_addr_fliter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanAddrFliterSpec;
impl crate::RegisterSpec for RegNanAddrFliterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_addr_fliter::R`](R) reader structure"]
impl crate::Readable for RegNanAddrFliterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_addr_fliter::W`](W) writer structure"]
impl crate::Writable for RegNanAddrFliterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_ADDR_FLITER to value 0"]
impl crate::Resettable for RegNanAddrFliterSpec {}
