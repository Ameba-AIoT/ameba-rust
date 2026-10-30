#[doc = "Register `REG_ADDRESSCAM_ENTRY08` reader"]
pub type R = crate::R<RegAddresscamEntry08Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY08` writer"]
pub type W = crate::W<RegAddresscamEntry08Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY08_A3` reader - "]
pub type AddresscamEntry08A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_A3` writer - "]
pub type AddresscamEntry08A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY08_A2` reader - "]
pub type AddresscamEntry08A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_A2` writer - "]
pub type AddresscamEntry08A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY08_A1` reader - "]
pub type AddresscamEntry08A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_A1` writer - "]
pub type AddresscamEntry08A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY08_UC` reader - "]
pub type AddresscamEntry08UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_UC` writer - "]
pub type AddresscamEntry08UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_MC` reader - "]
pub type AddresscamEntry08McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_MC` writer - "]
pub type AddresscamEntry08McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_BC` reader - "]
pub type AddresscamEntry08BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_BC` writer - "]
pub type AddresscamEntry08BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_SUBTYPE` reader - "]
pub type AddresscamEntry08SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_SUBTYPE` writer - "]
pub type AddresscamEntry08SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY08_CTRL` reader - "]
pub type AddresscamEntry08CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_CTRL` writer - "]
pub type AddresscamEntry08CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_MGNT` reader - "]
pub type AddresscamEntry08MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_MGNT` writer - "]
pub type AddresscamEntry08MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_DATA` reader - "]
pub type AddresscamEntry08DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_DATA` writer - "]
pub type AddresscamEntry08DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_RELAY` reader - "]
pub type AddresscamEntry08RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_RELAY` writer - "]
pub type AddresscamEntry08RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_UL_RELAY` reader - "]
pub type AddresscamEntry08UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_UL_RELAY` writer - "]
pub type AddresscamEntry08UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_NORMAL` reader - "]
pub type AddresscamEntry08NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_NORMAL` writer - "]
pub type AddresscamEntry08NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_A2_UNMATCH` reader - "]
pub type AddresscamEntry08A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_A2_UNMATCH` writer - "]
pub type AddresscamEntry08A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry08SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry08SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY08_VALID` reader - "]
pub type AddresscamEntry08ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY08_VALID` writer - "]
pub type AddresscamEntry08ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry08_a3(&self) -> AddresscamEntry08A3R {
        AddresscamEntry08A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry08_a2(&self) -> AddresscamEntry08A2R {
        AddresscamEntry08A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry08_a1(&self) -> AddresscamEntry08A1R {
        AddresscamEntry08A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry08_uc(&self) -> AddresscamEntry08UcR {
        AddresscamEntry08UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry08_mc(&self) -> AddresscamEntry08McR {
        AddresscamEntry08McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry08_bc(&self) -> AddresscamEntry08BcR {
        AddresscamEntry08BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry08_subtype(&self) -> AddresscamEntry08SubtypeR {
        AddresscamEntry08SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry08_ctrl(&self) -> AddresscamEntry08CtrlR {
        AddresscamEntry08CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry08_mgnt(&self) -> AddresscamEntry08MgntR {
        AddresscamEntry08MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry08_data(&self) -> AddresscamEntry08DataR {
        AddresscamEntry08DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry08_relay(&self) -> AddresscamEntry08RelayR {
        AddresscamEntry08RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry08_ul_relay(&self) -> AddresscamEntry08UlRelayR {
        AddresscamEntry08UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry08_normal(&self) -> AddresscamEntry08NormalR {
        AddresscamEntry08NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry08_a2_unmatch(&self) -> AddresscamEntry08A2UnmatchR {
        AddresscamEntry08A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry08_subtype_mask(&self) -> AddresscamEntry08SubtypeMaskR {
        AddresscamEntry08SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry08_valid(&self) -> AddresscamEntry08ValidR {
        AddresscamEntry08ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry08_a3(&mut self) -> AddresscamEntry08A3W<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry08_a2(&mut self) -> AddresscamEntry08A2W<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry08_a1(&mut self) -> AddresscamEntry08A1W<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry08_uc(&mut self) -> AddresscamEntry08UcW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry08_mc(&mut self) -> AddresscamEntry08McW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry08_bc(&mut self) -> AddresscamEntry08BcW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry08_subtype(
        &mut self,
    ) -> AddresscamEntry08SubtypeW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry08_ctrl(
        &mut self,
    ) -> AddresscamEntry08CtrlW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry08_mgnt(
        &mut self,
    ) -> AddresscamEntry08MgntW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry08_data(
        &mut self,
    ) -> AddresscamEntry08DataW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry08_relay(
        &mut self,
    ) -> AddresscamEntry08RelayW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry08_ul_relay(
        &mut self,
    ) -> AddresscamEntry08UlRelayW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry08_normal(
        &mut self,
    ) -> AddresscamEntry08NormalW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry08_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry08A2UnmatchW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry08_subtype_mask(
        &mut self,
    ) -> AddresscamEntry08SubtypeMaskW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry08_valid(
        &mut self,
    ) -> AddresscamEntry08ValidW<'_, RegAddresscamEntry08Spec> {
        AddresscamEntry08ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry08Spec;
impl crate::RegisterSpec for RegAddresscamEntry08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry08::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry08Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry08::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY08 to value 0"]
impl crate::Resettable for RegAddresscamEntry08Spec {}
