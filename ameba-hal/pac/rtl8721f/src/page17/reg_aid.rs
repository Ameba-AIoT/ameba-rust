#[doc = "Register `REG_AID` reader"]
pub type R = crate::R<RegAidSpec>;
#[doc = "Register `REG_AID` writer"]
pub type W = crate::W<RegAidSpec>;
#[doc = "Field `r_mac_aid12_0` reader - aid0"]
pub type RMacAid12_0R = crate::FieldReader<u16>;
#[doc = "Field `r_mac_aid12_0` writer - aid0"]
pub type RMacAid12_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_mac_aid12_1` reader - aid1"]
pub type RMacAid12_1R = crate::FieldReader<u16>;
#[doc = "Field `r_mac_aid12_1` writer - aid1"]
pub type RMacAid12_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - aid0"]
    #[inline(always)]
    pub fn r_mac_aid12_0(&self) -> RMacAid12_0R {
        RMacAid12_0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - aid1"]
    #[inline(always)]
    pub fn r_mac_aid12_1(&self) -> RMacAid12_1R {
        RMacAid12_1R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - aid0"]
    #[inline(always)]
    pub fn r_mac_aid12_0(&mut self) -> RMacAid12_0W<'_, RegAidSpec> {
        RMacAid12_0W::new(self, 0)
    }
    #[doc = "Bits 12:23 - aid1"]
    #[inline(always)]
    pub fn r_mac_aid12_1(&mut self) -> RMacAid12_1W<'_, RegAidSpec> {
        RMacAid12_1W::new(self, 12)
    }
}
#[doc = "aid register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAidSpec;
impl crate::RegisterSpec for RegAidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aid::R`](R) reader structure"]
impl crate::Readable for RegAidSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_aid::W`](W) writer structure"]
impl crate::Writable for RegAidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AID to value 0"]
impl crate::Resettable for RegAidSpec {}
