#[doc = "Register `REG_ADDRESSCAM_ENTRY09` reader"]
pub type R = crate::R<RegAddresscamEntry09Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY09` writer"]
pub type W = crate::W<RegAddresscamEntry09Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY09_A3` reader - "]
pub type AddresscamEntry09A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_A3` writer - "]
pub type AddresscamEntry09A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY09_A2` reader - "]
pub type AddresscamEntry09A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_A2` writer - "]
pub type AddresscamEntry09A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY09_A1` reader - "]
pub type AddresscamEntry09A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_A1` writer - "]
pub type AddresscamEntry09A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY09_UC` reader - "]
pub type AddresscamEntry09UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_UC` writer - "]
pub type AddresscamEntry09UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_MC` reader - "]
pub type AddresscamEntry09McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_MC` writer - "]
pub type AddresscamEntry09McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_BC` reader - "]
pub type AddresscamEntry09BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_BC` writer - "]
pub type AddresscamEntry09BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_SUBTYPE` reader - "]
pub type AddresscamEntry09SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_SUBTYPE` writer - "]
pub type AddresscamEntry09SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY09_CTRL` reader - "]
pub type AddresscamEntry09CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_CTRL` writer - "]
pub type AddresscamEntry09CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_MGNT` reader - "]
pub type AddresscamEntry09MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_MGNT` writer - "]
pub type AddresscamEntry09MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_DATA` reader - "]
pub type AddresscamEntry09DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_DATA` writer - "]
pub type AddresscamEntry09DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_RELAY` reader - "]
pub type AddresscamEntry09RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_RELAY` writer - "]
pub type AddresscamEntry09RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_UL_RELAY` reader - "]
pub type AddresscamEntry09UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_UL_RELAY` writer - "]
pub type AddresscamEntry09UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_NORMAL` reader - "]
pub type AddresscamEntry09NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_NORMAL` writer - "]
pub type AddresscamEntry09NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_A2_UNMATCH` reader - "]
pub type AddresscamEntry09A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_A2_UNMATCH` writer - "]
pub type AddresscamEntry09A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry09SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry09SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY09_VALID` reader - "]
pub type AddresscamEntry09ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY09_VALID` writer - "]
pub type AddresscamEntry09ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry09_a3(&self) -> AddresscamEntry09A3R {
        AddresscamEntry09A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry09_a2(&self) -> AddresscamEntry09A2R {
        AddresscamEntry09A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry09_a1(&self) -> AddresscamEntry09A1R {
        AddresscamEntry09A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry09_uc(&self) -> AddresscamEntry09UcR {
        AddresscamEntry09UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry09_mc(&self) -> AddresscamEntry09McR {
        AddresscamEntry09McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry09_bc(&self) -> AddresscamEntry09BcR {
        AddresscamEntry09BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry09_subtype(&self) -> AddresscamEntry09SubtypeR {
        AddresscamEntry09SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry09_ctrl(&self) -> AddresscamEntry09CtrlR {
        AddresscamEntry09CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry09_mgnt(&self) -> AddresscamEntry09MgntR {
        AddresscamEntry09MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry09_data(&self) -> AddresscamEntry09DataR {
        AddresscamEntry09DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry09_relay(&self) -> AddresscamEntry09RelayR {
        AddresscamEntry09RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry09_ul_relay(&self) -> AddresscamEntry09UlRelayR {
        AddresscamEntry09UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry09_normal(&self) -> AddresscamEntry09NormalR {
        AddresscamEntry09NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry09_a2_unmatch(&self) -> AddresscamEntry09A2UnmatchR {
        AddresscamEntry09A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry09_subtype_mask(&self) -> AddresscamEntry09SubtypeMaskR {
        AddresscamEntry09SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry09_valid(&self) -> AddresscamEntry09ValidR {
        AddresscamEntry09ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry09_a3(&mut self) -> AddresscamEntry09A3W<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry09_a2(&mut self) -> AddresscamEntry09A2W<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry09_a1(&mut self) -> AddresscamEntry09A1W<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry09_uc(&mut self) -> AddresscamEntry09UcW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry09_mc(&mut self) -> AddresscamEntry09McW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry09_bc(&mut self) -> AddresscamEntry09BcW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry09_subtype(
        &mut self,
    ) -> AddresscamEntry09SubtypeW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry09_ctrl(
        &mut self,
    ) -> AddresscamEntry09CtrlW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry09_mgnt(
        &mut self,
    ) -> AddresscamEntry09MgntW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry09_data(
        &mut self,
    ) -> AddresscamEntry09DataW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry09_relay(
        &mut self,
    ) -> AddresscamEntry09RelayW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry09_ul_relay(
        &mut self,
    ) -> AddresscamEntry09UlRelayW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry09_normal(
        &mut self,
    ) -> AddresscamEntry09NormalW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry09_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry09A2UnmatchW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry09_subtype_mask(
        &mut self,
    ) -> AddresscamEntry09SubtypeMaskW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry09_valid(
        &mut self,
    ) -> AddresscamEntry09ValidW<'_, RegAddresscamEntry09Spec> {
        AddresscamEntry09ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry09::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry09::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry09Spec;
impl crate::RegisterSpec for RegAddresscamEntry09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry09::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry09Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry09::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry09Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY09 to value 0"]
impl crate::Resettable for RegAddresscamEntry09Spec {}
