#[doc = "Register `REG_KEYCAM0_ENTRY19` reader"]
pub type R = crate::R<RegKeycam0Entry19Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY19` writer"]
pub type W = crate::W<RegKeycam0Entry19Spec>;
#[doc = "Field `KEYCAM0_ENTRY19_KEYID_BIT` reader - "]
pub type Keycam0Entry19KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY19_KEYID_BIT` writer - "]
pub type Keycam0Entry19KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY19_TYPE` reader - "]
pub type Keycam0Entry19TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY19_TYPE` writer - "]
pub type Keycam0Entry19TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY19_MICKEY_BIT` reader - "]
pub type Keycam0Entry19MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY19_MICKEY_BIT` writer - "]
pub type Keycam0Entry19MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY19_GRPKEY_BIT` reader - "]
pub type Keycam0Entry19GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY19_GRPKEY_BIT` writer - "]
pub type Keycam0Entry19GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY19_MGNT` reader - "]
pub type Keycam0Entry19MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY19_MGNT` writer - "]
pub type Keycam0Entry19MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY19_REPEATER_MODE` reader - "]
pub type Keycam0Entry19RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY19_REPEATER_MODE` writer - "]
pub type Keycam0Entry19RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY19_EXTTYPE` reader - "]
pub type Keycam0Entry19ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY19_EXTTYPE` writer - "]
pub type Keycam0Entry19ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY19_VALID` reader - "]
pub type Keycam0Entry19ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY19_VALID` writer - "]
pub type Keycam0Entry19ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY19_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry19Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY19_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry19Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry19_keyid_bit(&self) -> Keycam0Entry19KeyidBitR {
        Keycam0Entry19KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry19_type(&self) -> Keycam0Entry19TypeR {
        Keycam0Entry19TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry19_mickey_bit(&self) -> Keycam0Entry19MickeyBitR {
        Keycam0Entry19MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry19_grpkey_bit(&self) -> Keycam0Entry19GrpkeyBitR {
        Keycam0Entry19GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry19_mgnt(&self) -> Keycam0Entry19MgntR {
        Keycam0Entry19MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry19_repeater_mode(&self) -> Keycam0Entry19RepeaterModeR {
        Keycam0Entry19RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry19_exttype(&self) -> Keycam0Entry19ExttypeR {
        Keycam0Entry19ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry19_valid(&self) -> Keycam0Entry19ValidR {
        Keycam0Entry19ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry19_addr0(&self) -> Keycam0Entry19Addr0R {
        Keycam0Entry19Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry19_keyid_bit(
        &mut self,
    ) -> Keycam0Entry19KeyidBitW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry19_type(&mut self) -> Keycam0Entry19TypeW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry19_mickey_bit(
        &mut self,
    ) -> Keycam0Entry19MickeyBitW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry19_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry19GrpkeyBitW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry19_mgnt(&mut self) -> Keycam0Entry19MgntW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry19_repeater_mode(
        &mut self,
    ) -> Keycam0Entry19RepeaterModeW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry19_exttype(&mut self) -> Keycam0Entry19ExttypeW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry19_valid(&mut self) -> Keycam0Entry19ValidW<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry19_addr0(&mut self) -> Keycam0Entry19Addr0W<'_, RegKeycam0Entry19Spec> {
        Keycam0Entry19Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry19Spec;
impl crate::RegisterSpec for RegKeycam0Entry19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry19::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry19::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY19 to value 0"]
impl crate::Resettable for RegKeycam0Entry19Spec {}
