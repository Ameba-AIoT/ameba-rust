#[doc = "Register `REG_WL_BTCOEX_CTRL` reader"]
pub type R = crate::R<RegWlBtcoexCtrlSpec>;
#[doc = "Register `REG_WL_BTCOEX_CTRL` writer"]
pub type W = crate::W<RegWlBtcoexCtrlSpec>;
#[doc = "Field `BIT_EnBT_V1` reader - Enable PTA ( from BT side)"]
pub type BitEnBtV1R = crate::BitReader;
#[doc = "Field `BIT_EnBT_V1` writer - Enable PTA ( from BT side)"]
pub type BitEnBtV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BTMode_V1` reader - Bluetooth Mode"]
pub type BitBtmodeV1R = crate::FieldReader;
#[doc = "Field `BIT_BTMode_V1` writer - Bluetooth Mode"]
pub type BitBtmodeV1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 16 - Enable PTA ( from BT side)"]
    #[inline(always)]
    pub fn bit_en_bt_v1(&self) -> BitEnBtV1R {
        BitEnBtV1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Bluetooth Mode"]
    #[inline(always)]
    pub fn bit_btmode_v1(&self) -> BitBtmodeV1R {
        BitBtmodeV1R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Enable PTA ( from BT side)"]
    #[inline(always)]
    pub fn bit_en_bt_v1(&mut self) -> BitEnBtV1W<'_, RegWlBtcoexCtrlSpec> {
        BitEnBtV1W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Bluetooth Mode"]
    #[inline(always)]
    pub fn bit_btmode_v1(&mut self) -> BitBtmodeV1W<'_, RegWlBtcoexCtrlSpec> {
        BitBtmodeV1W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wl_btcoex_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wl_btcoex_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlBtcoexCtrlSpec;
impl crate::RegisterSpec for RegWlBtcoexCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wl_btcoex_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWlBtcoexCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wl_btcoex_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWlBtcoexCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WL_BTCOEX_CTRL to value 0"]
impl crate::Resettable for RegWlBtcoexCtrlSpec {}
