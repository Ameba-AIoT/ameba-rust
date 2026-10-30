#[doc = "Register `REG_SW_SWD_CTRL` reader"]
pub type R = crate::R<RegSwSwdCtrlSpec>;
#[doc = "Register `REG_SW_SWD_CTRL` writer"]
pub type W = crate::W<RegSwSwdCtrlSpec>;
#[doc = "Field `sw_swd_dbgen` reader - "]
pub type SwSwdDbgenR = crate::BitReader;
#[doc = "Field `sw_swd_dbgen` writer - "]
pub type SwSwdDbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_swd_niden` reader - "]
pub type SwSwdNidenR = crate::BitReader;
#[doc = "Field `sw_swd_niden` writer - "]
pub type SwSwdNidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_swd_spiden` reader - "]
pub type SwSwdSpidenR = crate::BitReader;
#[doc = "Field `sw_swd_spiden` writer - "]
pub type SwSwdSpidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_swd_spniden` reader - "]
pub type SwSwdSpnidenR = crate::BitReader;
#[doc = "Field `sw_swd_spniden` writer - "]
pub type SwSwdSpnidenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_swd_dbgen(&self) -> SwSwdDbgenR {
        SwSwdDbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_swd_niden(&self) -> SwSwdNidenR {
        SwSwdNidenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sw_swd_spiden(&self) -> SwSwdSpidenR {
        SwSwdSpidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sw_swd_spniden(&self) -> SwSwdSpnidenR {
        SwSwdSpnidenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_swd_dbgen(&mut self) -> SwSwdDbgenW<'_, RegSwSwdCtrlSpec> {
        SwSwdDbgenW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_swd_niden(&mut self) -> SwSwdNidenW<'_, RegSwSwdCtrlSpec> {
        SwSwdNidenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sw_swd_spiden(&mut self) -> SwSwdSpidenW<'_, RegSwSwdCtrlSpec> {
        SwSwdSpidenW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sw_swd_spniden(&mut self) -> SwSwdSpnidenW<'_, RegSwSwdCtrlSpec> {
        SwSwdSpnidenW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_swd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_swd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwSwdCtrlSpec;
impl crate::RegisterSpec for RegSwSwdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_swd_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSwSwdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_swd_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSwSwdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_SWD_CTRL to value 0"]
impl crate::Resettable for RegSwSwdCtrlSpec {}
