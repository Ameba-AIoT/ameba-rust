#[doc = "Register `REG_PKT_LIFE_TIME` reader"]
pub type R = crate::R<RegPktLifeTimeSpec>;
#[doc = "Register `REG_PKT_LIFE_TIME` writer"]
pub type W = crate::W<RegPktLifeTimeSpec>;
#[doc = "Field `BIT_PKT_LIFTIME_VOVI` reader - VO/VI packet lifetime unit: 256us"]
pub type BitPktLiftimeVoviR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PKT_LIFTIME_VOVI` writer - VO/VI packet lifetime unit: 256us"]
pub type BitPktLiftimeVoviW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_PKT_LIFTIME_BEBK` reader - BE/BK packet lifetime unit: 256us"]
pub type BitPktLiftimeBebkR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PKT_LIFTIME_BEBK` writer - BE/BK packet lifetime unit: 256us"]
pub type BitPktLiftimeBebkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VO/VI packet lifetime unit: 256us"]
    #[inline(always)]
    pub fn bit_pkt_liftime_vovi(&self) -> BitPktLiftimeVoviR {
        BitPktLiftimeVoviR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BE/BK packet lifetime unit: 256us"]
    #[inline(always)]
    pub fn bit_pkt_liftime_bebk(&self) -> BitPktLiftimeBebkR {
        BitPktLiftimeBebkR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VO/VI packet lifetime unit: 256us"]
    #[inline(always)]
    pub fn bit_pkt_liftime_vovi(&mut self) -> BitPktLiftimeVoviW<'_, RegPktLifeTimeSpec> {
        BitPktLiftimeVoviW::new(self, 0)
    }
    #[doc = "Bits 16:31 - BE/BK packet lifetime unit: 256us"]
    #[inline(always)]
    pub fn bit_pkt_liftime_bebk(&mut self) -> BitPktLiftimeBebkW<'_, RegPktLifeTimeSpec> {
        BitPktLiftimeBebkW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkt_life_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pkt_life_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPktLifeTimeSpec;
impl crate::RegisterSpec for RegPktLifeTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkt_life_time::R`](R) reader structure"]
impl crate::Readable for RegPktLifeTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pkt_life_time::W`](W) writer structure"]
impl crate::Writable for RegPktLifeTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PKT_LIFE_TIME to value 0x1000_1000"]
impl crate::Resettable for RegPktLifeTimeSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
