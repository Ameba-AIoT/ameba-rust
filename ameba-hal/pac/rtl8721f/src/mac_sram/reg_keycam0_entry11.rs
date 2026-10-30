#[doc = "Register `REG_KEYCAM0_ENTRY11` reader"]
pub type R = crate::R<RegKeycam0Entry11Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY11` writer"]
pub type W = crate::W<RegKeycam0Entry11Spec>;
#[doc = "Field `KEYCAM0_ENTRY11_KEYID_BIT` reader - "]
pub type Keycam0Entry11KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY11_KEYID_BIT` writer - "]
pub type Keycam0Entry11KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY11_TYPE` reader - "]
pub type Keycam0Entry11TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY11_TYPE` writer - "]
pub type Keycam0Entry11TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY11_MICKEY_BIT` reader - "]
pub type Keycam0Entry11MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY11_MICKEY_BIT` writer - "]
pub type Keycam0Entry11MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY11_GRPKEY_BIT` reader - "]
pub type Keycam0Entry11GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY11_GRPKEY_BIT` writer - "]
pub type Keycam0Entry11GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY11_MGNT` reader - "]
pub type Keycam0Entry11MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY11_MGNT` writer - "]
pub type Keycam0Entry11MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY11_REPEATER_MODE` reader - "]
pub type Keycam0Entry11RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY11_REPEATER_MODE` writer - "]
pub type Keycam0Entry11RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY11_EXTTYPE` reader - "]
pub type Keycam0Entry11ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY11_EXTTYPE` writer - "]
pub type Keycam0Entry11ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY11_VALID` reader - "]
pub type Keycam0Entry11ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY11_VALID` writer - "]
pub type Keycam0Entry11ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY11_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry11Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY11_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry11Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry11_keyid_bit(&self) -> Keycam0Entry11KeyidBitR {
        Keycam0Entry11KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry11_type(&self) -> Keycam0Entry11TypeR {
        Keycam0Entry11TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry11_mickey_bit(&self) -> Keycam0Entry11MickeyBitR {
        Keycam0Entry11MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry11_grpkey_bit(&self) -> Keycam0Entry11GrpkeyBitR {
        Keycam0Entry11GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry11_mgnt(&self) -> Keycam0Entry11MgntR {
        Keycam0Entry11MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry11_repeater_mode(&self) -> Keycam0Entry11RepeaterModeR {
        Keycam0Entry11RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry11_exttype(&self) -> Keycam0Entry11ExttypeR {
        Keycam0Entry11ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry11_valid(&self) -> Keycam0Entry11ValidR {
        Keycam0Entry11ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry11_addr0(&self) -> Keycam0Entry11Addr0R {
        Keycam0Entry11Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry11_keyid_bit(
        &mut self,
    ) -> Keycam0Entry11KeyidBitW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry11_type(&mut self) -> Keycam0Entry11TypeW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry11_mickey_bit(
        &mut self,
    ) -> Keycam0Entry11MickeyBitW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry11_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry11GrpkeyBitW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry11_mgnt(&mut self) -> Keycam0Entry11MgntW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry11_repeater_mode(
        &mut self,
    ) -> Keycam0Entry11RepeaterModeW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry11_exttype(&mut self) -> Keycam0Entry11ExttypeW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry11_valid(&mut self) -> Keycam0Entry11ValidW<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry11_addr0(&mut self) -> Keycam0Entry11Addr0W<'_, RegKeycam0Entry11Spec> {
        Keycam0Entry11Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry11Spec;
impl crate::RegisterSpec for RegKeycam0Entry11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry11::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry11::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY11 to value 0"]
impl crate::Resettable for RegKeycam0Entry11Spec {}
