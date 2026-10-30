#[doc = "Register `GRP_USB_E0_CTRL` reader"]
pub type R = crate::R<GrpUsbE0CtrlSpec>;
#[doc = "Register `GRP_USB_E0_CTRL` writer"]
pub type W = crate::W<GrpUsbE0CtrlSpec>;
#[doc = "Field `grp_usb_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpUsbE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_usb_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpUsbE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_usb_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpUsbE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_usb_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpUsbE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_usb_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpUsbE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_usb_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpUsbE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_usb_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpUsbE0MselR = crate::FieldReader;
#[doc = "Field `grp_usb_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpUsbE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usb_e0_rme` reader - RME"]
pub type UsbE0RmeR = crate::BitReader;
#[doc = "Field `usb_e0_rme` writer - RME"]
pub type UsbE0RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb_e0_rm` reader - RM"]
pub type UsbE0RmR = crate::FieldReader;
#[doc = "Field `usb_e0_rm` writer - RM"]
pub type UsbE0RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `usb_e0_test1` reader - TEST1"]
pub type UsbE0Test1R = crate::BitReader;
#[doc = "Field `usb_e0_test1` writer - TEST1"]
pub type UsbE0Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_usb_e0_pwr_act(&self) -> GrpUsbE0PwrActR {
        GrpUsbE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_usb_e0_pwr_lps(&self) -> GrpUsbE0PwrLpsR {
        GrpUsbE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_usb_e0_loop_mode(&self) -> GrpUsbE0LoopModeR {
        GrpUsbE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_usb_e0_msel(&self) -> GrpUsbE0MselR {
        GrpUsbE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn usb_e0_rme(&self) -> UsbE0RmeR {
        UsbE0RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn usb_e0_rm(&self) -> UsbE0RmR {
        UsbE0RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn usb_e0_test1(&self) -> UsbE0Test1R {
        UsbE0Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_usb_e0_pwr_act(&mut self) -> GrpUsbE0PwrActW<'_, GrpUsbE0CtrlSpec> {
        GrpUsbE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_usb_e0_pwr_lps(&mut self) -> GrpUsbE0PwrLpsW<'_, GrpUsbE0CtrlSpec> {
        GrpUsbE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_usb_e0_loop_mode(&mut self) -> GrpUsbE0LoopModeW<'_, GrpUsbE0CtrlSpec> {
        GrpUsbE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_usb_e0_msel(&mut self) -> GrpUsbE0MselW<'_, GrpUsbE0CtrlSpec> {
        GrpUsbE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn usb_e0_rme(&mut self) -> UsbE0RmeW<'_, GrpUsbE0CtrlSpec> {
        UsbE0RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn usb_e0_rm(&mut self) -> UsbE0RmW<'_, GrpUsbE0CtrlSpec> {
        UsbE0RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn usb_e0_test1(&mut self) -> UsbE0Test1W<'_, GrpUsbE0CtrlSpec> {
        UsbE0Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_usb_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_usb_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpUsbE0CtrlSpec;
impl crate::RegisterSpec for GrpUsbE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_usb_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpUsbE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_usb_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpUsbE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_USB_E0_CTRL to value 0x1310"]
impl crate::Resettable for GrpUsbE0CtrlSpec {
    const RESET_VALUE: u32 = 0x1310;
}
