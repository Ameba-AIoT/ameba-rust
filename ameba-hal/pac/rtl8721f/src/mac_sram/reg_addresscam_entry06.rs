#[doc = "Register `REG_ADDRESSCAM_ENTRY06` reader"]
pub type R = crate::R<RegAddresscamEntry06Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY06` writer"]
pub type W = crate::W<RegAddresscamEntry06Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY06_A3` reader - "]
pub type AddresscamEntry06A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_A3` writer - "]
pub type AddresscamEntry06A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY06_A2` reader - "]
pub type AddresscamEntry06A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_A2` writer - "]
pub type AddresscamEntry06A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY06_A1` reader - "]
pub type AddresscamEntry06A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_A1` writer - "]
pub type AddresscamEntry06A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY06_UC` reader - "]
pub type AddresscamEntry06UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_UC` writer - "]
pub type AddresscamEntry06UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_MC` reader - "]
pub type AddresscamEntry06McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_MC` writer - "]
pub type AddresscamEntry06McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_BC` reader - "]
pub type AddresscamEntry06BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_BC` writer - "]
pub type AddresscamEntry06BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_SUBTYPE` reader - "]
pub type AddresscamEntry06SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_SUBTYPE` writer - "]
pub type AddresscamEntry06SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY06_CTRL` reader - "]
pub type AddresscamEntry06CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_CTRL` writer - "]
pub type AddresscamEntry06CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_MGNT` reader - "]
pub type AddresscamEntry06MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_MGNT` writer - "]
pub type AddresscamEntry06MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_DATA` reader - "]
pub type AddresscamEntry06DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_DATA` writer - "]
pub type AddresscamEntry06DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_RELAY` reader - "]
pub type AddresscamEntry06RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_RELAY` writer - "]
pub type AddresscamEntry06RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_UL_RELAY` reader - "]
pub type AddresscamEntry06UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_UL_RELAY` writer - "]
pub type AddresscamEntry06UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_NORMAL` reader - "]
pub type AddresscamEntry06NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_NORMAL` writer - "]
pub type AddresscamEntry06NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_A2_UNMATCH` reader - "]
pub type AddresscamEntry06A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_A2_UNMATCH` writer - "]
pub type AddresscamEntry06A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry06SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry06SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY06_VALID` reader - "]
pub type AddresscamEntry06ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY06_VALID` writer - "]
pub type AddresscamEntry06ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry06_a3(&self) -> AddresscamEntry06A3R {
        AddresscamEntry06A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry06_a2(&self) -> AddresscamEntry06A2R {
        AddresscamEntry06A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry06_a1(&self) -> AddresscamEntry06A1R {
        AddresscamEntry06A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry06_uc(&self) -> AddresscamEntry06UcR {
        AddresscamEntry06UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry06_mc(&self) -> AddresscamEntry06McR {
        AddresscamEntry06McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry06_bc(&self) -> AddresscamEntry06BcR {
        AddresscamEntry06BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry06_subtype(&self) -> AddresscamEntry06SubtypeR {
        AddresscamEntry06SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry06_ctrl(&self) -> AddresscamEntry06CtrlR {
        AddresscamEntry06CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry06_mgnt(&self) -> AddresscamEntry06MgntR {
        AddresscamEntry06MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry06_data(&self) -> AddresscamEntry06DataR {
        AddresscamEntry06DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry06_relay(&self) -> AddresscamEntry06RelayR {
        AddresscamEntry06RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry06_ul_relay(&self) -> AddresscamEntry06UlRelayR {
        AddresscamEntry06UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry06_normal(&self) -> AddresscamEntry06NormalR {
        AddresscamEntry06NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry06_a2_unmatch(&self) -> AddresscamEntry06A2UnmatchR {
        AddresscamEntry06A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry06_subtype_mask(&self) -> AddresscamEntry06SubtypeMaskR {
        AddresscamEntry06SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry06_valid(&self) -> AddresscamEntry06ValidR {
        AddresscamEntry06ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry06_a3(&mut self) -> AddresscamEntry06A3W<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry06_a2(&mut self) -> AddresscamEntry06A2W<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry06_a1(&mut self) -> AddresscamEntry06A1W<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry06_uc(&mut self) -> AddresscamEntry06UcW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry06_mc(&mut self) -> AddresscamEntry06McW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry06_bc(&mut self) -> AddresscamEntry06BcW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry06_subtype(
        &mut self,
    ) -> AddresscamEntry06SubtypeW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry06_ctrl(
        &mut self,
    ) -> AddresscamEntry06CtrlW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry06_mgnt(
        &mut self,
    ) -> AddresscamEntry06MgntW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry06_data(
        &mut self,
    ) -> AddresscamEntry06DataW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry06_relay(
        &mut self,
    ) -> AddresscamEntry06RelayW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry06_ul_relay(
        &mut self,
    ) -> AddresscamEntry06UlRelayW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry06_normal(
        &mut self,
    ) -> AddresscamEntry06NormalW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry06_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry06A2UnmatchW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry06_subtype_mask(
        &mut self,
    ) -> AddresscamEntry06SubtypeMaskW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry06_valid(
        &mut self,
    ) -> AddresscamEntry06ValidW<'_, RegAddresscamEntry06Spec> {
        AddresscamEntry06ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry06Spec;
impl crate::RegisterSpec for RegAddresscamEntry06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry06::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry06Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry06::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry06Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY06 to value 0"]
impl crate::Resettable for RegAddresscamEntry06Spec {}
