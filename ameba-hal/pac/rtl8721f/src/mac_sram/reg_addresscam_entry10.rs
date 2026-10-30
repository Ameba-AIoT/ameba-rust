#[doc = "Register `REG_ADDRESSCAM_ENTRY10` reader"]
pub type R = crate::R<RegAddresscamEntry10Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY10` writer"]
pub type W = crate::W<RegAddresscamEntry10Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY10_A3` reader - "]
pub type AddresscamEntry10A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_A3` writer - "]
pub type AddresscamEntry10A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY10_A2` reader - "]
pub type AddresscamEntry10A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_A2` writer - "]
pub type AddresscamEntry10A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY10_A1` reader - "]
pub type AddresscamEntry10A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_A1` writer - "]
pub type AddresscamEntry10A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY10_UC` reader - "]
pub type AddresscamEntry10UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_UC` writer - "]
pub type AddresscamEntry10UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_MC` reader - "]
pub type AddresscamEntry10McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_MC` writer - "]
pub type AddresscamEntry10McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_BC` reader - "]
pub type AddresscamEntry10BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_BC` writer - "]
pub type AddresscamEntry10BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_SUBTYPE` reader - "]
pub type AddresscamEntry10SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_SUBTYPE` writer - "]
pub type AddresscamEntry10SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY10_CTRL` reader - "]
pub type AddresscamEntry10CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_CTRL` writer - "]
pub type AddresscamEntry10CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_MGNT` reader - "]
pub type AddresscamEntry10MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_MGNT` writer - "]
pub type AddresscamEntry10MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_DATA` reader - "]
pub type AddresscamEntry10DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_DATA` writer - "]
pub type AddresscamEntry10DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_RELAY` reader - "]
pub type AddresscamEntry10RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_RELAY` writer - "]
pub type AddresscamEntry10RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_UL_RELAY` reader - "]
pub type AddresscamEntry10UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_UL_RELAY` writer - "]
pub type AddresscamEntry10UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_NORMAL` reader - "]
pub type AddresscamEntry10NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_NORMAL` writer - "]
pub type AddresscamEntry10NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_A2_UNMATCH` reader - "]
pub type AddresscamEntry10A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_A2_UNMATCH` writer - "]
pub type AddresscamEntry10A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry10SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry10SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY10_VALID` reader - "]
pub type AddresscamEntry10ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY10_VALID` writer - "]
pub type AddresscamEntry10ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry10_a3(&self) -> AddresscamEntry10A3R {
        AddresscamEntry10A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry10_a2(&self) -> AddresscamEntry10A2R {
        AddresscamEntry10A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry10_a1(&self) -> AddresscamEntry10A1R {
        AddresscamEntry10A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry10_uc(&self) -> AddresscamEntry10UcR {
        AddresscamEntry10UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry10_mc(&self) -> AddresscamEntry10McR {
        AddresscamEntry10McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry10_bc(&self) -> AddresscamEntry10BcR {
        AddresscamEntry10BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry10_subtype(&self) -> AddresscamEntry10SubtypeR {
        AddresscamEntry10SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry10_ctrl(&self) -> AddresscamEntry10CtrlR {
        AddresscamEntry10CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry10_mgnt(&self) -> AddresscamEntry10MgntR {
        AddresscamEntry10MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry10_data(&self) -> AddresscamEntry10DataR {
        AddresscamEntry10DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry10_relay(&self) -> AddresscamEntry10RelayR {
        AddresscamEntry10RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry10_ul_relay(&self) -> AddresscamEntry10UlRelayR {
        AddresscamEntry10UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry10_normal(&self) -> AddresscamEntry10NormalR {
        AddresscamEntry10NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry10_a2_unmatch(&self) -> AddresscamEntry10A2UnmatchR {
        AddresscamEntry10A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry10_subtype_mask(&self) -> AddresscamEntry10SubtypeMaskR {
        AddresscamEntry10SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry10_valid(&self) -> AddresscamEntry10ValidR {
        AddresscamEntry10ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry10_a3(&mut self) -> AddresscamEntry10A3W<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry10_a2(&mut self) -> AddresscamEntry10A2W<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry10_a1(&mut self) -> AddresscamEntry10A1W<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry10_uc(&mut self) -> AddresscamEntry10UcW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry10_mc(&mut self) -> AddresscamEntry10McW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry10_bc(&mut self) -> AddresscamEntry10BcW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry10_subtype(
        &mut self,
    ) -> AddresscamEntry10SubtypeW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry10_ctrl(
        &mut self,
    ) -> AddresscamEntry10CtrlW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry10_mgnt(
        &mut self,
    ) -> AddresscamEntry10MgntW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry10_data(
        &mut self,
    ) -> AddresscamEntry10DataW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry10_relay(
        &mut self,
    ) -> AddresscamEntry10RelayW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry10_ul_relay(
        &mut self,
    ) -> AddresscamEntry10UlRelayW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry10_normal(
        &mut self,
    ) -> AddresscamEntry10NormalW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry10_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry10A2UnmatchW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry10_subtype_mask(
        &mut self,
    ) -> AddresscamEntry10SubtypeMaskW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry10_valid(
        &mut self,
    ) -> AddresscamEntry10ValidW<'_, RegAddresscamEntry10Spec> {
        AddresscamEntry10ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry10Spec;
impl crate::RegisterSpec for RegAddresscamEntry10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry10::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry10::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY10 to value 0"]
impl crate::Resettable for RegAddresscamEntry10Spec {}
