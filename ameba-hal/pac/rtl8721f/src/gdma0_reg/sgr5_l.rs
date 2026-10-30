#[doc = "Register `SGR5_L` reader"]
pub type R = crate::R<Sgr5LSpec>;
#[doc = "Register `SGR5_L` writer"]
pub type W = crate::W<Sgr5LSpec>;
#[doc = "Field `SGR5_L_SGI` reader - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr5LSgiR = crate::FieldReader<u32>;
#[doc = "Field `SGR5_L_SGI` writer - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr5LSgiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SGR5_L_SGC` reader - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr5LSgcR = crate::FieldReader<u16>;
#[doc = "Field `SGR5_L_SGC` writer - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr5LSgcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr5_l_sgi(&self) -> Sgr5LSgiR {
        Sgr5LSgiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr5_l_sgc(&self) -> Sgr5LSgcR {
        Sgr5LSgcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr5_l_sgi(&mut self) -> Sgr5LSgiW<'_, Sgr5LSpec> {
        Sgr5LSgiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr5_l_sgc(&mut self) -> Sgr5LSgcW<'_, Sgr5LSpec> {
        Sgr5LSgcW::new(self, 20)
    }
}
#[doc = "Channel 5 low-32 bits Source Gather Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr5_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr5_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgr5LSpec;
impl crate::RegisterSpec for Sgr5LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr5_l::R`](R) reader structure"]
impl crate::Readable for Sgr5LSpec {}
#[doc = "`write(|w| ..)` method takes [`sgr5_l::W`](W) writer structure"]
impl crate::Writable for Sgr5LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGR5_L to value 0"]
impl crate::Resettable for Sgr5LSpec {}
