#[doc = "Register `REG_ADDRESSCAM_ENTRY04` reader"]
pub type R = crate::R<RegAddresscamEntry04Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY04` writer"]
pub type W = crate::W<RegAddresscamEntry04Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY04_A3` reader - "]
pub type AddresscamEntry04A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_A3` writer - "]
pub type AddresscamEntry04A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY04_A2` reader - "]
pub type AddresscamEntry04A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_A2` writer - "]
pub type AddresscamEntry04A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY04_A1` reader - "]
pub type AddresscamEntry04A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_A1` writer - "]
pub type AddresscamEntry04A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY04_UC` reader - "]
pub type AddresscamEntry04UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_UC` writer - "]
pub type AddresscamEntry04UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_MC` reader - "]
pub type AddresscamEntry04McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_MC` writer - "]
pub type AddresscamEntry04McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_BC` reader - "]
pub type AddresscamEntry04BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_BC` writer - "]
pub type AddresscamEntry04BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_SUBTYPE` reader - "]
pub type AddresscamEntry04SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_SUBTYPE` writer - "]
pub type AddresscamEntry04SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY04_CTRL` reader - "]
pub type AddresscamEntry04CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_CTRL` writer - "]
pub type AddresscamEntry04CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_MGNT` reader - "]
pub type AddresscamEntry04MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_MGNT` writer - "]
pub type AddresscamEntry04MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_DATA` reader - "]
pub type AddresscamEntry04DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_DATA` writer - "]
pub type AddresscamEntry04DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_RELAY` reader - "]
pub type AddresscamEntry04RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_RELAY` writer - "]
pub type AddresscamEntry04RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_UL_RELAY` reader - "]
pub type AddresscamEntry04UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_UL_RELAY` writer - "]
pub type AddresscamEntry04UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_NORMAL` reader - "]
pub type AddresscamEntry04NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_NORMAL` writer - "]
pub type AddresscamEntry04NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_A2_UNMATCH` reader - "]
pub type AddresscamEntry04A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_A2_UNMATCH` writer - "]
pub type AddresscamEntry04A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry04SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry04SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY04_VALID` reader - "]
pub type AddresscamEntry04ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY04_VALID` writer - "]
pub type AddresscamEntry04ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry04_a3(&self) -> AddresscamEntry04A3R {
        AddresscamEntry04A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry04_a2(&self) -> AddresscamEntry04A2R {
        AddresscamEntry04A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry04_a1(&self) -> AddresscamEntry04A1R {
        AddresscamEntry04A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry04_uc(&self) -> AddresscamEntry04UcR {
        AddresscamEntry04UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry04_mc(&self) -> AddresscamEntry04McR {
        AddresscamEntry04McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry04_bc(&self) -> AddresscamEntry04BcR {
        AddresscamEntry04BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry04_subtype(&self) -> AddresscamEntry04SubtypeR {
        AddresscamEntry04SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry04_ctrl(&self) -> AddresscamEntry04CtrlR {
        AddresscamEntry04CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry04_mgnt(&self) -> AddresscamEntry04MgntR {
        AddresscamEntry04MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry04_data(&self) -> AddresscamEntry04DataR {
        AddresscamEntry04DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry04_relay(&self) -> AddresscamEntry04RelayR {
        AddresscamEntry04RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry04_ul_relay(&self) -> AddresscamEntry04UlRelayR {
        AddresscamEntry04UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry04_normal(&self) -> AddresscamEntry04NormalR {
        AddresscamEntry04NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry04_a2_unmatch(&self) -> AddresscamEntry04A2UnmatchR {
        AddresscamEntry04A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry04_subtype_mask(&self) -> AddresscamEntry04SubtypeMaskR {
        AddresscamEntry04SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry04_valid(&self) -> AddresscamEntry04ValidR {
        AddresscamEntry04ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry04_a3(&mut self) -> AddresscamEntry04A3W<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry04_a2(&mut self) -> AddresscamEntry04A2W<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry04_a1(&mut self) -> AddresscamEntry04A1W<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry04_uc(&mut self) -> AddresscamEntry04UcW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry04_mc(&mut self) -> AddresscamEntry04McW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry04_bc(&mut self) -> AddresscamEntry04BcW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry04_subtype(
        &mut self,
    ) -> AddresscamEntry04SubtypeW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry04_ctrl(
        &mut self,
    ) -> AddresscamEntry04CtrlW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry04_mgnt(
        &mut self,
    ) -> AddresscamEntry04MgntW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry04_data(
        &mut self,
    ) -> AddresscamEntry04DataW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry04_relay(
        &mut self,
    ) -> AddresscamEntry04RelayW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry04_ul_relay(
        &mut self,
    ) -> AddresscamEntry04UlRelayW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry04_normal(
        &mut self,
    ) -> AddresscamEntry04NormalW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry04_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry04A2UnmatchW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry04_subtype_mask(
        &mut self,
    ) -> AddresscamEntry04SubtypeMaskW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry04_valid(
        &mut self,
    ) -> AddresscamEntry04ValidW<'_, RegAddresscamEntry04Spec> {
        AddresscamEntry04ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry04Spec;
impl crate::RegisterSpec for RegAddresscamEntry04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry04::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry04Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry04::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY04 to value 0"]
impl crate::Resettable for RegAddresscamEntry04Spec {}
