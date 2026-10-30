#[doc = "Register `REG_KEYCAM0_ENTRY23` reader"]
pub type R = crate::R<RegKeycam0Entry23Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY23` writer"]
pub type W = crate::W<RegKeycam0Entry23Spec>;
#[doc = "Field `KEYCAM0_ENTRY23_KEYID_BIT` reader - "]
pub type Keycam0Entry23KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY23_KEYID_BIT` writer - "]
pub type Keycam0Entry23KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY23_TYPE` reader - "]
pub type Keycam0Entry23TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY23_TYPE` writer - "]
pub type Keycam0Entry23TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY23_MICKEY_BIT` reader - "]
pub type Keycam0Entry23MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY23_MICKEY_BIT` writer - "]
pub type Keycam0Entry23MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY23_GRPKEY_BIT` reader - "]
pub type Keycam0Entry23GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY23_GRPKEY_BIT` writer - "]
pub type Keycam0Entry23GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY23_MGNT` reader - "]
pub type Keycam0Entry23MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY23_MGNT` writer - "]
pub type Keycam0Entry23MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY23_REPEATER_MODE` reader - "]
pub type Keycam0Entry23RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY23_REPEATER_MODE` writer - "]
pub type Keycam0Entry23RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY23_EXTTYPE` reader - "]
pub type Keycam0Entry23ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY23_EXTTYPE` writer - "]
pub type Keycam0Entry23ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY23_VALID` reader - "]
pub type Keycam0Entry23ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY23_VALID` writer - "]
pub type Keycam0Entry23ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY23_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry23Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY23_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry23Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry23_keyid_bit(&self) -> Keycam0Entry23KeyidBitR {
        Keycam0Entry23KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry23_type(&self) -> Keycam0Entry23TypeR {
        Keycam0Entry23TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry23_mickey_bit(&self) -> Keycam0Entry23MickeyBitR {
        Keycam0Entry23MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry23_grpkey_bit(&self) -> Keycam0Entry23GrpkeyBitR {
        Keycam0Entry23GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry23_mgnt(&self) -> Keycam0Entry23MgntR {
        Keycam0Entry23MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry23_repeater_mode(&self) -> Keycam0Entry23RepeaterModeR {
        Keycam0Entry23RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry23_exttype(&self) -> Keycam0Entry23ExttypeR {
        Keycam0Entry23ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry23_valid(&self) -> Keycam0Entry23ValidR {
        Keycam0Entry23ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry23_addr0(&self) -> Keycam0Entry23Addr0R {
        Keycam0Entry23Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry23_keyid_bit(
        &mut self,
    ) -> Keycam0Entry23KeyidBitW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry23_type(&mut self) -> Keycam0Entry23TypeW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry23_mickey_bit(
        &mut self,
    ) -> Keycam0Entry23MickeyBitW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry23_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry23GrpkeyBitW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry23_mgnt(&mut self) -> Keycam0Entry23MgntW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry23_repeater_mode(
        &mut self,
    ) -> Keycam0Entry23RepeaterModeW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry23_exttype(&mut self) -> Keycam0Entry23ExttypeW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry23_valid(&mut self) -> Keycam0Entry23ValidW<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry23_addr0(&mut self) -> Keycam0Entry23Addr0W<'_, RegKeycam0Entry23Spec> {
        Keycam0Entry23Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry23Spec;
impl crate::RegisterSpec for RegKeycam0Entry23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry23::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry23Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry23::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY23 to value 0"]
impl crate::Resettable for RegKeycam0Entry23Spec {}
