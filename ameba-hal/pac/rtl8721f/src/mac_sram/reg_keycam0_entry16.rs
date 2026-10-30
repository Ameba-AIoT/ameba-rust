#[doc = "Register `REG_KEYCAM0_ENTRY16` reader"]
pub type R = crate::R<RegKeycam0Entry16Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY16` writer"]
pub type W = crate::W<RegKeycam0Entry16Spec>;
#[doc = "Field `KEYCAM0_ENTRY16_KEYID_BIT` reader - "]
pub type Keycam0Entry16KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY16_KEYID_BIT` writer - "]
pub type Keycam0Entry16KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY16_TYPE` reader - "]
pub type Keycam0Entry16TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY16_TYPE` writer - "]
pub type Keycam0Entry16TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY16_MICKEY_BIT` reader - "]
pub type Keycam0Entry16MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY16_MICKEY_BIT` writer - "]
pub type Keycam0Entry16MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY16_GRPKEY_BIT` reader - "]
pub type Keycam0Entry16GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY16_GRPKEY_BIT` writer - "]
pub type Keycam0Entry16GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY16_MGNT` reader - "]
pub type Keycam0Entry16MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY16_MGNT` writer - "]
pub type Keycam0Entry16MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY16_REPEATER_MODE` reader - "]
pub type Keycam0Entry16RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY16_REPEATER_MODE` writer - "]
pub type Keycam0Entry16RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY16_EXTTYPE` reader - "]
pub type Keycam0Entry16ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY16_EXTTYPE` writer - "]
pub type Keycam0Entry16ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY16_VALID` reader - "]
pub type Keycam0Entry16ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY16_VALID` writer - "]
pub type Keycam0Entry16ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY16_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry16Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY16_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry16Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry16_keyid_bit(&self) -> Keycam0Entry16KeyidBitR {
        Keycam0Entry16KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry16_type(&self) -> Keycam0Entry16TypeR {
        Keycam0Entry16TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry16_mickey_bit(&self) -> Keycam0Entry16MickeyBitR {
        Keycam0Entry16MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry16_grpkey_bit(&self) -> Keycam0Entry16GrpkeyBitR {
        Keycam0Entry16GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry16_mgnt(&self) -> Keycam0Entry16MgntR {
        Keycam0Entry16MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry16_repeater_mode(&self) -> Keycam0Entry16RepeaterModeR {
        Keycam0Entry16RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry16_exttype(&self) -> Keycam0Entry16ExttypeR {
        Keycam0Entry16ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry16_valid(&self) -> Keycam0Entry16ValidR {
        Keycam0Entry16ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry16_addr0(&self) -> Keycam0Entry16Addr0R {
        Keycam0Entry16Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry16_keyid_bit(
        &mut self,
    ) -> Keycam0Entry16KeyidBitW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry16_type(&mut self) -> Keycam0Entry16TypeW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry16_mickey_bit(
        &mut self,
    ) -> Keycam0Entry16MickeyBitW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry16_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry16GrpkeyBitW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry16_mgnt(&mut self) -> Keycam0Entry16MgntW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry16_repeater_mode(
        &mut self,
    ) -> Keycam0Entry16RepeaterModeW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry16_exttype(&mut self) -> Keycam0Entry16ExttypeW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry16_valid(&mut self) -> Keycam0Entry16ValidW<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry16_addr0(&mut self) -> Keycam0Entry16Addr0W<'_, RegKeycam0Entry16Spec> {
        Keycam0Entry16Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry16Spec;
impl crate::RegisterSpec for RegKeycam0Entry16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry16::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry16::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY16 to value 0"]
impl crate::Resettable for RegKeycam0Entry16Spec {}
