#[doc = "Register `REG_VENDOR_TSFT_CTL` reader"]
pub type R = crate::R<RegVendorTsftCtlSpec>;
#[doc = "Register `REG_VENDOR_TSFT_CTL` writer"]
pub type W = crate::W<RegVendorTsftCtlSpec>;
#[doc = "Field `r_tx_vendor_ns_offset` reader - vendor ns offset for tx"]
pub type RTxVendorNsOffsetR = crate::FieldReader<u16>;
#[doc = "Field `r_tx_vendor_ns_offset` writer - vendor ns offset for tx"]
pub type RTxVendorNsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_tx_vendor_tsft_en` reader - vendor tsft enable for tx"]
pub type RTxVendorTsftEnR = crate::BitReader;
#[doc = "Field `r_tx_vendor_tsft_en` writer - vendor tsft enable for tx"]
pub type RTxVendorTsftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rx_vendor_ns_offset` reader - vendor ns offset for rx"]
pub type RRxVendorNsOffsetR = crate::FieldReader<u16>;
#[doc = "Field `r_rx_vendor_ns_offset` writer - vendor ns offset for rx"]
pub type RRxVendorNsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_rx_vendor_tsft_en` reader - vendor tsft enable for rx"]
pub type RRxVendorTsftEnR = crate::BitReader;
#[doc = "Field `r_rx_vendor_tsft_en` writer - vendor tsft enable for rx"]
pub type RRxVendorTsftEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - vendor ns offset for tx"]
    #[inline(always)]
    pub fn r_tx_vendor_ns_offset(&self) -> RTxVendorNsOffsetR {
        RTxVendorNsOffsetR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - vendor tsft enable for tx"]
    #[inline(always)]
    pub fn r_tx_vendor_tsft_en(&self) -> RTxVendorTsftEnR {
        RTxVendorTsftEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:25 - vendor ns offset for rx"]
    #[inline(always)]
    pub fn r_rx_vendor_ns_offset(&self) -> RRxVendorNsOffsetR {
        RRxVendorNsOffsetR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - vendor tsft enable for rx"]
    #[inline(always)]
    pub fn r_rx_vendor_tsft_en(&self) -> RRxVendorTsftEnR {
        RRxVendorTsftEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - vendor ns offset for tx"]
    #[inline(always)]
    pub fn r_tx_vendor_ns_offset(&mut self) -> RTxVendorNsOffsetW<'_, RegVendorTsftCtlSpec> {
        RTxVendorNsOffsetW::new(self, 0)
    }
    #[doc = "Bit 10 - vendor tsft enable for tx"]
    #[inline(always)]
    pub fn r_tx_vendor_tsft_en(&mut self) -> RTxVendorTsftEnW<'_, RegVendorTsftCtlSpec> {
        RTxVendorTsftEnW::new(self, 10)
    }
    #[doc = "Bits 16:25 - vendor ns offset for rx"]
    #[inline(always)]
    pub fn r_rx_vendor_ns_offset(&mut self) -> RRxVendorNsOffsetW<'_, RegVendorTsftCtlSpec> {
        RRxVendorNsOffsetW::new(self, 16)
    }
    #[doc = "Bit 26 - vendor tsft enable for rx"]
    #[inline(always)]
    pub fn r_rx_vendor_tsft_en(&mut self) -> RRxVendorTsftEnW<'_, RegVendorTsftCtlSpec> {
        RRxVendorTsftEnW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_vendor_tsft_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_vendor_tsft_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegVendorTsftCtlSpec;
impl crate::RegisterSpec for RegVendorTsftCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_vendor_tsft_ctl::R`](R) reader structure"]
impl crate::Readable for RegVendorTsftCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_vendor_tsft_ctl::W`](W) writer structure"]
impl crate::Writable for RegVendorTsftCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VENDOR_TSFT_CTL to value 0"]
impl crate::Resettable for RegVendorTsftCtlSpec {}
