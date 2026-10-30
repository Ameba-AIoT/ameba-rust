#[doc = "Register `LCDC_RGB_SYNC_STATUS` reader"]
pub type R = crate::R<LcdcRgbSyncStatusSpec>;
#[doc = "Field `hsstatus` reader - HSYNC Status. 00: HSYNC 01: HBP 11: ACTIVE 10: HFP"]
pub type HsstatusR = crate::FieldReader;
#[doc = "Field `vsstatus` reader - VSYNC Status. 00: VSYNC 01: VBP 11: ACTIVE 10: VFP"]
pub type VsstatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - HSYNC Status. 00: HSYNC 01: HBP 11: ACTIVE 10: HFP"]
    #[inline(always)]
    pub fn hsstatus(&self) -> HsstatusR {
        HsstatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - VSYNC Status. 00: VSYNC 01: VBP 11: ACTIVE 10: VFP"]
    #[inline(always)]
    pub fn vsstatus(&self) -> VsstatusR {
        VsstatusR::new(((self.bits >> 2) & 3) as u8)
    }
}
#[doc = "RGB HSYNC/VSYNC status\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_rgb_sync_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcRgbSyncStatusSpec;
impl crate::RegisterSpec for LcdcRgbSyncStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_rgb_sync_status::R`](R) reader structure"]
impl crate::Readable for LcdcRgbSyncStatusSpec {}
#[doc = "`reset()` method sets LCDC_RGB_SYNC_STATUS to value 0"]
impl crate::Resettable for LcdcRgbSyncStatusSpec {}
