#[doc = "Register `REG_RXCSIBUF_CFG` reader"]
pub type R = crate::R<RegRxcsibufCfgSpec>;
#[doc = "Register `REG_RXCSIBUF_CFG` writer"]
pub type W = crate::W<RegRxcsibufCfgSpec>;
#[doc = "Field `r_csibuf_endaddr` reader - End address of rxcsibuffer, unit 8bytes. Rxcsibuffer is 1536 bytes"]
pub type RCsibufEndaddrR = crate::FieldReader;
#[doc = "Field `r_csibuf_endaddr` writer - End address of rxcsibuffer, unit 8bytes. Rxcsibuffer is 1536 bytes"]
pub type RCsibufEndaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_csibuf_threshod` reader - rxcsibuffer full threshold, size must greater than or equal to csi descriptors"]
pub type RCsibufThreshodR = crate::FieldReader;
#[doc = "Field `r_csibuf_threshod` writer - rxcsibuffer full threshold, size must greater than or equal to csi descriptors"]
pub type RCsibufThreshodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_chinfo_wdfs_time` reader - Wait DFS timeout value, unit 4 mac clock cycles"]
pub type RChinfoWdfsTimeR = crate::FieldReader;
#[doc = "Field `r_chinfo_wdfs_time` writer - Wait DFS timeout value, unit 4 mac clock cycles"]
pub type RChinfoWdfsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_chinfo_req_size` reader - The channel info trigger request rxcsibuffer size, unit 32bytes, the actually request size = (r_chinfo_req_size + 1) * 32byte"]
pub type RChinfoReqSizeR = crate::FieldReader;
#[doc = "Field `r_chinfo_req_size` writer - The channel info trigger request rxcsibuffer size, unit 32bytes, the actually request size = (r_chinfo_req_size + 1) * 32byte"]
pub type RChinfoReqSizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_chinfo_offset` reader - used for rx normal mode the payload offset for compare with configured value. 0 means the 0-3bytes payload need to compare after mac header/iv/eiv"]
pub type RChinfoOffsetR = crate::FieldReader;
#[doc = "Field `r_chinfo_offset` writer - used for rx normal mode the payload offset for compare with configured value. 0 means the 0-3bytes payload need to compare after mac header/iv/eiv"]
pub type RChinfoOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End address of rxcsibuffer, unit 8bytes. Rxcsibuffer is 1536 bytes"]
    #[inline(always)]
    pub fn r_csibuf_endaddr(&self) -> RCsibufEndaddrR {
        RCsibufEndaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - rxcsibuffer full threshold, size must greater than or equal to csi descriptors"]
    #[inline(always)]
    pub fn r_csibuf_threshod(&self) -> RCsibufThreshodR {
        RCsibufThreshodR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Wait DFS timeout value, unit 4 mac clock cycles"]
    #[inline(always)]
    pub fn r_chinfo_wdfs_time(&self) -> RChinfoWdfsTimeR {
        RChinfoWdfsTimeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - The channel info trigger request rxcsibuffer size, unit 32bytes, the actually request size = (r_chinfo_req_size + 1) * 32byte"]
    #[inline(always)]
    pub fn r_chinfo_req_size(&self) -> RChinfoReqSizeR {
        RChinfoReqSizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - used for rx normal mode the payload offset for compare with configured value. 0 means the 0-3bytes payload need to compare after mac header/iv/eiv"]
    #[inline(always)]
    pub fn r_chinfo_offset(&self) -> RChinfoOffsetR {
        RChinfoOffsetR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End address of rxcsibuffer, unit 8bytes. Rxcsibuffer is 1536 bytes"]
    #[inline(always)]
    pub fn r_csibuf_endaddr(&mut self) -> RCsibufEndaddrW<'_, RegRxcsibufCfgSpec> {
        RCsibufEndaddrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - rxcsibuffer full threshold, size must greater than or equal to csi descriptors"]
    #[inline(always)]
    pub fn r_csibuf_threshod(&mut self) -> RCsibufThreshodW<'_, RegRxcsibufCfgSpec> {
        RCsibufThreshodW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Wait DFS timeout value, unit 4 mac clock cycles"]
    #[inline(always)]
    pub fn r_chinfo_wdfs_time(&mut self) -> RChinfoWdfsTimeW<'_, RegRxcsibufCfgSpec> {
        RChinfoWdfsTimeW::new(self, 12)
    }
    #[doc = "Bits 16:21 - The channel info trigger request rxcsibuffer size, unit 32bytes, the actually request size = (r_chinfo_req_size + 1) * 32byte"]
    #[inline(always)]
    pub fn r_chinfo_req_size(&mut self) -> RChinfoReqSizeW<'_, RegRxcsibufCfgSpec> {
        RChinfoReqSizeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - used for rx normal mode the payload offset for compare with configured value. 0 means the 0-3bytes payload need to compare after mac header/iv/eiv"]
    #[inline(always)]
    pub fn r_chinfo_offset(&mut self) -> RChinfoOffsetW<'_, RegRxcsibufCfgSpec> {
        RChinfoOffsetW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxcsibuf_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxcsibuf_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxcsibufCfgSpec;
impl crate::RegisterSpec for RegRxcsibufCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxcsibuf_cfg::R`](R) reader structure"]
impl crate::Readable for RegRxcsibufCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxcsibuf_cfg::W`](W) writer structure"]
impl crate::Writable for RegRxcsibufCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXCSIBUF_CFG to value 0xa4bf"]
impl crate::Resettable for RegRxcsibufCfgSpec {
    const RESET_VALUE: u32 = 0xa4bf;
}
