#[doc = "Register `REG_ADDRESSCAM_ENTRY11` reader"]
pub type R = crate::R<RegAddresscamEntry11Spec>;
#[doc = "Register `REG_ADDRESSCAM_ENTRY11` writer"]
pub type W = crate::W<RegAddresscamEntry11Spec>;
#[doc = "Field `ADDRESSCAM_ENTRY11_A3` reader - "]
pub type AddresscamEntry11A3R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_A3` writer - "]
pub type AddresscamEntry11A3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY11_A2` reader - "]
pub type AddresscamEntry11A2R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_A2` writer - "]
pub type AddresscamEntry11A2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY11_A1` reader - "]
pub type AddresscamEntry11A1R = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_A1` writer - "]
pub type AddresscamEntry11A1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY11_UC` reader - "]
pub type AddresscamEntry11UcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_UC` writer - "]
pub type AddresscamEntry11UcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_MC` reader - "]
pub type AddresscamEntry11McR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_MC` writer - "]
pub type AddresscamEntry11McW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_BC` reader - "]
pub type AddresscamEntry11BcR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_BC` writer - "]
pub type AddresscamEntry11BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_SUBTYPE` reader - "]
pub type AddresscamEntry11SubtypeR = crate::FieldReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_SUBTYPE` writer - "]
pub type AddresscamEntry11SubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESSCAM_ENTRY11_CTRL` reader - "]
pub type AddresscamEntry11CtrlR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_CTRL` writer - "]
pub type AddresscamEntry11CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_MGNT` reader - "]
pub type AddresscamEntry11MgntR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_MGNT` writer - "]
pub type AddresscamEntry11MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_DATA` reader - "]
pub type AddresscamEntry11DataR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_DATA` writer - "]
pub type AddresscamEntry11DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_RELAY` reader - "]
pub type AddresscamEntry11RelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_RELAY` writer - "]
pub type AddresscamEntry11RelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_UL_RELAY` reader - "]
pub type AddresscamEntry11UlRelayR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_UL_RELAY` writer - "]
pub type AddresscamEntry11UlRelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_NORMAL` reader - "]
pub type AddresscamEntry11NormalR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_NORMAL` writer - "]
pub type AddresscamEntry11NormalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_A2_UNMATCH` reader - "]
pub type AddresscamEntry11A2UnmatchR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_A2_UNMATCH` writer - "]
pub type AddresscamEntry11A2UnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_SUBTYPE_MASK` reader - "]
pub type AddresscamEntry11SubtypeMaskR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_SUBTYPE_MASK` writer - "]
pub type AddresscamEntry11SubtypeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSCAM_ENTRY11_VALID` reader - "]
pub type AddresscamEntry11ValidR = crate::BitReader;
#[doc = "Field `ADDRESSCAM_ENTRY11_VALID` writer - "]
pub type AddresscamEntry11ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry11_a3(&self) -> AddresscamEntry11A3R {
        AddresscamEntry11A3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry11_a2(&self) -> AddresscamEntry11A2R {
        AddresscamEntry11A2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry11_a1(&self) -> AddresscamEntry11A1R {
        AddresscamEntry11A1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry11_uc(&self) -> AddresscamEntry11UcR {
        AddresscamEntry11UcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry11_mc(&self) -> AddresscamEntry11McR {
        AddresscamEntry11McR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry11_bc(&self) -> AddresscamEntry11BcR {
        AddresscamEntry11BcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry11_subtype(&self) -> AddresscamEntry11SubtypeR {
        AddresscamEntry11SubtypeR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry11_ctrl(&self) -> AddresscamEntry11CtrlR {
        AddresscamEntry11CtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry11_mgnt(&self) -> AddresscamEntry11MgntR {
        AddresscamEntry11MgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry11_data(&self) -> AddresscamEntry11DataR {
        AddresscamEntry11DataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry11_relay(&self) -> AddresscamEntry11RelayR {
        AddresscamEntry11RelayR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry11_ul_relay(&self) -> AddresscamEntry11UlRelayR {
        AddresscamEntry11UlRelayR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry11_normal(&self) -> AddresscamEntry11NormalR {
        AddresscamEntry11NormalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry11_a2_unmatch(&self) -> AddresscamEntry11A2UnmatchR {
        AddresscamEntry11A2UnmatchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry11_subtype_mask(&self) -> AddresscamEntry11SubtypeMaskR {
        AddresscamEntry11SubtypeMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry11_valid(&self) -> AddresscamEntry11ValidR {
        AddresscamEntry11ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn addresscam_entry11_a3(&mut self) -> AddresscamEntry11A3W<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11A3W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn addresscam_entry11_a2(&mut self) -> AddresscamEntry11A2W<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11A2W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn addresscam_entry11_a1(&mut self) -> AddresscamEntry11A1W<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11A1W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn addresscam_entry11_uc(&mut self) -> AddresscamEntry11UcW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11UcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn addresscam_entry11_mc(&mut self) -> AddresscamEntry11McW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11McW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn addresscam_entry11_bc(&mut self) -> AddresscamEntry11BcW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11BcW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn addresscam_entry11_subtype(
        &mut self,
    ) -> AddresscamEntry11SubtypeW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11SubtypeW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn addresscam_entry11_ctrl(
        &mut self,
    ) -> AddresscamEntry11CtrlW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11CtrlW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn addresscam_entry11_mgnt(
        &mut self,
    ) -> AddresscamEntry11MgntW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11MgntW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn addresscam_entry11_data(
        &mut self,
    ) -> AddresscamEntry11DataW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11DataW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn addresscam_entry11_relay(
        &mut self,
    ) -> AddresscamEntry11RelayW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11RelayW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn addresscam_entry11_ul_relay(
        &mut self,
    ) -> AddresscamEntry11UlRelayW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11UlRelayW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn addresscam_entry11_normal(
        &mut self,
    ) -> AddresscamEntry11NormalW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11NormalW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn addresscam_entry11_a2_unmatch(
        &mut self,
    ) -> AddresscamEntry11A2UnmatchW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11A2UnmatchW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn addresscam_entry11_subtype_mask(
        &mut self,
    ) -> AddresscamEntry11SubtypeMaskW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11SubtypeMaskW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn addresscam_entry11_valid(
        &mut self,
    ) -> AddresscamEntry11ValidW<'_, RegAddresscamEntry11Spec> {
        AddresscamEntry11ValidW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_addresscam_entry11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_addresscam_entry11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddresscamEntry11Spec;
impl crate::RegisterSpec for RegAddresscamEntry11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_addresscam_entry11::R`](R) reader structure"]
impl crate::Readable for RegAddresscamEntry11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_addresscam_entry11::W`](W) writer structure"]
impl crate::Writable for RegAddresscamEntry11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESSCAM_ENTRY11 to value 0"]
impl crate::Resettable for RegAddresscamEntry11Spec {}
