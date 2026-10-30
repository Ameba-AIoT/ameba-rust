#[doc = "Register `REG_LPAD1` reader"]
pub type R = crate::R<RegLpad1Spec>;
#[doc = "Register `REG_LPAD1` writer"]
pub type W = crate::W<RegLpad1Spec>;
#[doc = "Field `reg_mbias_sel` reader - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type RegMbiasSelR = crate::FieldReader<u16>;
#[doc = "Field `reg_diode` reader - 1: enable diode"]
pub type RegDiodeR = crate::BitReader;
#[doc = "Field `bypassen_touch` reader - 1: bypass mode 0: divide 3.3V mode (en_mode_18=0) 0: divide 1.8V mode (en_mode_18=1)"]
pub type BypassenTouchR = crate::BitReader;
#[doc = "Field `lpad_dmy2` reader - Reserve for future function"]
pub type LpadDmy2R = crate::FieldReader<u16>;
#[doc = "Field `lpad_dmy2` writer - Reserve for future function"]
pub type LpadDmy2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn reg_mbias_sel(&self) -> RegMbiasSelR {
        RegMbiasSelR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 1: enable diode"]
    #[inline(always)]
    pub fn reg_diode(&self) -> RegDiodeR {
        RegDiodeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: bypass mode 0: divide 3.3V mode (en_mode_18=0) 0: divide 1.8V mode (en_mode_18=1)"]
    #[inline(always)]
    pub fn bypassen_touch(&self) -> BypassenTouchR {
        BypassenTouchR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - Reserve for future function"]
    #[inline(always)]
    pub fn lpad_dmy2(&self) -> LpadDmy2R {
        LpadDmy2R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 18:31 - Reserve for future function"]
    #[inline(always)]
    pub fn lpad_dmy2(&mut self) -> LpadDmy2W<'_, RegLpad1Spec> {
        LpadDmy2W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpad1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpad1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpad1Spec;
impl crate::RegisterSpec for RegLpad1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpad1::R`](R) reader structure"]
impl crate::Readable for RegLpad1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpad1::W`](W) writer structure"]
impl crate::Writable for RegLpad1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPAD1 to value 0"]
impl crate::Resettable for RegLpad1Spec {}
