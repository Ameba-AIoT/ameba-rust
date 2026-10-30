#[doc = "Register `Interupt_Status_REG` reader"]
pub type R = crate::R<InteruptStatusRegSpec>;
#[doc = "Register `Interupt_Status_REG` writer"]
pub type W = crate::W<InteruptStatusRegSpec>;
#[doc = "Field `RSTNS` reader - Reset Notify When set, indicate Isolate is detected. Write 1 to clear."]
pub type RstnsR = crate::BitReader;
#[doc = "Field `LPBKNS` reader - Loopback Notify When set, indicate Isolate is detected. Write 1 to clear."]
pub type LpbknsR = crate::BitReader;
#[doc = "Field `PWRDWNNS` reader - Power Down Notify When set, indicate Isolate is detected. Write 1 to clear."]
pub type PwrdwnnsR = crate::BitReader;
#[doc = "Field `ISONS` reader - Isolate Notify When set, indicate Isolate is detected. Write 1 to clear."]
pub type IsonsR = crate::BitReader;
#[doc = "Field `lx_slv_to_int` reader - Interrupt status of lexra slave timeout"]
pub type LxSlvToIntR = crate::BitReader;
#[doc = "Field `lx_slv_to_int` writer - Interrupt status of lexra slave timeout"]
pub type LxSlvToIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lx_mst_to_int` reader - Interrupt status of lexra master timeout"]
pub type LxMstToIntR = crate::BitReader;
#[doc = "Field `lx_mst_to_int` writer - Interrupt status of lexra master timeout"]
pub type LxMstToIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset Notify When set, indicate Isolate is detected. Write 1 to clear."]
    #[inline(always)]
    pub fn rstns(&self) -> RstnsR {
        RstnsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Notify When set, indicate Isolate is detected. Write 1 to clear."]
    #[inline(always)]
    pub fn lpbkns(&self) -> LpbknsR {
        LpbknsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Down Notify When set, indicate Isolate is detected. Write 1 to clear."]
    #[inline(always)]
    pub fn pwrdwnns(&self) -> PwrdwnnsR {
        PwrdwnnsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Isolate Notify When set, indicate Isolate is detected. Write 1 to clear."]
    #[inline(always)]
    pub fn isons(&self) -> IsonsR {
        IsonsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status of lexra slave timeout"]
    #[inline(always)]
    pub fn lx_slv_to_int(&self) -> LxSlvToIntR {
        LxSlvToIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status of lexra master timeout"]
    #[inline(always)]
    pub fn lx_mst_to_int(&self) -> LxMstToIntR {
        LxMstToIntR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Interrupt status of lexra slave timeout"]
    #[inline(always)]
    pub fn lx_slv_to_int(&mut self) -> LxSlvToIntW<'_, InteruptStatusRegSpec> {
        LxSlvToIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt status of lexra master timeout"]
    #[inline(always)]
    pub fn lx_mst_to_int(&mut self) -> LxMstToIntW<'_, InteruptStatusRegSpec> {
        LxMstToIntW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interupt_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interupt_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InteruptStatusRegSpec;
impl crate::RegisterSpec for InteruptStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interupt_status_reg::R`](R) reader structure"]
impl crate::Readable for InteruptStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`interupt_status_reg::W`](W) writer structure"]
impl crate::Writable for InteruptStatusRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Interupt_Status_REG to value 0"]
impl crate::Resettable for InteruptStatusRegSpec {}
