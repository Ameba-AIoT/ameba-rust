#[doc = "Register `REG_NAN_ADDR` reader"]
pub type R = crate::R<RegNanAddrSpec>;
#[doc = "Register `REG_NAN_ADDR` writer"]
pub type W = crate::W<RegNanAddrSpec>;
#[doc = "Field `r_NAN_START_ADDR_OFFSET` reader - Start offset Location of NAN beacon report FIFO. Unit: 8bytes"]
pub type RNanStartAddrOffsetR = crate::FieldReader<u16>;
#[doc = "Field `r_NAN_START_ADDR_OFFSET` writer - Start offset Location of NAN beacon report FIFO. Unit: 8bytes"]
pub type RNanStartAddrOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `r_RX_NAN_ADDR_FILTER_EN` reader - enable of rx nan beacon address filter, can not used in D2 testchip"]
pub type RRxNanAddrFilterEnR = crate::BitReader;
#[doc = "Field `r_RX_NAN_ADDR_FILTER_EN` writer - enable of rx nan beacon address filter, can not used in D2 testchip"]
pub type RRxNanAddrFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - Start offset Location of NAN beacon report FIFO. Unit: 8bytes"]
    #[inline(always)]
    pub fn r_nan_start_addr_offset(&self) -> RNanStartAddrOffsetR {
        RNanStartAddrOffsetR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - enable of rx nan beacon address filter, can not used in D2 testchip"]
    #[inline(always)]
    pub fn r_rx_nan_addr_filter_en(&self) -> RRxNanAddrFilterEnR {
        RRxNanAddrFilterEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Start offset Location of NAN beacon report FIFO. Unit: 8bytes"]
    #[inline(always)]
    pub fn r_nan_start_addr_offset(&mut self) -> RNanStartAddrOffsetW<'_, RegNanAddrSpec> {
        RNanStartAddrOffsetW::new(self, 0)
    }
    #[doc = "Bit 13 - enable of rx nan beacon address filter, can not used in D2 testchip"]
    #[inline(always)]
    pub fn r_rx_nan_addr_filter_en(&mut self) -> RRxNanAddrFilterEnW<'_, RegNanAddrSpec> {
        RRxNanAddrFilterEnW::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanAddrSpec;
impl crate::RegisterSpec for RegNanAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_addr::R`](R) reader structure"]
impl crate::Readable for RegNanAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_addr::W`](W) writer structure"]
impl crate::Writable for RegNanAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_ADDR to value 0x0152"]
impl crate::Resettable for RegNanAddrSpec {
    const RESET_VALUE: u32 = 0x0152;
}
