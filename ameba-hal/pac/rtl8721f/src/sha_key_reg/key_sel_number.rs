#[doc = "Register `Key_Sel_Number` reader"]
pub type R = crate::R<KeySelNumberSpec>;
#[doc = "Register `Key_Sel_Number` writer"]
pub type W = crate::W<KeySelNumberSpec>;
#[doc = "Field `Key_Sel_Number` reader - The key number determines which key to use in the key setting. HW keys: 0~31, the number can be used is determined by the number of HW keys configured. Hardwired key: 32, if enabled Secure SW key: 33 Shared SW key: 34~ , the number can be used is determined by the number of SW keys configured."]
pub type KeySelNumberR = crate::FieldReader;
#[doc = "Field `Key_Sel_Number` writer - The key number determines which key to use in the key setting. HW keys: 0~31, the number can be used is determined by the number of HW keys configured. Hardwired key: 32, if enabled Secure SW key: 33 Shared SW key: 34~ , the number can be used is determined by the number of SW keys configured."]
pub type KeySelNumberW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Set_xts_en` reader - Only available when SUPPORT_XTS_MODE = 1. When this bit is set to 1, the trigger will write the selected key to xts_key2. Otherwise, the trigger will write the selected key to key/xts_key1."]
pub type SetXtsEnR = crate::BitReader;
#[doc = "Field `Set_xts_en` writer - Only available when SUPPORT_XTS_MODE = 1. When this bit is set to 1, the trigger will write the selected key to xts_key2. Otherwise, the trigger will write the selected key to key/xts_key1."]
pub type SetXtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - The key number determines which key to use in the key setting. HW keys: 0~31, the number can be used is determined by the number of HW keys configured. Hardwired key: 32, if enabled Secure SW key: 33 Shared SW key: 34~ , the number can be used is determined by the number of SW keys configured."]
    #[inline(always)]
    pub fn key_sel_number(&self) -> KeySelNumberR {
        KeySelNumberR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Only available when SUPPORT_XTS_MODE = 1. When this bit is set to 1, the trigger will write the selected key to xts_key2. Otherwise, the trigger will write the selected key to key/xts_key1."]
    #[inline(always)]
    pub fn set_xts_en(&self) -> SetXtsEnR {
        SetXtsEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The key number determines which key to use in the key setting. HW keys: 0~31, the number can be used is determined by the number of HW keys configured. Hardwired key: 32, if enabled Secure SW key: 33 Shared SW key: 34~ , the number can be used is determined by the number of SW keys configured."]
    #[inline(always)]
    pub fn key_sel_number(&mut self) -> KeySelNumberW<'_, KeySelNumberSpec> {
        KeySelNumberW::new(self, 0)
    }
    #[doc = "Bit 8 - Only available when SUPPORT_XTS_MODE = 1. When this bit is set to 1, the trigger will write the selected key to xts_key2. Otherwise, the trigger will write the selected key to key/xts_key1."]
    #[inline(always)]
    pub fn set_xts_en(&mut self) -> SetXtsEnW<'_, KeySelNumberSpec> {
        SetXtsEnW::new(self, 8)
    }
}
#[doc = "Key Selection Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`key_sel_number::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_sel_number::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySelNumberSpec;
impl crate::RegisterSpec for KeySelNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_sel_number::R`](R) reader structure"]
impl crate::Readable for KeySelNumberSpec {}
#[doc = "`write(|w| ..)` method takes [`key_sel_number::W`](W) writer structure"]
impl crate::Writable for KeySelNumberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key_Sel_Number to value 0"]
impl crate::Resettable for KeySelNumberSpec {}
