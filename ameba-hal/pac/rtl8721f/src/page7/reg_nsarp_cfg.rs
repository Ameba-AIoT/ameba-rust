#[doc = "Register `REG_NSARP_CFG` reader"]
pub type R = crate::R<RegNsarpCfgSpec>;
#[doc = "Register `REG_NSARP_CFG` writer"]
pub type W = crate::W<RegNsarpCfgSpec>;
#[doc = "Field `BIT_NSARP_RSPSEC` reader - select response packet security type"]
pub type BitNsarpRspsecR = crate::FieldReader;
#[doc = "Field `BIT_NSARP_RSPSEC` writer - select response packet security type"]
pub type BitNsarpRspsecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_NSARP_RSPFTP` reader - used to select Response_type and TID"]
pub type BitNsarpRspftpR = crate::FieldReader;
#[doc = "Field `BIT_NSARP_RSPFTP` writer - used to select Response_type and TID"]
pub type BitNsarpRspftpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_NSARP_MODEN` reader - r_WMAC_NSARP_MODEN\\[0\\]: when set to 1, enable arp request; r_WMAC_NSARP_MODEN\\[1\\]: when set to 1, enable ipv6 request;"]
pub type BitNsarpModenR = crate::FieldReader;
#[doc = "Field `BIT_NSARP_MODEN` writer - r_WMAC_NSARP_MODEN\\[0\\]: when set to 1, enable arp request; r_WMAC_NSARP_MODEN\\[1\\]: when set to 1, enable ipv6 request;"]
pub type BitNsarpModenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_NSARP_RIPV6` reader - when set to 1, reject IPV6"]
pub type BitNsarpRipv6R = crate::BitReader;
#[doc = "Field `BIT_NSARP_RIPV6` writer - when set to 1, reject IPV6"]
pub type BitNsarpRipv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NSARP_PARP` reader - when set to 1, reject ARP"]
pub type BitNsarpParpR = crate::BitReader;
#[doc = "Field `BIT_NSARP_PARP` writer - when set to 1, reject ARP"]
pub type BitNsarpParpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NSARP_RESPEN` reader - NAARP parser enable"]
pub type BitNsarpRespenR = crate::BitReader;
#[doc = "Field `BIT_NSARP_RESPEN` writer - NAARP parser enable"]
pub type BitNsarpRespenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_wmac_csi_timer` reader - A timeout value for csi_wait_poll. (unit: 512us)"]
pub type BitWmacCsiTimerR = crate::FieldReader;
#[doc = "Field `BIT_wmac_csi_timer` writer - A timeout value for csi_wait_poll. (unit: 512us)"]
pub type BitWmacCsiTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - select response packet security type"]
    #[inline(always)]
    pub fn bit_nsarp_rspsec(&self) -> BitNsarpRspsecR {
        BitNsarpRspsecR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - used to select Response_type and TID"]
    #[inline(always)]
    pub fn bit_nsarp_rspftp(&self) -> BitNsarpRspftpR {
        BitNsarpRspftpR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - r_WMAC_NSARP_MODEN\\[0\\]: when set to 1, enable arp request; r_WMAC_NSARP_MODEN\\[1\\]: when set to 1, enable ipv6 request;"]
    #[inline(always)]
    pub fn bit_nsarp_moden(&self) -> BitNsarpModenR {
        BitNsarpModenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - when set to 1, reject IPV6"]
    #[inline(always)]
    pub fn bit_nsarp_ripv6(&self) -> BitNsarpRipv6R {
        BitNsarpRipv6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - when set to 1, reject ARP"]
    #[inline(always)]
    pub fn bit_nsarp_parp(&self) -> BitNsarpParpR {
        BitNsarpParpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - NAARP parser enable"]
    #[inline(always)]
    pub fn bit_nsarp_respen(&self) -> BitNsarpRespenR {
        BitNsarpRespenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - A timeout value for csi_wait_poll. (unit: 512us)"]
    #[inline(always)]
    pub fn bit_wmac_csi_timer(&self) -> BitWmacCsiTimerR {
        BitWmacCsiTimerR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - select response packet security type"]
    #[inline(always)]
    pub fn bit_nsarp_rspsec(&mut self) -> BitNsarpRspsecW<'_, RegNsarpCfgSpec> {
        BitNsarpRspsecW::new(self, 0)
    }
    #[doc = "Bits 4:5 - used to select Response_type and TID"]
    #[inline(always)]
    pub fn bit_nsarp_rspftp(&mut self) -> BitNsarpRspftpW<'_, RegNsarpCfgSpec> {
        BitNsarpRspftpW::new(self, 4)
    }
    #[doc = "Bits 6:7 - r_WMAC_NSARP_MODEN\\[0\\]: when set to 1, enable arp request; r_WMAC_NSARP_MODEN\\[1\\]: when set to 1, enable ipv6 request;"]
    #[inline(always)]
    pub fn bit_nsarp_moden(&mut self) -> BitNsarpModenW<'_, RegNsarpCfgSpec> {
        BitNsarpModenW::new(self, 6)
    }
    #[doc = "Bit 8 - when set to 1, reject IPV6"]
    #[inline(always)]
    pub fn bit_nsarp_ripv6(&mut self) -> BitNsarpRipv6W<'_, RegNsarpCfgSpec> {
        BitNsarpRipv6W::new(self, 8)
    }
    #[doc = "Bit 9 - when set to 1, reject ARP"]
    #[inline(always)]
    pub fn bit_nsarp_parp(&mut self) -> BitNsarpParpW<'_, RegNsarpCfgSpec> {
        BitNsarpParpW::new(self, 9)
    }
    #[doc = "Bit 15 - NAARP parser enable"]
    #[inline(always)]
    pub fn bit_nsarp_respen(&mut self) -> BitNsarpRespenW<'_, RegNsarpCfgSpec> {
        BitNsarpRespenW::new(self, 15)
    }
    #[doc = "Bits 16:23 - A timeout value for csi_wait_poll. (unit: 512us)"]
    #[inline(always)]
    pub fn bit_wmac_csi_timer(&mut self) -> BitWmacCsiTimerW<'_, RegNsarpCfgSpec> {
        BitWmacCsiTimerW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nsarp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nsarp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNsarpCfgSpec;
impl crate::RegisterSpec for RegNsarpCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nsarp_cfg::R`](R) reader structure"]
impl crate::Readable for RegNsarpCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nsarp_cfg::W`](W) writer structure"]
impl crate::Writable for RegNsarpCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NSARP_CFG to value 0x0042_0000"]
impl crate::Resettable for RegNsarpCfgSpec {
    const RESET_VALUE: u32 = 0x0042_0000;
}
