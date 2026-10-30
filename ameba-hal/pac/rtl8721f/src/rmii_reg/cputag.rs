#[doc = "Register `CPUTAG` reader"]
pub type R = crate::R<CputagSpec>;
#[doc = "Register `CPUTAG` writer"]
pub type W = crate::W<CputagSpec>;
#[doc = "Field `CTPV` reader - CPU tag protocol value. 8306:0x90 8368:0xa0 or 0xb0 8370:0x04 8307h:0x04"]
pub type CtpvR = crate::FieldReader;
#[doc = "Field `CTPV` writer - CPU tag protocol value. 8306:0x90 8368:0xa0 or 0xb0 8370:0x04 8307h:0x04"]
pub type CtpvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTPM` reader - CPU tag protocol mask. 8306:0xf0 8368:0xe0 8370:0xff 8307h:0xff"]
pub type CtpmR = crate::FieldReader;
#[doc = "Field `CTPM` writer - CPU tag protocol mask. 8306:0xf0 8368:0xe0 8370:0xff 8307h:0xff"]
pub type CtpmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CT_RSIZE_1_0` reader - "]
pub type CtRsize1_0R = crate::FieldReader;
#[doc = "Field `CT_RSIZE_1_0` writer - "]
pub type CtRsize1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CT_SWITCH` reader - Support cputag format of switch 0: no cputag support 1: 8368 2: 8306 3: 8307 4: 8370 5: gmac in 8681. 6: gmac in Apollo, rle0371, rl6266 and rtl9601(rle0639)"]
pub type CtSwitchR = crate::FieldReader;
#[doc = "Field `CT_SWITCH` writer - Support cputag format of switch 0: no cputag support 1: 8368 2: 8306 3: 8307 4: 8370 5: gmac in 8681. 6: gmac in Apollo, rle0371, rl6266 and rtl9601(rle0639)"]
pub type CtSwitchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CT_ASPRI` reader - In RTL8307h, setting ASPRI field in tx cputag when short_dsc_format = 0. In RTL8370S, setting priority select field in tx cputag when short_dsc_format =0.."]
pub type CtAspriR = crate::BitReader;
#[doc = "Field `CT_ASPRI` writer - In RTL8307h, setting ASPRI field in tx cputag when short_dsc_format = 0. In RTL8370S, setting priority select field in tx cputag when short_dsc_format =0.."]
pub type CtAspriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CT_NORMK` reader - Setting of NORMK field in RTL8307h tx cputag when short_dsc_format = 0."]
pub type CtNormkR = crate::BitReader;
#[doc = "Field `CT_NORMK` writer - Setting of NORMK field in RTL8307h tx cputag when short_dsc_format = 0."]
pub type CtNormkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CT_DSLRN` reader - In rtl8370s, Setting of Disable Learning field in tx cputag. Setting of Disable Learning field in RTL8368 or DSLRN field in RTL8307h tx cputag when short_dsc_format = 0."]
pub type CtDslrnR = crate::BitReader;
#[doc = "Field `CT_DSLRN` writer - In rtl8370s, Setting of Disable Learning field in tx cputag. Setting of Disable Learning field in RTL8368 or DSLRN field in RTL8307h tx cputag when short_dsc_format = 0."]
pub type CtDslrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CT_RSIZE_3_2` reader - After rl8681 (including), this field is only for ingress pkt. CTR_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte F or Apollo cputag with PTP timestamp, set CT_RSIZE to h2."]
pub type CtRsize3_2R = crate::FieldReader;
#[doc = "Field `CT_RSIZE_3_2` writer - After rl8681 (including), this field is only for ingress pkt. CTR_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte F or Apollo cputag with PTP timestamp, set CT_RSIZE to h2."]
pub type CtRsize3_2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CT_TSIZE` reader - Cputag size in egress pkt. CTT_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte 4'h3: 12 Byte"]
pub type CtTsizeR = crate::FieldReader;
#[doc = "Field `CT_TSIZE` writer - Cputag size in egress pkt. CTT_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte 4'h3: 12 Byte"]
pub type CtTsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTEN_RX` reader - Enable parsing ingress packet with cputag format."]
pub type CtenRxR = crate::BitReader;
#[doc = "Field `CTEN_RX` writer - Enable parsing ingress packet with cputag format."]
pub type CtenRxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - CPU tag protocol value. 8306:0x90 8368:0xa0 or 0xb0 8370:0x04 8307h:0x04"]
    #[inline(always)]
    pub fn ctpv(&self) -> CtpvR {
        CtpvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CPU tag protocol mask. 8306:0xf0 8368:0xe0 8370:0xff 8307h:0xff"]
    #[inline(always)]
    pub fn ctpm(&self) -> CtpmR {
        CtpmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ct_rsize_1_0(&self) -> CtRsize1_0R {
        CtRsize1_0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - Support cputag format of switch 0: no cputag support 1: 8368 2: 8306 3: 8307 4: 8370 5: gmac in 8681. 6: gmac in Apollo, rle0371, rl6266 and rtl9601(rle0639)"]
    #[inline(always)]
    pub fn ct_switch(&self) -> CtSwitchR {
        CtSwitchR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - In RTL8307h, setting ASPRI field in tx cputag when short_dsc_format = 0. In RTL8370S, setting priority select field in tx cputag when short_dsc_format =0.."]
    #[inline(always)]
    pub fn ct_aspri(&self) -> CtAspriR {
        CtAspriR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Setting of NORMK field in RTL8307h tx cputag when short_dsc_format = 0."]
    #[inline(always)]
    pub fn ct_normk(&self) -> CtNormkR {
        CtNormkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In rtl8370s, Setting of Disable Learning field in tx cputag. Setting of Disable Learning field in RTL8368 or DSLRN field in RTL8307h tx cputag when short_dsc_format = 0."]
    #[inline(always)]
    pub fn ct_dslrn(&self) -> CtDslrnR {
        CtDslrnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - After rl8681 (including), this field is only for ingress pkt. CTR_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte F or Apollo cputag with PTP timestamp, set CT_RSIZE to h2."]
    #[inline(always)]
    pub fn ct_rsize_3_2(&self) -> CtRsize3_2R {
        CtRsize3_2R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:30 - Cputag size in egress pkt. CTT_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte 4'h3: 12 Byte"]
    #[inline(always)]
    pub fn ct_tsize(&self) -> CtTsizeR {
        CtTsizeR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Enable parsing ingress packet with cputag format."]
    #[inline(always)]
    pub fn cten_rx(&self) -> CtenRxR {
        CtenRxR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPU tag protocol value. 8306:0x90 8368:0xa0 or 0xb0 8370:0x04 8307h:0x04"]
    #[inline(always)]
    pub fn ctpv(&mut self) -> CtpvW<'_, CputagSpec> {
        CtpvW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CPU tag protocol mask. 8306:0xf0 8368:0xe0 8370:0xff 8307h:0xff"]
    #[inline(always)]
    pub fn ctpm(&mut self) -> CtpmW<'_, CputagSpec> {
        CtpmW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ct_rsize_1_0(&mut self) -> CtRsize1_0W<'_, CputagSpec> {
        CtRsize1_0W::new(self, 16)
    }
    #[doc = "Bits 18:21 - Support cputag format of switch 0: no cputag support 1: 8368 2: 8306 3: 8307 4: 8370 5: gmac in 8681. 6: gmac in Apollo, rle0371, rl6266 and rtl9601(rle0639)"]
    #[inline(always)]
    pub fn ct_switch(&mut self) -> CtSwitchW<'_, CputagSpec> {
        CtSwitchW::new(self, 18)
    }
    #[doc = "Bit 22 - In RTL8307h, setting ASPRI field in tx cputag when short_dsc_format = 0. In RTL8370S, setting priority select field in tx cputag when short_dsc_format =0.."]
    #[inline(always)]
    pub fn ct_aspri(&mut self) -> CtAspriW<'_, CputagSpec> {
        CtAspriW::new(self, 22)
    }
    #[doc = "Bit 23 - Setting of NORMK field in RTL8307h tx cputag when short_dsc_format = 0."]
    #[inline(always)]
    pub fn ct_normk(&mut self) -> CtNormkW<'_, CputagSpec> {
        CtNormkW::new(self, 23)
    }
    #[doc = "Bit 24 - In rtl8370s, Setting of Disable Learning field in tx cputag. Setting of Disable Learning field in RTL8368 or DSLRN field in RTL8307h tx cputag when short_dsc_format = 0."]
    #[inline(always)]
    pub fn ct_dslrn(&mut self) -> CtDslrnW<'_, CputagSpec> {
        CtDslrnW::new(self, 24)
    }
    #[doc = "Bits 25:26 - After rl8681 (including), this field is only for ingress pkt. CTR_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte F or Apollo cputag with PTP timestamp, set CT_RSIZE to h2."]
    #[inline(always)]
    pub fn ct_rsize_3_2(&mut self) -> CtRsize3_2W<'_, CputagSpec> {
        CtRsize3_2W::new(self, 25)
    }
    #[doc = "Bits 27:30 - Cputag size in egress pkt. CTT_size: 4'h0: 0 Byte 4'h1: 4 Byte 4'h2: 8Byte 4'h3: 12 Byte"]
    #[inline(always)]
    pub fn ct_tsize(&mut self) -> CtTsizeW<'_, CputagSpec> {
        CtTsizeW::new(self, 27)
    }
    #[doc = "Bit 31 - Enable parsing ingress packet with cputag format."]
    #[inline(always)]
    pub fn cten_rx(&mut self) -> CtenRxW<'_, CputagSpec> {
        CtenRxW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cputag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cputag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CputagSpec;
impl crate::RegisterSpec for CputagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cputag::R`](R) reader structure"]
impl crate::Readable for CputagSpec {}
#[doc = "`write(|w| ..)` method takes [`cputag::W`](W) writer structure"]
impl crate::Writable for CputagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUTAG to value 0x0140_0000"]
impl crate::Resettable for CputagSpec {
    const RESET_VALUE: u32 = 0x0140_0000;
}
