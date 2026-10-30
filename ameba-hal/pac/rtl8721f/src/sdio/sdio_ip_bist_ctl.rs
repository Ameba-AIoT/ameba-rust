#[doc = "Register `SDIO_IP_BIST_CTL` reader"]
pub type R = crate::R<SdioIpBistCtlSpec>;
#[doc = "Register `SDIO_IP_BIST_CTL` writer"]
pub type W = crate::W<SdioIpBistCtlSpec>;
#[doc = "Field `bist_ls` reader - (Reserved)"]
pub type BistLsR = crate::BitReader;
#[doc = "Field `bist_ls` writer - (Reserved)"]
pub type BistLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_ring_buf_rm_0` reader - (Reserved)"]
pub type BistRingBufRm0R = crate::FieldReader;
#[doc = "Field `bist_ring_buf_rm_0` writer - (Reserved)"]
pub type BistRingBufRm0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bist_ring_buf_rme_0` reader - (Reserved)"]
pub type BistRingBufRme0R = crate::BitReader;
#[doc = "Field `bist_ring_buf_rme_0` writer - (Reserved)"]
pub type BistRingBufRme0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_ring_buf_sd_0` reader - (Reserved)"]
pub type BistRingBufSd0R = crate::BitReader;
#[doc = "Field `bist_ring_buf_sd_0` writer - (Reserved)"]
pub type BistRingBufSd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_ring_buf_ds_0` reader - (Reserved)"]
pub type BistRingBufDs0R = crate::BitReader;
#[doc = "Field `bist_ring_buf_ds_0` writer - (Reserved)"]
pub type BistRingBufDs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_ls_hw_sel` reader - (Reserved)"]
pub type BistLsHwSelR = crate::BitReader;
#[doc = "Field `bist_ls_hw_sel` writer - (Reserved)"]
pub type BistLsHwSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ls(&self) -> BistLsR {
        BistLsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_rm_0(&self) -> BistRingBufRm0R {
        BistRingBufRm0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_rme_0(&self) -> BistRingBufRme0R {
        BistRingBufRme0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_sd_0(&self) -> BistRingBufSd0R {
        BistRingBufSd0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_ds_0(&self) -> BistRingBufDs0R {
        BistRingBufDs0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ls_hw_sel(&self) -> BistLsHwSelR {
        BistLsHwSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ls(&mut self) -> BistLsW<'_, SdioIpBistCtlSpec> {
        BistLsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_rm_0(&mut self) -> BistRingBufRm0W<'_, SdioIpBistCtlSpec> {
        BistRingBufRm0W::new(self, 4)
    }
    #[doc = "Bit 8 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_rme_0(&mut self) -> BistRingBufRme0W<'_, SdioIpBistCtlSpec> {
        BistRingBufRme0W::new(self, 8)
    }
    #[doc = "Bit 9 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_sd_0(&mut self) -> BistRingBufSd0W<'_, SdioIpBistCtlSpec> {
        BistRingBufSd0W::new(self, 9)
    }
    #[doc = "Bit 10 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ring_buf_ds_0(&mut self) -> BistRingBufDs0W<'_, SdioIpBistCtlSpec> {
        BistRingBufDs0W::new(self, 10)
    }
    #[doc = "Bit 11 - (Reserved)"]
    #[inline(always)]
    pub fn bist_ls_hw_sel(&mut self) -> BistLsHwSelW<'_, SdioIpBistCtlSpec> {
        BistLsHwSelW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_ip_bist_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_ip_bist_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioIpBistCtlSpec;
impl crate::RegisterSpec for SdioIpBistCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_ip_bist_ctl::R`](R) reader structure"]
impl crate::Readable for SdioIpBistCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdio_ip_bist_ctl::W`](W) writer structure"]
impl crate::Writable for SdioIpBistCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_IP_BIST_CTL to value 0x0800"]
impl crate::Resettable for SdioIpBistCtlSpec {
    const RESET_VALUE: u32 = 0x0800;
}
