#[doc = "Register `REG_BE_ADMTIME` reader"]
pub type R = crate::R<RegBeAdmtimeSpec>;
#[doc = "Register `REG_BE_ADMTIME` writer"]
pub type W = crate::W<RegBeAdmtimeSpec>;
#[doc = "Field `BIT_BE_ADMITTED_TIME` reader - Admitted_time of BE queue. In unit of 32 microsecond. Calculated and set by driver."]
pub type BitBeAdmittedTimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BE_ADMITTED_TIME` writer - Admitted_time of BE queue. In unit of 32 microsecond. Calculated and set by driver."]
pub type BitBeAdmittedTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_MHDR_NAV_OFFSET` reader - NDPA NAV compensation value"]
pub type BitMhdrNavOffsetR = crate::FieldReader;
#[doc = "Field `BIT_MHDR_NAV_OFFSET` writer - NDPA NAV compensation value"]
pub type BitMhdrNavOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DIS_NDPA_NAV_CHK` reader - 1-->tx NDPA/NDP can cross TBTT,0-->tx NDPA/NDP can't cross TBTT"]
pub type BitDisNdpaNavChkR = crate::BitReader;
#[doc = "Field `BIT_DIS_NDPA_NAV_CHK` writer - 1-->tx NDPA/NDP can cross TBTT,0-->tx NDPA/NDP can't cross TBTT"]
pub type BitDisNdpaNavChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - Admitted_time of BE queue. In unit of 32 microsecond. Calculated and set by driver."]
    #[inline(always)]
    pub fn bit_be_admitted_time(&self) -> BitBeAdmittedTimeR {
        BitBeAdmittedTimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - NDPA NAV compensation value"]
    #[inline(always)]
    pub fn bit_mhdr_nav_offset(&self) -> BitMhdrNavOffsetR {
        BitMhdrNavOffsetR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1-->tx NDPA/NDP can cross TBTT,0-->tx NDPA/NDP can't cross TBTT"]
    #[inline(always)]
    pub fn bit_dis_ndpa_nav_chk(&self) -> BitDisNdpaNavChkR {
        BitDisNdpaNavChkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Admitted_time of BE queue. In unit of 32 microsecond. Calculated and set by driver."]
    #[inline(always)]
    pub fn bit_be_admitted_time(&mut self) -> BitBeAdmittedTimeW<'_, RegBeAdmtimeSpec> {
        BitBeAdmittedTimeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - NDPA NAV compensation value"]
    #[inline(always)]
    pub fn bit_mhdr_nav_offset(&mut self) -> BitMhdrNavOffsetW<'_, RegBeAdmtimeSpec> {
        BitMhdrNavOffsetW::new(self, 16)
    }
    #[doc = "Bit 24 - 1-->tx NDPA/NDP can cross TBTT,0-->tx NDPA/NDP can't cross TBTT"]
    #[inline(always)]
    pub fn bit_dis_ndpa_nav_chk(&mut self) -> BitDisNdpaNavChkW<'_, RegBeAdmtimeSpec> {
        BitDisNdpaNavChkW::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegBeAdmtimeSpec> {
        DummyW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_be_admtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_be_admtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeAdmtimeSpec;
impl crate::RegisterSpec for RegBeAdmtimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_be_admtime::R`](R) reader structure"]
impl crate::Readable for RegBeAdmtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_be_admtime::W`](W) writer structure"]
impl crate::Writable for RegBeAdmtimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BE_ADMTIME to value 0"]
impl crate::Resettable for RegBeAdmtimeSpec {}
