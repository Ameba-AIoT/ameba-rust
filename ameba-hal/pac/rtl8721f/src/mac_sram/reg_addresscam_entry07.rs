#[doc = "Register `REG_ADDRESSCAM_ENTRY07` reader"]
pub type R = crate::R<RegAddresscamEntry07Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY07` writer"]
pub type W = crate::W<RegAddresscamEntry07Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY07_A3` reader - "]
pub type AddresscamEntry07A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_A3` writer - "]
pub type AddresscamEntry07A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY07_A2` reader - "]
pub type AddresscamEntry07A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_A2` writer - "]
pub type AddresscamEntry07A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY07_A1` reader - "]
pub type AddresscamEntry07A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_A1` writer - "]
pub type AddresscamEntry07A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY07_UC` reader - "]
pub type AddresscamEntry07UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_UC` writer - "]
pub type AddresscamEntry07UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_MC` reader - "]
pub type AddresscamEntry07McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_MC` writer - "]
pub type AddresscamEntry07McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_BC` reader - "]
pub type AddresscamEntry07BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_BC` writer - "]
pub type AddresscamEntry07BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_SUBTYPE` reader - "]
pub type AddresscamEntry07SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_SUBTYPE` writer - "]
pub type AddresscamEntry07SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY07_CTRL` reader - "]
pub type AddresscamEntry07CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_CTRL` writer - "]
pub type AddresscamEntry07CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_MGNT` reader - "]
pub type AddresscamEntry07MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_MGNT` writer - "]
pub type AddresscamEntry07MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_DATA` reader - "]
pub type AddresscamEntry07DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_DATA` writer - "]
pub type AddresscamEntry07DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_RELAY` reader - "]
pub type AddresscamEntry07RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_RELAY` writer - "]
pub type AddresscamEntry07RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_UL_RELAY` reader - "]
pub type AddresscamEntry07UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_UL_RELAY` writer - "]
pub type AddresscamEntry07UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_NORMAL` reader - "]
pub type AddresscamEntry07NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_NORMAL` writer - "]
pub type AddresscamEntry07NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_A2_UNMATCH` reader - "]
pub type AddresscamEntry07A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_A2_UNMATCH` writer - "]
pub type AddresscamEntry07A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry07SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry07SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY07_VALID` reader - "]
pub type AddresscamEntry07ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY07_VALID` writer - "]
pub type AddresscamEntry07ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry07_a3(&self) -> AddresscamEntry07A3R {
        AddresscamEntry07A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry07_a2(&self) -> AddresscamEntry07A2R {
        AddresscamEntry07A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry07_a1(&self) -> AddresscamEntry07A1R {
        AddresscamEntry07A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry07_uc(&self) -> AddresscamEntry07UcR {
        AddresscamEntry07UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry07_mc(&self) -> AddresscamEntry07McR {
        AddresscamEntry07McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry07_bc(&self) -> AddresscamEntry07BcR {
        AddresscamEntry07BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry07_subtype(&self) -> AddresscamEntry07SubtypeR {
        AddresscamEntry07SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry07_ctrl(&self) -> AddresscamEntry07CtrlR {
        AddresscamEntry07CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry07_mgnt(&self) -> AddresscamEntry07MgntR {
        AddresscamEntry07MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry07_data(&self) -> AddresscamEntry07DataR {
        AddresscamEntry07DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry07_relay(&self) -> AddresscamEntry07RelayR {
        AddresscamEntry07RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry07_ul_relay(&self) -> AddresscamEntry07UlRelayR {
        AddresscamEntry07UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry07_normal(&self) -> AddresscamEntry07NormalR {
        AddresscamEntry07NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry07_a2_unmatch(&self) -> AddresscamEntry07A2UnmatchR {
        AddresscamEntry07A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry07_subtype_mask(&self) -> AddresscamEntry07SubtypeMaskR {
        AddresscamEntry07SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry07_valid(&self) -> AddresscamEntry07ValidR {
        AddresscamEntry07ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry07_a3(&mut self) -> AddresscamEntry07A3W<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry07_a2(&mut self) -> AddresscamEntry07A2W<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry07_a1(&mut self) -> AddresscamEntry07A1W<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry07_uc(&mut self) -> AddresscamEntry07UcW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry07_mc(&mut self) -> AddresscamEntry07McW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry07_bc(&mut self) -> AddresscamEntry07BcW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry07_subtype(
        &mut self,
    ) -> AddresscamEntry07SubtypeW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry07_ctrl(
        &mut self,
    ) -> AddresscamEntry07CtrlW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry07_mgnt(
        &mut self,
    ) -> AddresscamEntry07MgntW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry07_data(
        &mut self,
    ) -> AddresscamEntry07DataW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry07_relay(
        &mut self,
    ) -> AddresscamEntry07RelayW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry07_ul_relay(
        &mut self,
    ) -> AddresscamEntry07UlRelayW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry07_normal(
        &mut self,
    ) -> AddresscamEntry07NormalW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry07_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry07A2UnmatchW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry07_subtype_mask(
        &mut self,
    ) -> AddresscamEntry07SubtypeMaskW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry07_valid(
        &mut self,
    ) -> AddresscamEntry07ValidW<'_, RegAddresscamEntry07Spec> {
        AddresscamEntry07ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry07::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry07::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry07Spec;
impl crate::RegisterSpec for RegAddresscamEntry07Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry07::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry07Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry07::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry07Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY07 to value 0"]
impl crate::Resettable for RegAddresscamEntry07Spec {}
