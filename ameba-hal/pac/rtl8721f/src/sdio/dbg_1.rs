#[doc = "Register `DBG_1` reader"]
pub type R = crate::R<Dbg1Spec>;
#[doc = "Register `DBG_1` writer"]
pub type W = crate::W<Dbg1Spec>;
#[doc = "Field `enable` reader - Debug Enable. If set to 1, the debug port will be switched to the selected probed signals for observation. If clear to 0 (default), the scpu_dbg_out0 and scpu_dbg_out1 are both static at h0."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Debug Enable. If set to 1, the debug port will be switched to the selected probed signals for observation. If clear to 0 (default), the scpu_dbg_out0 and scpu_dbg_out1 are both static at h0."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `write_enable1` writer - Write enable for bit0."]
pub type WriteEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel0` reader - Select control of dbg_sel0."]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `sel0` writer - Select control of dbg_sel0."]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `write_enable2` writer - Write enable for bit\\[4..2\\]"]
pub type WriteEnable2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel1` reader - Select control of dbg_sel1."]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `sel1` writer - Select control of dbg_sel1."]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `write_enable3` writer - Write enable for bit\\[8..6\\]"]
pub type WriteEnable3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Enable. If set to 1, the debug port will be switched to the selected probed signals for observation. If clear to 0 (default), the scpu_dbg_out0 and scpu_dbg_out1 are both static at h0."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Select control of dbg_sel0."]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Select control of dbg_sel1."]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Enable. If set to 1, the debug port will be switched to the selected probed signals for observation. If clear to 0 (default), the scpu_dbg_out0 and scpu_dbg_out1 are both static at h0."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Dbg1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Write enable for bit0."]
    #[inline(always)]
    pub fn write_enable1(&mut self) -> WriteEnable1W<'_, Dbg1Spec> {
        WriteEnable1W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Select control of dbg_sel0."]
    #[inline(always)]
    pub fn sel0(&mut self) -> Sel0W<'_, Dbg1Spec> {
        Sel0W::new(self, 2)
    }
    #[doc = "Bit 5 - Write enable for bit\\[4..2\\]"]
    #[inline(always)]
    pub fn write_enable2(&mut self) -> WriteEnable2W<'_, Dbg1Spec> {
        WriteEnable2W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Select control of dbg_sel1."]
    #[inline(always)]
    pub fn sel1(&mut self) -> Sel1W<'_, Dbg1Spec> {
        Sel1W::new(self, 6)
    }
    #[doc = "Bit 9 - Write enable for bit\\[8..6\\]"]
    #[inline(always)]
    pub fn write_enable3(&mut self) -> WriteEnable3W<'_, Dbg1Spec> {
        WriteEnable3W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg1Spec;
impl crate::RegisterSpec for Dbg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_1::R`](R) reader structure"]
impl crate::Readable for Dbg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_1::W`](W) writer structure"]
impl crate::Writable for Dbg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_1 to value 0"]
impl crate::Resettable for Dbg1Spec {}
