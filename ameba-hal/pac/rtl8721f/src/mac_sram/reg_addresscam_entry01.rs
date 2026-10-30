#[doc = "Register `REG_ADDRESSCAM_ENTRY01` reader"]
pub type R = crate::R<RegAddresscamEntry01Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY01` writer"]
pub type W = crate::W<RegAddresscamEntry01Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY01_A3` reader - "]
pub type AddresscamEntry01A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_A3` writer - "]
pub type AddresscamEntry01A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY01_A2` reader - "]
pub type AddresscamEntry01A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_A2` writer - "]
pub type AddresscamEntry01A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY01_A1` reader - "]
pub type AddresscamEntry01A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_A1` writer - "]
pub type AddresscamEntry01A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY01_UC` reader - "]
pub type AddresscamEntry01UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_UC` writer - "]
pub type AddresscamEntry01UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_MC` reader - "]
pub type AddresscamEntry01McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_MC` writer - "]
pub type AddresscamEntry01McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_BC` reader - "]
pub type AddresscamEntry01BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_BC` writer - "]
pub type AddresscamEntry01BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_SUBTYPE` reader - "]
pub type AddresscamEntry01SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_SUBTYPE` writer - "]
pub type AddresscamEntry01SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY01_CTRL` reader - "]
pub type AddresscamEntry01CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_CTRL` writer - "]
pub type AddresscamEntry01CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_MGNT` reader - "]
pub type AddresscamEntry01MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_MGNT` writer - "]
pub type AddresscamEntry01MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_DATA` reader - "]
pub type AddresscamEntry01DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_DATA` writer - "]
pub type AddresscamEntry01DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_RELAY` reader - "]
pub type AddresscamEntry01RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_RELAY` writer - "]
pub type AddresscamEntry01RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_UL_RELAY` reader - "]
pub type AddresscamEntry01UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_UL_RELAY` writer - "]
pub type AddresscamEntry01UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_NORMAL` reader - "]
pub type AddresscamEntry01NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_NORMAL` writer - "]
pub type AddresscamEntry01NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_A2_UNMATCH` reader - "]
pub type AddresscamEntry01A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_A2_UNMATCH` writer - "]
pub type AddresscamEntry01A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry01SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry01SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY01_VALID` reader - "]
pub type AddresscamEntry01ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY01_VALID` writer - "]
pub type AddresscamEntry01ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry01_a3(&self) -> AddresscamEntry01A3R {
        AddresscamEntry01A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry01_a2(&self) -> AddresscamEntry01A2R {
        AddresscamEntry01A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry01_a1(&self) -> AddresscamEntry01A1R {
        AddresscamEntry01A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry01_uc(&self) -> AddresscamEntry01UcR {
        AddresscamEntry01UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry01_mc(&self) -> AddresscamEntry01McR {
        AddresscamEntry01McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry01_bc(&self) -> AddresscamEntry01BcR {
        AddresscamEntry01BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry01_subtype(&self) -> AddresscamEntry01SubtypeR {
        AddresscamEntry01SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry01_ctrl(&self) -> AddresscamEntry01CtrlR {
        AddresscamEntry01CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry01_mgnt(&self) -> AddresscamEntry01MgntR {
        AddresscamEntry01MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry01_data(&self) -> AddresscamEntry01DataR {
        AddresscamEntry01DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry01_relay(&self) -> AddresscamEntry01RelayR {
        AddresscamEntry01RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry01_ul_relay(&self) -> AddresscamEntry01UlRelayR {
        AddresscamEntry01UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry01_normal(&self) -> AddresscamEntry01NormalR {
        AddresscamEntry01NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry01_a2_unmatch(&self) -> AddresscamEntry01A2UnmatchR {
        AddresscamEntry01A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry01_subtype_mask(&self) -> AddresscamEntry01SubtypeMaskR {
        AddresscamEntry01SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry01_valid(&self) -> AddresscamEntry01ValidR {
        AddresscamEntry01ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry01_a3(&mut self) -> AddresscamEntry01A3W<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry01_a2(&mut self) -> AddresscamEntry01A2W<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry01_a1(&mut self) -> AddresscamEntry01A1W<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry01_uc(&mut self) -> AddresscamEntry01UcW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry01_mc(&mut self) -> AddresscamEntry01McW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry01_bc(&mut self) -> AddresscamEntry01BcW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry01_subtype(
        &mut self,
    ) -> AddresscamEntry01SubtypeW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry01_ctrl(
        &mut self,
    ) -> AddresscamEntry01CtrlW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry01_mgnt(
        &mut self,
    ) -> AddresscamEntry01MgntW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry01_data(
        &mut self,
    ) -> AddresscamEntry01DataW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry01_relay(
        &mut self,
    ) -> AddresscamEntry01RelayW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry01_ul_relay(
        &mut self,
    ) -> AddresscamEntry01UlRelayW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry01_normal(
        &mut self,
    ) -> AddresscamEntry01NormalW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry01_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry01A2UnmatchW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry01_subtype_mask(
        &mut self,
    ) -> AddresscamEntry01SubtypeMaskW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry01_valid(
        &mut self,
    ) -> AddresscamEntry01ValidW<'_, RegAddresscamEntry01Spec> {
        AddresscamEntry01ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry01Spec;
impl crate::RegisterSpec for RegAddresscamEntry01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry01::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry01Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry01::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY01 to value 0"]
impl crate::Resettable for RegAddresscamEntry01Spec {}
