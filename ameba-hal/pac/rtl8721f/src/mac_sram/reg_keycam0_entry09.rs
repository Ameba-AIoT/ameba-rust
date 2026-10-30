#[doc = "Register `REG_KEYCAM0_ENTRY09` reader"]
pub type R = crate::R<RegKeycam0Entry09Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY09` writer"]
pub type W = crate::W<RegKeycam0Entry09Spec>;
#[doc = "Field `KEYCAM0_ENTRY09_KEYID_BIT` reader - "]
pub type Keycam0Entry09KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY09_KEYID_BIT` writer - "]
pub type Keycam0Entry09KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY09_TYPE` reader - "]
pub type Keycam0Entry09TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY09_TYPE` writer - "]
pub type Keycam0Entry09TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY09_MICKEY_BIT` reader - "]
pub type Keycam0Entry09MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY09_MICKEY_BIT` writer - "]
pub type Keycam0Entry09MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY09_GRPKEY_BIT` reader - "]
pub type Keycam0Entry09GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY09_GRPKEY_BIT` writer - "]
pub type Keycam0Entry09GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY09_MGNT` reader - "]
pub type Keycam0Entry09MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY09_MGNT` writer - "]
pub type Keycam0Entry09MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY09_REPEATER_MODE` reader - "]
pub type Keycam0Entry09RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY09_REPEATER_MODE` writer - "]
pub type Keycam0Entry09RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY09_EXTTYPE` reader - "]
pub type Keycam0Entry09ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY09_EXTTYPE` writer - "]
pub type Keycam0Entry09ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY09_VALID` reader - "]
pub type Keycam0Entry09ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY09_VALID` writer - "]
pub type Keycam0Entry09ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY09_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry09Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY09_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry09Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry09_keyid_bit(&self) -> Keycam0Entry09KeyidBitR {
        Keycam0Entry09KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry09_type(&self) -> Keycam0Entry09TypeR {
        Keycam0Entry09TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry09_mickey_bit(&self) -> Keycam0Entry09MickeyBitR {
        Keycam0Entry09MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry09_grpkey_bit(&self) -> Keycam0Entry09GrpkeyBitR {
        Keycam0Entry09GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry09_mgnt(&self) -> Keycam0Entry09MgntR {
        Keycam0Entry09MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry09_repeater_mode(&self) -> Keycam0Entry09RepeaterModeR {
        Keycam0Entry09RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry09_exttype(&self) -> Keycam0Entry09ExttypeR {
        Keycam0Entry09ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry09_valid(&self) -> Keycam0Entry09ValidR {
        Keycam0Entry09ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry09_addr0(&self) -> Keycam0Entry09Addr0R {
        Keycam0Entry09Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry09_keyid_bit(
        &mut self,
    ) -> Keycam0Entry09KeyidBitW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry09_type(&mut self) -> Keycam0Entry09TypeW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry09_mickey_bit(
        &mut self,
    ) -> Keycam0Entry09MickeyBitW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry09_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry09GrpkeyBitW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry09_mgnt(&mut self) -> Keycam0Entry09MgntW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry09_repeater_mode(
        &mut self,
    ) -> Keycam0Entry09RepeaterModeW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry09_exttype(&mut self) -> Keycam0Entry09ExttypeW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry09_valid(&mut self) -> Keycam0Entry09ValidW<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry09_addr0(&mut self) -> Keycam0Entry09Addr0W<'_, RegKeycam0Entry09Spec> {
        Keycam0Entry09Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry09::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry09::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry09Spec;
impl crate::RegisterSpec for RegKeycam0Entry09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry09::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry09Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry09::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry09Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY09 to value 0"]
impl crate::Resettable for RegKeycam0Entry09Spec {}
