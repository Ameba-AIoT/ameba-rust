#[doc = "Register `REG_KEYCAM0_ENTRY29` reader"]
pub type R = crate::R<RegKeycam0Entry29Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY29` writer"]
pub type W = crate::W<RegKeycam0Entry29Spec>;
#[doc = "Field `KEYCAM0_ENTRY29_KEYID_BIT` reader - "]
pub type Keycam0Entry29KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY29_KEYID_BIT` writer - "]
pub type Keycam0Entry29KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY29_TYPE` reader - "]
pub type Keycam0Entry29TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY29_TYPE` writer - "]
pub type Keycam0Entry29TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY29_MICKEY_BIT` reader - "]
pub type Keycam0Entry29MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY29_MICKEY_BIT` writer - "]
pub type Keycam0Entry29MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY29_GRPKEY_BIT` reader - "]
pub type Keycam0Entry29GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY29_GRPKEY_BIT` writer - "]
pub type Keycam0Entry29GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY29_MGNT` reader - "]
pub type Keycam0Entry29MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY29_MGNT` writer - "]
pub type Keycam0Entry29MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY29_REPEATER_MODE` reader - "]
pub type Keycam0Entry29RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY29_REPEATER_MODE` writer - "]
pub type Keycam0Entry29RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY29_EXTTYPE` reader - "]
pub type Keycam0Entry29ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY29_EXTTYPE` writer - "]
pub type Keycam0Entry29ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY29_VALID` reader - "]
pub type Keycam0Entry29ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY29_VALID` writer - "]
pub type Keycam0Entry29ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY29_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry29Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY29_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry29Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry29_keyid_bit(&self) -> Keycam0Entry29KeyidBitR {
        Keycam0Entry29KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry29_type(&self) -> Keycam0Entry29TypeR {
        Keycam0Entry29TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry29_mickey_bit(&self) -> Keycam0Entry29MickeyBitR {
        Keycam0Entry29MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry29_grpkey_bit(&self) -> Keycam0Entry29GrpkeyBitR {
        Keycam0Entry29GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry29_mgnt(&self) -> Keycam0Entry29MgntR {
        Keycam0Entry29MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry29_repeater_mode(&self) -> Keycam0Entry29RepeaterModeR {
        Keycam0Entry29RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry29_exttype(&self) -> Keycam0Entry29ExttypeR {
        Keycam0Entry29ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry29_valid(&self) -> Keycam0Entry29ValidR {
        Keycam0Entry29ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry29_addr0(&self) -> Keycam0Entry29Addr0R {
        Keycam0Entry29Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry29_keyid_bit(
        &mut self,
    ) -> Keycam0Entry29KeyidBitW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry29_type(&mut self) -> Keycam0Entry29TypeW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry29_mickey_bit(
        &mut self,
    ) -> Keycam0Entry29MickeyBitW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry29_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry29GrpkeyBitW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry29_mgnt(&mut self) -> Keycam0Entry29MgntW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry29_repeater_mode(
        &mut self,
    ) -> Keycam0Entry29RepeaterModeW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry29_exttype(&mut self) -> Keycam0Entry29ExttypeW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry29_valid(&mut self) -> Keycam0Entry29ValidW<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry29_addr0(&mut self) -> Keycam0Entry29Addr0W<'_, RegKeycam0Entry29Spec> {
        Keycam0Entry29Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry29Spec;
impl crate::RegisterSpec for RegKeycam0Entry29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry29::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry29Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry29::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY29 to value 0"]
impl crate::Resettable for RegKeycam0Entry29Spec {}
