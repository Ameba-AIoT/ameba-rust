#[doc = "Register `REG_KEYCAM0_ENTRY10` reader"]
pub type R = crate::R<RegKeycam0Entry10Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY10` writer"]
pub type W = crate::W<RegKeycam0Entry10Spec>;
#[doc = "Field `KEYCAM0_ENTRY10_KEYID_BIT` reader - "]
pub type Keycam0Entry10KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY10_KEYID_BIT` writer - "]
pub type Keycam0Entry10KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY10_TYPE` reader - "]
pub type Keycam0Entry10TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY10_TYPE` writer - "]
pub type Keycam0Entry10TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY10_MICKEY_BIT` reader - "]
pub type Keycam0Entry10MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY10_MICKEY_BIT` writer - "]
pub type Keycam0Entry10MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY10_GRPKEY_BIT` reader - "]
pub type Keycam0Entry10GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY10_GRPKEY_BIT` writer - "]
pub type Keycam0Entry10GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY10_MGNT` reader - "]
pub type Keycam0Entry10MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY10_MGNT` writer - "]
pub type Keycam0Entry10MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY10_REPEATER_MODE` reader - "]
pub type Keycam0Entry10RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY10_REPEATER_MODE` writer - "]
pub type Keycam0Entry10RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY10_EXTTYPE` reader - "]
pub type Keycam0Entry10ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY10_EXTTYPE` writer - "]
pub type Keycam0Entry10ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY10_VALID` reader - "]
pub type Keycam0Entry10ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY10_VALID` writer - "]
pub type Keycam0Entry10ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY10_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry10Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY10_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry10Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry10_keyid_bit(&self) -> Keycam0Entry10KeyidBitR {
        Keycam0Entry10KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry10_type(&self) -> Keycam0Entry10TypeR {
        Keycam0Entry10TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry10_mickey_bit(&self) -> Keycam0Entry10MickeyBitR {
        Keycam0Entry10MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry10_grpkey_bit(&self) -> Keycam0Entry10GrpkeyBitR {
        Keycam0Entry10GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry10_mgnt(&self) -> Keycam0Entry10MgntR {
        Keycam0Entry10MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry10_repeater_mode(&self) -> Keycam0Entry10RepeaterModeR {
        Keycam0Entry10RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry10_exttype(&self) -> Keycam0Entry10ExttypeR {
        Keycam0Entry10ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry10_valid(&self) -> Keycam0Entry10ValidR {
        Keycam0Entry10ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry10_addr0(&self) -> Keycam0Entry10Addr0R {
        Keycam0Entry10Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry10_keyid_bit(
        &mut self,
    ) -> Keycam0Entry10KeyidBitW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry10_type(&mut self) -> Keycam0Entry10TypeW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry10_mickey_bit(
        &mut self,
    ) -> Keycam0Entry10MickeyBitW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry10_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry10GrpkeyBitW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry10_mgnt(&mut self) -> Keycam0Entry10MgntW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry10_repeater_mode(
        &mut self,
    ) -> Keycam0Entry10RepeaterModeW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry10_exttype(&mut self) -> Keycam0Entry10ExttypeW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry10_valid(&mut self) -> Keycam0Entry10ValidW<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry10_addr0(&mut self) -> Keycam0Entry10Addr0W<'_, RegKeycam0Entry10Spec> {
        Keycam0Entry10Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry10Spec;
impl crate::RegisterSpec for RegKeycam0Entry10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry10::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry10::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY10 to value 0"]
impl crate::Resettable for RegKeycam0Entry10Spec {}
