#[doc = "Register `GPIO_ITN_LS_SYNC` reader"]
pub type R = crate::R<GpioItnLsSyncSpec>;
#[doc = "Register `GPIO_ITN_LS_SYNC` writer"]
pub type W = crate::W<GpioItnLsSyncSpec>;
#[doc = "Field `GPIO_Level_SYNC` reader - Writing a 1 to this register results in all level-sensitive interrupts being synchronized to pclk_intr. 0: No synchronization to pclk_intr (default) 1: Synchronize to pclk_intr"]
pub type GpioLevelSyncR = crate::BitReader;
#[doc = "Field `GPIO_Level_SYNC` writer - Writing a 1 to this register results in all level-sensitive interrupts being synchronized to pclk_intr. 0: No synchronization to pclk_intr (default) 1: Synchronize to pclk_intr"]
pub type GpioLevelSyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a 1 to this register results in all level-sensitive interrupts being synchronized to pclk_intr. 0: No synchronization to pclk_intr (default) 1: Synchronize to pclk_intr"]
    #[inline(always)]
    pub fn gpio_level_sync(&self) -> GpioLevelSyncR {
        GpioLevelSyncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this register results in all level-sensitive interrupts being synchronized to pclk_intr. 0: No synchronization to pclk_intr (default) 1: Synchronize to pclk_intr"]
    #[inline(always)]
    pub fn gpio_level_sync(&mut self) -> GpioLevelSyncW<'_, GpioItnLsSyncSpec> {
        GpioLevelSyncW::new(self, 0)
    }
}
#[doc = "Port level interrupt synchronization\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_itn_ls_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_itn_ls_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioItnLsSyncSpec;
impl crate::RegisterSpec for GpioItnLsSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_itn_ls_sync::R`](R) reader structure"]
impl crate::Readable for GpioItnLsSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_itn_ls_sync::W`](W) writer structure"]
impl crate::Writable for GpioItnLsSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_ITN_LS_SYNC to value 0"]
impl crate::Resettable for GpioItnLsSyncSpec {}
