#[doc = "Register `REG_KEYCAM0_ENTRY02` reader"]
pub type R = crate::R<RegKeycam0Entry02Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY02` writer"]
pub type W = crate::W<RegKeycam0Entry02Spec>;
#[doc = "Field `KEYCAM0_ENTRY02_KEYID_BIT` reader - "]
pub type Keycam0Entry02KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY02_KEYID_BIT` writer - "]
pub type Keycam0Entry02KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY02_TYPE` reader - "]
pub type Keycam0Entry02TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY02_TYPE` writer - "]
pub type Keycam0Entry02TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY02_MICKEY_BIT` reader - "]
pub type Keycam0Entry02MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY02_MICKEY_BIT` writer - "]
pub type Keycam0Entry02MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY02_GRPKEY_BIT` reader - "]
pub type Keycam0Entry02GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY02_GRPKEY_BIT` writer - "]
pub type Keycam0Entry02GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY02_MGNT` reader - "]
pub type Keycam0Entry02MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY02_MGNT` writer - "]
pub type Keycam0Entry02MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY02_REPEATER_MODE` reader - "]
pub type Keycam0Entry02RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY02_REPEATER_MODE` writer - "]
pub type Keycam0Entry02RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY02_EXTTYPE` reader - "]
pub type Keycam0Entry02ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY02_EXTTYPE` writer - "]
pub type Keycam0Entry02ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY02_VALID` reader - "]
pub type Keycam0Entry02ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY02_VALID` writer - "]
pub type Keycam0Entry02ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY02_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry02Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY02_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry02Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry02_keyid_bit(&self) -> Keycam0Entry02KeyidBitR {
        Keycam0Entry02KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry02_type(&self) -> Keycam0Entry02TypeR {
        Keycam0Entry02TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry02_mickey_bit(&self) -> Keycam0Entry02MickeyBitR {
        Keycam0Entry02MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry02_grpkey_bit(&self) -> Keycam0Entry02GrpkeyBitR {
        Keycam0Entry02GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry02_mgnt(&self) -> Keycam0Entry02MgntR {
        Keycam0Entry02MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry02_repeater_mode(&self) -> Keycam0Entry02RepeaterModeR {
        Keycam0Entry02RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry02_exttype(&self) -> Keycam0Entry02ExttypeR {
        Keycam0Entry02ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry02_valid(&self) -> Keycam0Entry02ValidR {
        Keycam0Entry02ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry02_addr0(&self) -> Keycam0Entry02Addr0R {
        Keycam0Entry02Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry02_keyid_bit(
        &mut self,
    ) -> Keycam0Entry02KeyidBitW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry02_type(&mut self) -> Keycam0Entry02TypeW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry02_mickey_bit(
        &mut self,
    ) -> Keycam0Entry02MickeyBitW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry02_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry02GrpkeyBitW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry02_mgnt(&mut self) -> Keycam0Entry02MgntW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry02_repeater_mode(
        &mut self,
    ) -> Keycam0Entry02RepeaterModeW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry02_exttype(&mut self) -> Keycam0Entry02ExttypeW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry02_valid(&mut self) -> Keycam0Entry02ValidW<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry02_addr0(&mut self) -> Keycam0Entry02Addr0W<'_, RegKeycam0Entry02Spec> {
        Keycam0Entry02Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry02Spec;
impl crate::RegisterSpec for RegKeycam0Entry02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry02::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry02Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry02::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry02Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY02 to value 0"]
impl crate::Resettable for RegKeycam0Entry02Spec {}
