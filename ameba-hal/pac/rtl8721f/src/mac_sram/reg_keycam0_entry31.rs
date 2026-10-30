#[doc = "Register `REG_KEYCAM0_ENTRY31` reader"]
pub type R = crate::R<RegKeycam0Entry31Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY31` writer"]
pub type W = crate::W<RegKeycam0Entry31Spec>;
#[doc = "Field `KEYCAM0_ENTRY31_KEYID_BIT` reader - "]
pub type Keycam0Entry31KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY31_KEYID_BIT` writer - "]
pub type Keycam0Entry31KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY31_TYPE` reader - "]
pub type Keycam0Entry31TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY31_TYPE` writer - "]
pub type Keycam0Entry31TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY31_MICKEY_BIT` reader - "]
pub type Keycam0Entry31MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY31_MICKEY_BIT` writer - "]
pub type Keycam0Entry31MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY31_GRPKEY_BIT` reader - "]
pub type Keycam0Entry31GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY31_GRPKEY_BIT` writer - "]
pub type Keycam0Entry31GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY31_MGNT` reader - "]
pub type Keycam0Entry31MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY31_MGNT` writer - "]
pub type Keycam0Entry31MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY31_REPEATER_MODE` reader - "]
pub type Keycam0Entry31RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY31_REPEATER_MODE` writer - "]
pub type Keycam0Entry31RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY31_EXTTYPE` reader - "]
pub type Keycam0Entry31ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY31_EXTTYPE` writer - "]
pub type Keycam0Entry31ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY31_VALID` reader - "]
pub type Keycam0Entry31ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY31_VALID` writer - "]
pub type Keycam0Entry31ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY31_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry31Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY31_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry31Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry31_keyid_bit(&self) -> Keycam0Entry31KeyidBitR {
        Keycam0Entry31KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry31_type(&self) -> Keycam0Entry31TypeR {
        Keycam0Entry31TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry31_mickey_bit(&self) -> Keycam0Entry31MickeyBitR {
        Keycam0Entry31MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry31_grpkey_bit(&self) -> Keycam0Entry31GrpkeyBitR {
        Keycam0Entry31GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry31_mgnt(&self) -> Keycam0Entry31MgntR {
        Keycam0Entry31MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry31_repeater_mode(&self) -> Keycam0Entry31RepeaterModeR {
        Keycam0Entry31RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry31_exttype(&self) -> Keycam0Entry31ExttypeR {
        Keycam0Entry31ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry31_valid(&self) -> Keycam0Entry31ValidR {
        Keycam0Entry31ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry31_addr0(&self) -> Keycam0Entry31Addr0R {
        Keycam0Entry31Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry31_keyid_bit(
        &mut self,
    ) -> Keycam0Entry31KeyidBitW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry31_type(&mut self) -> Keycam0Entry31TypeW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry31_mickey_bit(
        &mut self,
    ) -> Keycam0Entry31MickeyBitW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry31_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry31GrpkeyBitW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry31_mgnt(&mut self) -> Keycam0Entry31MgntW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry31_repeater_mode(
        &mut self,
    ) -> Keycam0Entry31RepeaterModeW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry31_exttype(&mut self) -> Keycam0Entry31ExttypeW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry31_valid(&mut self) -> Keycam0Entry31ValidW<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry31_addr0(&mut self) -> Keycam0Entry31Addr0W<'_, RegKeycam0Entry31Spec> {
        Keycam0Entry31Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry31Spec;
impl crate::RegisterSpec for RegKeycam0Entry31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry31::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry31::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY31 to value 0"]
impl crate::Resettable for RegKeycam0Entry31Spec {}
