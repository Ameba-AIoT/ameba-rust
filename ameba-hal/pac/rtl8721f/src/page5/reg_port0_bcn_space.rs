#[doc = "Register `REG_PORT0_BCN_SPACE` reader"]
pub type R = crate::R<RegPort0BcnSpaceSpec>;
#[doc = "Register `REG_PORT0_BCN_SPACE` writer"]
pub type W = crate::W<RegPort0BcnSpaceSpec>;
#[doc = "Field `BIT_BCN_SPACE_PORT0` reader - Unit is ms. When MBSSID is disable, this field is the beacon interval. When MBSSID is enable, this register is the beacon space of root."]
pub type BitBcnSpacePort0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BCN_SPACE_PORT0` writer - Unit is ms. When MBSSID is disable, this field is the beacon interval. When MBSSID is enable, this register is the beacon space of root."]
pub type BitBcnSpacePort0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_BCN_SUB_SPACE_PORT0` reader - Unit is ms. The sub-beacon interval of the port 0.(valid if mbssid function is enable)"]
pub type BitBcnSubSpacePort0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BCN_SUB_SPACE_PORT0` writer - Unit is ms. The sub-beacon interval of the port 0.(valid if mbssid function is enable)"]
pub type BitBcnSubSpacePort0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Unit is ms. When MBSSID is disable, this field is the beacon interval. When MBSSID is enable, this register is the beacon space of root."]
    #[inline(always)]
    pub fn bit_bcn_space_port0(&self) -> BitBcnSpacePort0R {
        BitBcnSpacePort0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Unit is ms. The sub-beacon interval of the port 0.(valid if mbssid function is enable)"]
    #[inline(always)]
    pub fn bit_bcn_sub_space_port0(&self) -> BitBcnSubSpacePort0R {
        BitBcnSubSpacePort0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unit is ms. When MBSSID is disable, this field is the beacon interval. When MBSSID is enable, this register is the beacon space of root."]
    #[inline(always)]
    pub fn bit_bcn_space_port0(&mut self) -> BitBcnSpacePort0W<'_, RegPort0BcnSpaceSpec> {
        BitBcnSpacePort0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Unit is ms. The sub-beacon interval of the port 0.(valid if mbssid function is enable)"]
    #[inline(always)]
    pub fn bit_bcn_sub_space_port0(&mut self) -> BitBcnSubSpacePort0W<'_, RegPort0BcnSpaceSpec> {
        BitBcnSubSpacePort0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_port0_bcn_space::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_port0_bcn_space::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPort0BcnSpaceSpec;
impl crate::RegisterSpec for RegPort0BcnSpaceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_port0_bcn_space::R`](R) reader structure"]
impl crate::Readable for RegPort0BcnSpaceSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_port0_bcn_space::W`](W) writer structure"]
impl crate::Writable for RegPort0BcnSpaceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PORT0_BCN_SPACE to value 0x0064_0064"]
impl crate::Resettable for RegPort0BcnSpaceSpec {
    const RESET_VALUE: u32 = 0x0064_0064;
}
