#[doc = "Register `REG_ADDRESSCAM_ENTRY02` reader"]
pub type R = crate::R<RegAddresscamEntry02Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY02` writer"]
pub type W = crate::W<RegAddresscamEntry02Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY02_A3` reader - "]
pub type AddresscamEntry02A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_A3` writer - "]
pub type AddresscamEntry02A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY02_A2` reader - "]
pub type AddresscamEntry02A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_A2` writer - "]
pub type AddresscamEntry02A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY02_A1` reader - "]
pub type AddresscamEntry02A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_A1` writer - "]
pub type AddresscamEntry02A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY02_UC` reader - "]
pub type AddresscamEntry02UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_UC` writer - "]
pub type AddresscamEntry02UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_MC` reader - "]
pub type AddresscamEntry02McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_MC` writer - "]
pub type AddresscamEntry02McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_BC` reader - "]
pub type AddresscamEntry02BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_BC` writer - "]
pub type AddresscamEntry02BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_SUBTYPE` reader - "]
pub type AddresscamEntry02SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_SUBTYPE` writer - "]
pub type AddresscamEntry02SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY02_CTRL` reader - "]
pub type AddresscamEntry02CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_CTRL` writer - "]
pub type AddresscamEntry02CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_MGNT` reader - "]
pub type AddresscamEntry02MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_MGNT` writer - "]
pub type AddresscamEntry02MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_DATA` reader - "]
pub type AddresscamEntry02DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_DATA` writer - "]
pub type AddresscamEntry02DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_RELAY` reader - "]
pub type AddresscamEntry02RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_RELAY` writer - "]
pub type AddresscamEntry02RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_UL_RELAY` reader - "]
pub type AddresscamEntry02UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_UL_RELAY` writer - "]
pub type AddresscamEntry02UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_NORMAL` reader - "]
pub type AddresscamEntry02NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_NORMAL` writer - "]
pub type AddresscamEntry02NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_A2_UNMATCH` reader - "]
pub type AddresscamEntry02A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_A2_UNMATCH` writer - "]
pub type AddresscamEntry02A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry02SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry02SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY02_VALID` reader - "]
pub type AddresscamEntry02ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY02_VALID` writer - "]
pub type AddresscamEntry02ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry02_a3(&self) -> AddresscamEntry02A3R {
        AddresscamEntry02A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry02_a2(&self) -> AddresscamEntry02A2R {
        AddresscamEntry02A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry02_a1(&self) -> AddresscamEntry02A1R {
        AddresscamEntry02A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry02_uc(&self) -> AddresscamEntry02UcR {
        AddresscamEntry02UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry02_mc(&self) -> AddresscamEntry02McR {
        AddresscamEntry02McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry02_bc(&self) -> AddresscamEntry02BcR {
        AddresscamEntry02BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry02_subtype(&self) -> AddresscamEntry02SubtypeR {
        AddresscamEntry02SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry02_ctrl(&self) -> AddresscamEntry02CtrlR {
        AddresscamEntry02CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry02_mgnt(&self) -> AddresscamEntry02MgntR {
        AddresscamEntry02MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry02_data(&self) -> AddresscamEntry02DataR {
        AddresscamEntry02DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry02_relay(&self) -> AddresscamEntry02RelayR {
        AddresscamEntry02RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry02_ul_relay(&self) -> AddresscamEntry02UlRelayR {
        AddresscamEntry02UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry02_normal(&self) -> AddresscamEntry02NormalR {
        AddresscamEntry02NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry02_a2_unmatch(&self) -> AddresscamEntry02A2UnmatchR {
        AddresscamEntry02A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry02_subtype_mask(&self) -> AddresscamEntry02SubtypeMaskR {
        AddresscamEntry02SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry02_valid(&self) -> AddresscamEntry02ValidR {
        AddresscamEntry02ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry02_a3(&mut self) -> AddresscamEntry02A3W<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry02_a2(&mut self) -> AddresscamEntry02A2W<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry02_a1(&mut self) -> AddresscamEntry02A1W<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry02_uc(&mut self) -> AddresscamEntry02UcW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry02_mc(&mut self) -> AddresscamEntry02McW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry02_bc(&mut self) -> AddresscamEntry02BcW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry02_subtype(
        &mut self,
    ) -> AddresscamEntry02SubtypeW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry02_ctrl(
        &mut self,
    ) -> AddresscamEntry02CtrlW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry02_mgnt(
        &mut self,
    ) -> AddresscamEntry02MgntW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry02_data(
        &mut self,
    ) -> AddresscamEntry02DataW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry02_relay(
        &mut self,
    ) -> AddresscamEntry02RelayW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry02_ul_relay(
        &mut self,
    ) -> AddresscamEntry02UlRelayW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry02_normal(
        &mut self,
    ) -> AddresscamEntry02NormalW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry02_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry02A2UnmatchW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry02_subtype_mask(
        &mut self,
    ) -> AddresscamEntry02SubtypeMaskW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry02_valid(
        &mut self,
    ) -> AddresscamEntry02ValidW<'_, RegAddresscamEntry02Spec> {
        AddresscamEntry02ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry02Spec;
impl crate::RegisterSpec for RegAddresscamEntry02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry02::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry02Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry02::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry02Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY02 to value 0"]
impl crate::Resettable for RegAddresscamEntry02Spec {}
