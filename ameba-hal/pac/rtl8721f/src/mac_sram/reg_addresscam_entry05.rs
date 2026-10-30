#[doc = "Register `REG_ADDRESSCAM_ENTRY05` reader"]
pub type R = crate::R<RegAddresscamEntry05Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY05` writer"]
pub type W = crate::W<RegAddresscamEntry05Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY05_A3` reader - "]
pub type AddresscamEntry05A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_A3` writer - "]
pub type AddresscamEntry05A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY05_A2` reader - "]
pub type AddresscamEntry05A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_A2` writer - "]
pub type AddresscamEntry05A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY05_A1` reader - "]
pub type AddresscamEntry05A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_A1` writer - "]
pub type AddresscamEntry05A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY05_UC` reader - "]
pub type AddresscamEntry05UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_UC` writer - "]
pub type AddresscamEntry05UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_MC` reader - "]
pub type AddresscamEntry05McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_MC` writer - "]
pub type AddresscamEntry05McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_BC` reader - "]
pub type AddresscamEntry05BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_BC` writer - "]
pub type AddresscamEntry05BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_SUBTYPE` reader - "]
pub type AddresscamEntry05SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_SUBTYPE` writer - "]
pub type AddresscamEntry05SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY05_CTRL` reader - "]
pub type AddresscamEntry05CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_CTRL` writer - "]
pub type AddresscamEntry05CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_MGNT` reader - "]
pub type AddresscamEntry05MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_MGNT` writer - "]
pub type AddresscamEntry05MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_DATA` reader - "]
pub type AddresscamEntry05DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_DATA` writer - "]
pub type AddresscamEntry05DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_RELAY` reader - "]
pub type AddresscamEntry05RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_RELAY` writer - "]
pub type AddresscamEntry05RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_UL_RELAY` reader - "]
pub type AddresscamEntry05UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_UL_RELAY` writer - "]
pub type AddresscamEntry05UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_NORMAL` reader - "]
pub type AddresscamEntry05NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_NORMAL` writer - "]
pub type AddresscamEntry05NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_A2_UNMATCH` reader - "]
pub type AddresscamEntry05A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_A2_UNMATCH` writer - "]
pub type AddresscamEntry05A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry05SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry05SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY05_VALID` reader - "]
pub type AddresscamEntry05ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY05_VALID` writer - "]
pub type AddresscamEntry05ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry05_a3(&self) -> AddresscamEntry05A3R {
        AddresscamEntry05A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry05_a2(&self) -> AddresscamEntry05A2R {
        AddresscamEntry05A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry05_a1(&self) -> AddresscamEntry05A1R {
        AddresscamEntry05A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry05_uc(&self) -> AddresscamEntry05UcR {
        AddresscamEntry05UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry05_mc(&self) -> AddresscamEntry05McR {
        AddresscamEntry05McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry05_bc(&self) -> AddresscamEntry05BcR {
        AddresscamEntry05BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry05_subtype(&self) -> AddresscamEntry05SubtypeR {
        AddresscamEntry05SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry05_ctrl(&self) -> AddresscamEntry05CtrlR {
        AddresscamEntry05CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry05_mgnt(&self) -> AddresscamEntry05MgntR {
        AddresscamEntry05MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry05_data(&self) -> AddresscamEntry05DataR {
        AddresscamEntry05DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry05_relay(&self) -> AddresscamEntry05RelayR {
        AddresscamEntry05RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry05_ul_relay(&self) -> AddresscamEntry05UlRelayR {
        AddresscamEntry05UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry05_normal(&self) -> AddresscamEntry05NormalR {
        AddresscamEntry05NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry05_a2_unmatch(&self) -> AddresscamEntry05A2UnmatchR {
        AddresscamEntry05A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry05_subtype_mask(&self) -> AddresscamEntry05SubtypeMaskR {
        AddresscamEntry05SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry05_valid(&self) -> AddresscamEntry05ValidR {
        AddresscamEntry05ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry05_a3(&mut self) -> AddresscamEntry05A3W<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry05_a2(&mut self) -> AddresscamEntry05A2W<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry05_a1(&mut self) -> AddresscamEntry05A1W<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry05_uc(&mut self) -> AddresscamEntry05UcW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry05_mc(&mut self) -> AddresscamEntry05McW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry05_bc(&mut self) -> AddresscamEntry05BcW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry05_subtype(
        &mut self,
    ) -> AddresscamEntry05SubtypeW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry05_ctrl(
        &mut self,
    ) -> AddresscamEntry05CtrlW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry05_mgnt(
        &mut self,
    ) -> AddresscamEntry05MgntW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry05_data(
        &mut self,
    ) -> AddresscamEntry05DataW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry05_relay(
        &mut self,
    ) -> AddresscamEntry05RelayW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry05_ul_relay(
        &mut self,
    ) -> AddresscamEntry05UlRelayW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry05_normal(
        &mut self,
    ) -> AddresscamEntry05NormalW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry05_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry05A2UnmatchW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry05_subtype_mask(
        &mut self,
    ) -> AddresscamEntry05SubtypeMaskW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry05_valid(
        &mut self,
    ) -> AddresscamEntry05ValidW<'_, RegAddresscamEntry05Spec> {
        AddresscamEntry05ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry05::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry05::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry05Spec;
impl crate::RegisterSpec for RegAddresscamEntry05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry05::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry05Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry05::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry05Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY05 to value 0"]
impl crate::Resettable for RegAddresscamEntry05Spec {}
