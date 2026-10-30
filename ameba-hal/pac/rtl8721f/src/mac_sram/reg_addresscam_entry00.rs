#[doc = "Register `REG_ADDRESSCAM_ENTRY00` reader"]
pub type R = crate::R<RegAddresscamEntry00Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY00` writer"]
pub type W = crate::W<RegAddresscamEntry00Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY00_A3` reader - "]
pub type AddresscamEntry00A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_A3` writer - "]
pub type AddresscamEntry00A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY00_A2` reader - "]
pub type AddresscamEntry00A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_A2` writer - "]
pub type AddresscamEntry00A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY00_A1` reader - "]
pub type AddresscamEntry00A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_A1` writer - "]
pub type AddresscamEntry00A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY00_UC` reader - "]
pub type AddresscamEntry00UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_UC` writer - "]
pub type AddresscamEntry00UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_MC` reader - "]
pub type AddresscamEntry00McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_MC` writer - "]
pub type AddresscamEntry00McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_BC` reader - "]
pub type AddresscamEntry00BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_BC` writer - "]
pub type AddresscamEntry00BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_SUBTYPE` reader - "]
pub type AddresscamEntry00SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_SUBTYPE` writer - "]
pub type AddresscamEntry00SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY00_CTRL` reader - "]
pub type AddresscamEntry00CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_CTRL` writer - "]
pub type AddresscamEntry00CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_MGNT` reader - "]
pub type AddresscamEntry00MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_MGNT` writer - "]
pub type AddresscamEntry00MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_DATA` reader - "]
pub type AddresscamEntry00DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_DATA` writer - "]
pub type AddresscamEntry00DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_RELAY` reader - "]
pub type AddresscamEntry00RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_RELAY` writer - "]
pub type AddresscamEntry00RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_UL_RELAY` reader - "]
pub type AddresscamEntry00UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_UL_RELAY` writer - "]
pub type AddresscamEntry00UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_NORMAL` reader - "]
pub type AddresscamEntry00NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_NORMAL` writer - "]
pub type AddresscamEntry00NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_A2_UNMATCH` reader - "]
pub type AddresscamEntry00A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_A2_UNMATCH` writer - "]
pub type AddresscamEntry00A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry00SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry00SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY00_VALID` reader - "]
pub type AddresscamEntry00ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY00_VALID` writer - "]
pub type AddresscamEntry00ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry00_a3(&self) -> AddresscamEntry00A3R {
        AddresscamEntry00A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry00_a2(&self) -> AddresscamEntry00A2R {
        AddresscamEntry00A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry00_a1(&self) -> AddresscamEntry00A1R {
        AddresscamEntry00A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry00_uc(&self) -> AddresscamEntry00UcR {
        AddresscamEntry00UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry00_mc(&self) -> AddresscamEntry00McR {
        AddresscamEntry00McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry00_bc(&self) -> AddresscamEntry00BcR {
        AddresscamEntry00BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry00_subtype(&self) -> AddresscamEntry00SubtypeR {
        AddresscamEntry00SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry00_ctrl(&self) -> AddresscamEntry00CtrlR {
        AddresscamEntry00CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry00_mgnt(&self) -> AddresscamEntry00MgntR {
        AddresscamEntry00MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry00_data(&self) -> AddresscamEntry00DataR {
        AddresscamEntry00DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry00_relay(&self) -> AddresscamEntry00RelayR {
        AddresscamEntry00RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry00_ul_relay(&self) -> AddresscamEntry00UlRelayR {
        AddresscamEntry00UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry00_normal(&self) -> AddresscamEntry00NormalR {
        AddresscamEntry00NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry00_a2_unmatch(&self) -> AddresscamEntry00A2UnmatchR {
        AddresscamEntry00A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry00_subtype_mask(&self) -> AddresscamEntry00SubtypeMaskR {
        AddresscamEntry00SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry00_valid(&self) -> AddresscamEntry00ValidR {
        AddresscamEntry00ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry00_a3(&mut self) -> AddresscamEntry00A3W<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry00_a2(&mut self) -> AddresscamEntry00A2W<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry00_a1(&mut self) -> AddresscamEntry00A1W<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry00_uc(&mut self) -> AddresscamEntry00UcW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry00_mc(&mut self) -> AddresscamEntry00McW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry00_bc(&mut self) -> AddresscamEntry00BcW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry00_subtype(
        &mut self,
    ) -> AddresscamEntry00SubtypeW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry00_ctrl(
        &mut self,
    ) -> AddresscamEntry00CtrlW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry00_mgnt(
        &mut self,
    ) -> AddresscamEntry00MgntW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry00_data(
        &mut self,
    ) -> AddresscamEntry00DataW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry00_relay(
        &mut self,
    ) -> AddresscamEntry00RelayW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry00_ul_relay(
        &mut self,
    ) -> AddresscamEntry00UlRelayW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry00_normal(
        &mut self,
    ) -> AddresscamEntry00NormalW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry00_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry00A2UnmatchW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry00_subtype_mask(
        &mut self,
    ) -> AddresscamEntry00SubtypeMaskW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry00_valid(
        &mut self,
    ) -> AddresscamEntry00ValidW<'_, RegAddresscamEntry00Spec> {
        AddresscamEntry00ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry00Spec;
impl crate::RegisterSpec for RegAddresscamEntry00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry00::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry00Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry00::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY00 to value 0"]
impl crate::Resettable for RegAddresscamEntry00Spec {}
