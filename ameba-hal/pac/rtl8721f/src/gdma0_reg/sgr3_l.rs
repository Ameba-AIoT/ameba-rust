#[doc = "Register `SGR3_L` reader"]
pub type R = crate::R<Sgr3LSpec>;
#[doc = "Register `SGR3_L` writer"]
pub type W = crate::W<Sgr3LSpec>;
#[doc = "Field `SGR3_L_SGI` reader - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr3LSgiR = crate::FieldReader<u32>;
#[doc = "Field `SGR3_L_SGI` writer - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr3LSgiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SGR3_L_SGC` reader - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr3LSgcR = crate::FieldReader<u16>;
#[doc = "Field `SGR3_L_SGC` writer - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr3LSgcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr3_l_sgi(&self) -> Sgr3LSgiR {
        Sgr3LSgiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr3_l_sgc(&self) -> Sgr3LSgcR {
        Sgr3LSgcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr3_l_sgi(&mut self) -> Sgr3LSgiW<'_, Sgr3LSpec> {
        Sgr3LSgiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr3_l_sgc(&mut self) -> Sgr3LSgcW<'_, Sgr3LSpec> {
        Sgr3LSgcW::new(self, 20)
    }
}
#[doc = "Channel 3 low-32 bits Source Gather Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr3_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr3_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgr3LSpec;
impl crate::RegisterSpec for Sgr3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr3_l::R`](R) reader structure"]
impl crate::Readable for Sgr3LSpec {}
#[doc = "`write(|w| ..)` method takes [`sgr3_l::W`](W) writer structure"]
impl crate::Writable for Sgr3LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGR3_L to value 0"]
impl crate::Resettable for Sgr3LSpec {}
