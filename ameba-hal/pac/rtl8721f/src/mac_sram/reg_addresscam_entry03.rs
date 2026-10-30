#[doc = "Register `REG_ADDRESSCAM_ENTRY03` reader"]
pub type R = crate::R<RegAddresscamEntry03Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY03` writer"]
pub type W = crate::W<RegAddresscamEntry03Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY03_A3` reader - "]
pub type AddresscamEntry03A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_A3` writer - "]
pub type AddresscamEntry03A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY03_A2` reader - "]
pub type AddresscamEntry03A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_A2` writer - "]
pub type AddresscamEntry03A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY03_A1` reader - "]
pub type AddresscamEntry03A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_A1` writer - "]
pub type AddresscamEntry03A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY03_UC` reader - "]
pub type AddresscamEntry03UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_UC` writer - "]
pub type AddresscamEntry03UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_MC` reader - "]
pub type AddresscamEntry03McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_MC` writer - "]
pub type AddresscamEntry03McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_BC` reader - "]
pub type AddresscamEntry03BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_BC` writer - "]
pub type AddresscamEntry03BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_SUBTYPE` reader - "]
pub type AddresscamEntry03SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_SUBTYPE` writer - "]
pub type AddresscamEntry03SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY03_CTRL` reader - "]
pub type AddresscamEntry03CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_CTRL` writer - "]
pub type AddresscamEntry03CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_MGNT` reader - "]
pub type AddresscamEntry03MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_MGNT` writer - "]
pub type AddresscamEntry03MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_DATA` reader - "]
pub type AddresscamEntry03DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_DATA` writer - "]
pub type AddresscamEntry03DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_RELAY` reader - "]
pub type AddresscamEntry03RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_RELAY` writer - "]
pub type AddresscamEntry03RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_UL_RELAY` reader - "]
pub type AddresscamEntry03UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_UL_RELAY` writer - "]
pub type AddresscamEntry03UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_NORMAL` reader - "]
pub type AddresscamEntry03NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_NORMAL` writer - "]
pub type AddresscamEntry03NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_A2_UNMATCH` reader - "]
pub type AddresscamEntry03A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_A2_UNMATCH` writer - "]
pub type AddresscamEntry03A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry03SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry03SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY03_VALID` reader - "]
pub type AddresscamEntry03ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY03_VALID` writer - "]
pub type AddresscamEntry03ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry03_a3(&self) -> AddresscamEntry03A3R {
        AddresscamEntry03A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry03_a2(&self) -> AddresscamEntry03A2R {
        AddresscamEntry03A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry03_a1(&self) -> AddresscamEntry03A1R {
        AddresscamEntry03A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry03_uc(&self) -> AddresscamEntry03UcR {
        AddresscamEntry03UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry03_mc(&self) -> AddresscamEntry03McR {
        AddresscamEntry03McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry03_bc(&self) -> AddresscamEntry03BcR {
        AddresscamEntry03BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry03_subtype(&self) -> AddresscamEntry03SubtypeR {
        AddresscamEntry03SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry03_ctrl(&self) -> AddresscamEntry03CtrlR {
        AddresscamEntry03CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry03_mgnt(&self) -> AddresscamEntry03MgntR {
        AddresscamEntry03MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry03_data(&self) -> AddresscamEntry03DataR {
        AddresscamEntry03DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry03_relay(&self) -> AddresscamEntry03RelayR {
        AddresscamEntry03RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry03_ul_relay(&self) -> AddresscamEntry03UlRelayR {
        AddresscamEntry03UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry03_normal(&self) -> AddresscamEntry03NormalR {
        AddresscamEntry03NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry03_a2_unmatch(&self) -> AddresscamEntry03A2UnmatchR {
        AddresscamEntry03A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry03_subtype_mask(&self) -> AddresscamEntry03SubtypeMaskR {
        AddresscamEntry03SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry03_valid(&self) -> AddresscamEntry03ValidR {
        AddresscamEntry03ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry03_a3(&mut self) -> AddresscamEntry03A3W<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry03_a2(&mut self) -> AddresscamEntry03A2W<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry03_a1(&mut self) -> AddresscamEntry03A1W<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry03_uc(&mut self) -> AddresscamEntry03UcW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry03_mc(&mut self) -> AddresscamEntry03McW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry03_bc(&mut self) -> AddresscamEntry03BcW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry03_subtype(
        &mut self,
    ) -> AddresscamEntry03SubtypeW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry03_ctrl(
        &mut self,
    ) -> AddresscamEntry03CtrlW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry03_mgnt(
        &mut self,
    ) -> AddresscamEntry03MgntW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry03_data(
        &mut self,
    ) -> AddresscamEntry03DataW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry03_relay(
        &mut self,
    ) -> AddresscamEntry03RelayW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry03_ul_relay(
        &mut self,
    ) -> AddresscamEntry03UlRelayW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry03_normal(
        &mut self,
    ) -> AddresscamEntry03NormalW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry03_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry03A2UnmatchW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry03_subtype_mask(
        &mut self,
    ) -> AddresscamEntry03SubtypeMaskW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry03_valid(
        &mut self,
    ) -> AddresscamEntry03ValidW<'_, RegAddresscamEntry03Spec> {
        AddresscamEntry03ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry03Spec;
impl crate::RegisterSpec for RegAddresscamEntry03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry03::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry03Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry03::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry03Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY03 to value 0"]
impl crate::Resettable for RegAddresscamEntry03Spec {}
