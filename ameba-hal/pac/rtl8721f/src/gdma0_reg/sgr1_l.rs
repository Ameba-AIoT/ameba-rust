#[doc = "Register `SGR1_L` reader"]
pub type R = crate::R<Sgr1LSpec>;
#[doc = "Register `SGR1_L` writer"]
pub type W = crate::W<Sgr1LSpec>;
#[doc = "Field `SGR1_L_SGI` reader - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr1LSgiR = crate::FieldReader<u32>;
#[doc = "Field `SGR1_L_SGI` writer - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr1LSgiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SGR1_L_SGC` reader - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr1LSgcR = crate::FieldReader<u16>;
#[doc = "Field `SGR1_L_SGC` writer - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
pub type Sgr1LSgcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr1_l_sgi(&self) -> Sgr1LSgiR {
        Sgr1LSgiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr1_l_sgc(&self) -> Sgr1LSgcR {
        Sgr1LSgcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Source gather interval. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr1_l_sgi(&mut self) -> Sgr1LSgiW<'_, Sgr1LSpec> {
        Sgr1LSgiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Source gather count. Source contiguous transfer count between successive gather boundaries. Note: if user not define DMAC_SRC_GAT_EN, it only can be readed."]
    #[inline(always)]
    pub fn sgr1_l_sgc(&mut self) -> Sgr1LSgcW<'_, Sgr1LSpec> {
        Sgr1LSgcW::new(self, 20)
    }
}
#[doc = "Channel 1 low-32 bits Source Gather Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr1_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr1_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgr1LSpec;
impl crate::RegisterSpec for Sgr1LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr1_l::R`](R) reader structure"]
impl crate::Readable for Sgr1LSpec {}
#[doc = "`write(|w| ..)` method takes [`sgr1_l::W`](W) writer structure"]
impl crate::Writable for Sgr1LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGR1_L to value 0"]
impl crate::Resettable for Sgr1LSpec {}
