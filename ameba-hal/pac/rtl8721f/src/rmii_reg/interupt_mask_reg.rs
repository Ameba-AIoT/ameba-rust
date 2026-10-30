#[doc = "Register `Interupt_Mask_REG` reader"]
pub type R = crate::R<InteruptMaskRegSpec>;
#[doc = "Register `Interupt_Mask_REG` writer"]
pub type W = crate::W<InteruptMaskRegSpec>;
#[doc = "Field `RSTN` reader - Reset Notify"]
pub type RstnR = crate::BitReader;
#[doc = "Field `RSTN` writer - Reset Notify"]
pub type RstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPBKN` reader - Loopback Notify"]
pub type LpbknR = crate::BitReader;
#[doc = "Field `LPBKN` writer - Loopback Notify"]
pub type LpbknW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDWNN` reader - Power Down Notify"]
pub type PwrdwnnR = crate::BitReader;
#[doc = "Field `PWRDWNN` writer - Power Down Notify"]
pub type PwrdwnnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISON` reader - Isolate Notify"]
pub type IsonR = crate::BitReader;
#[doc = "Field `ISON` writer - Isolate Notify"]
pub type IsonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lx_slv_to_inten` reader - Interrupt enable of lexra slave timeout"]
pub type LxSlvToIntenR = crate::BitReader;
#[doc = "Field `lx_slv_to_inten` writer - Interrupt enable of lexra slave timeout"]
pub type LxSlvToIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lx_mst_to_inten` reader - Interrupt enable of lexra master timeout"]
pub type LxMstToIntenR = crate::BitReader;
#[doc = "Field `lx_mst_to_inten` writer - Interrupt enable of lexra master timeout"]
pub type LxMstToIntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset Notify"]
    #[inline(always)]
    pub fn rstn(&self) -> RstnR {
        RstnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Notify"]
    #[inline(always)]
    pub fn lpbkn(&self) -> LpbknR {
        LpbknR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Down Notify"]
    #[inline(always)]
    pub fn pwrdwnn(&self) -> PwrdwnnR {
        PwrdwnnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Isolate Notify"]
    #[inline(always)]
    pub fn ison(&self) -> IsonR {
        IsonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable of lexra slave timeout"]
    #[inline(always)]
    pub fn lx_slv_to_inten(&self) -> LxSlvToIntenR {
        LxSlvToIntenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable of lexra master timeout"]
    #[inline(always)]
    pub fn lx_mst_to_inten(&self) -> LxMstToIntenR {
        LxMstToIntenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Notify"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RstnW<'_, InteruptMaskRegSpec> {
        RstnW::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback Notify"]
    #[inline(always)]
    pub fn lpbkn(&mut self) -> LpbknW<'_, InteruptMaskRegSpec> {
        LpbknW::new(self, 1)
    }
    #[doc = "Bit 2 - Power Down Notify"]
    #[inline(always)]
    pub fn pwrdwnn(&mut self) -> PwrdwnnW<'_, InteruptMaskRegSpec> {
        PwrdwnnW::new(self, 2)
    }
    #[doc = "Bit 3 - Isolate Notify"]
    #[inline(always)]
    pub fn ison(&mut self) -> IsonW<'_, InteruptMaskRegSpec> {
        IsonW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable of lexra slave timeout"]
    #[inline(always)]
    pub fn lx_slv_to_inten(&mut self) -> LxSlvToIntenW<'_, InteruptMaskRegSpec> {
        LxSlvToIntenW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable of lexra master timeout"]
    #[inline(always)]
    pub fn lx_mst_to_inten(&mut self) -> LxMstToIntenW<'_, InteruptMaskRegSpec> {
        LxMstToIntenW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interupt_mask_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interupt_mask_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InteruptMaskRegSpec;
impl crate::RegisterSpec for InteruptMaskRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interupt_mask_reg::R`](R) reader structure"]
impl crate::Readable for InteruptMaskRegSpec {}
#[doc = "`write(|w| ..)` method takes [`interupt_mask_reg::W`](W) writer structure"]
impl crate::Writable for InteruptMaskRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Interupt_Mask_REG to value 0"]
impl crate::Resettable for InteruptMaskRegSpec {}
