#[doc = "Register `REG_PLAT_STATUS` reader"]
pub type R = crate::R<RegPlatStatusSpec>;
#[doc = "Register `REG_PLAT_STATUS` writer"]
pub type W = crate::W<RegPlatStatusSpec>;
#[doc = "Field `km4tz_slp_mode` reader - km4tz sleep status"]
pub type Km4tzSlpModeR = crate::BitReader;
#[doc = "Field `km4tz_lockup_m` reader - km4tz lockup status"]
pub type Km4tzLockupMR = crate::BitReader;
#[doc = "Field `km4tz_dbg_mode` reader - km4tz halt status"]
pub type Km4tzDbgModeR = crate::BitReader;
#[doc = "Field `km4tz_dbgrestarted_m` reader - km4tz debug restart status"]
pub type Km4tzDbgrestartedMR = crate::BitReader;
#[doc = "Field `km4ns_slp_mode` reader - km4ns sleep status"]
pub type Km4nsSlpModeR = crate::BitReader;
#[doc = "Field `km4ns_lockup_m` reader - km4ns lockup status"]
pub type Km4nsLockupMR = crate::BitReader;
#[doc = "Field `km4ns_dbg_mode` reader - km4ns halt status"]
pub type Km4nsDbgModeR = crate::BitReader;
#[doc = "Field `km4ns_dbgrestarted_m` reader - km4ns debug restart status"]
pub type Km4nsDbgrestartedMR = crate::BitReader;
#[doc = "Field `km4ns_share_cache_mem` reader - 0: km4ns cache memory used as cache 1: km4ns cache memory used as dtcm"]
pub type Km4nsShareCacheMemR = crate::BitReader;
#[doc = "Field `km4ns_share_cache_mem` writer - 0: km4ns cache memory used as cache 1: km4ns cache memory used as dtcm"]
pub type Km4nsShareCacheMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4tz_share_cache_mem` reader - 0: km4tz cache memory used as cache 1: km4tz cache memory used as dtcm"]
pub type Km4tzShareCacheMemR = crate::BitReader;
#[doc = "Field `km4tz_share_cache_mem` writer - 0: km4tz cache memory used as cache 1: km4tz cache memory used as dtcm"]
pub type Km4tzShareCacheMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4ns_rst_halt` reader - "]
pub type Km4nsRstHaltR = crate::BitReader;
#[doc = "Field `km4ns_rst_halt` writer - "]
pub type Km4nsRstHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4tz_rst_halt` reader - "]
pub type Km4tzRstHaltR = crate::BitReader;
#[doc = "Field `km4tz_rst_halt` writer - "]
pub type Km4tzRstHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `init_xo_range_en` reader - "]
pub type InitXoRangeEnR = crate::BitReader;
#[doc = "Field `init_xo_range_en` writer - "]
pub type InitXoRangeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_attr` reader - pke key secure attribute"]
pub type PkeAttrR = crate::FieldReader;
#[doc = "Field `pke_attr` writer - pke key secure attribute"]
pub type PkeAttrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - km4tz sleep status"]
    #[inline(always)]
    pub fn km4tz_slp_mode(&self) -> Km4tzSlpModeR {
        Km4tzSlpModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - km4tz lockup status"]
    #[inline(always)]
    pub fn km4tz_lockup_m(&self) -> Km4tzLockupMR {
        Km4tzLockupMR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - km4tz halt status"]
    #[inline(always)]
    pub fn km4tz_dbg_mode(&self) -> Km4tzDbgModeR {
        Km4tzDbgModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - km4tz debug restart status"]
    #[inline(always)]
    pub fn km4tz_dbgrestarted_m(&self) -> Km4tzDbgrestartedMR {
        Km4tzDbgrestartedMR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - km4ns sleep status"]
    #[inline(always)]
    pub fn km4ns_slp_mode(&self) -> Km4nsSlpModeR {
        Km4nsSlpModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - km4ns lockup status"]
    #[inline(always)]
    pub fn km4ns_lockup_m(&self) -> Km4nsLockupMR {
        Km4nsLockupMR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - km4ns halt status"]
    #[inline(always)]
    pub fn km4ns_dbg_mode(&self) -> Km4nsDbgModeR {
        Km4nsDbgModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - km4ns debug restart status"]
    #[inline(always)]
    pub fn km4ns_dbgrestarted_m(&self) -> Km4nsDbgrestartedMR {
        Km4nsDbgrestartedMR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - 0: km4ns cache memory used as cache 1: km4ns cache memory used as dtcm"]
    #[inline(always)]
    pub fn km4ns_share_cache_mem(&self) -> Km4nsShareCacheMemR {
        Km4nsShareCacheMemR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0: km4tz cache memory used as cache 1: km4tz cache memory used as dtcm"]
    #[inline(always)]
    pub fn km4tz_share_cache_mem(&self) -> Km4tzShareCacheMemR {
        Km4tzShareCacheMemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn km4ns_rst_halt(&self) -> Km4nsRstHaltR {
        Km4nsRstHaltR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn km4tz_rst_halt(&self) -> Km4tzRstHaltR {
        Km4tzRstHaltR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn init_xo_range_en(&self) -> InitXoRangeEnR {
        InitXoRangeEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - pke key secure attribute"]
    #[inline(always)]
    pub fn pke_attr(&self) -> PkeAttrR {
        PkeAttrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - 0: km4ns cache memory used as cache 1: km4ns cache memory used as dtcm"]
    #[inline(always)]
    pub fn km4ns_share_cache_mem(&mut self) -> Km4nsShareCacheMemW<'_, RegPlatStatusSpec> {
        Km4nsShareCacheMemW::new(self, 12)
    }
    #[doc = "Bit 13 - 0: km4tz cache memory used as cache 1: km4tz cache memory used as dtcm"]
    #[inline(always)]
    pub fn km4tz_share_cache_mem(&mut self) -> Km4tzShareCacheMemW<'_, RegPlatStatusSpec> {
        Km4tzShareCacheMemW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn km4ns_rst_halt(&mut self) -> Km4nsRstHaltW<'_, RegPlatStatusSpec> {
        Km4nsRstHaltW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn km4tz_rst_halt(&mut self) -> Km4tzRstHaltW<'_, RegPlatStatusSpec> {
        Km4tzRstHaltW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn init_xo_range_en(&mut self) -> InitXoRangeEnW<'_, RegPlatStatusSpec> {
        InitXoRangeEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - pke key secure attribute"]
    #[inline(always)]
    pub fn pke_attr(&mut self) -> PkeAttrW<'_, RegPlatStatusSpec> {
        PkeAttrW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_plat_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_plat_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPlatStatusSpec;
impl crate::RegisterSpec for RegPlatStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_plat_status::R`](R) reader structure"]
impl crate::Readable for RegPlatStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_plat_status::W`](W) writer structure"]
impl crate::Writable for RegPlatStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PLAT_STATUS to value 0"]
impl crate::Resettable for RegPlatStatusSpec {}
