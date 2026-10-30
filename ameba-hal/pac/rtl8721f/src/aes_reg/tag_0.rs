#[doc = "Register `tag_0` reader"]
pub type R = crate::R<Tag0Spec>;
#[doc = "Register `tag_0` writer"]
pub type W = crate::W<Tag0Spec>;
#[doc = "Field `tag_0` reader - Tag register bit\\[32*0+31:32*0\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
pub type Tag0R = crate::FieldReader<u32>;
#[doc = "Field `tag_0` writer - Tag register bit\\[32*0+31:32*0\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
pub type Tag0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tag register bit\\[32*0+31:32*0\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
    #[inline(always)]
    pub fn tag_0(&self) -> Tag0R {
        Tag0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tag register bit\\[32*0+31:32*0\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
    #[inline(always)]
    pub fn tag_0(&mut self) -> Tag0W<'_, Tag0Spec> {
        Tag0W::new(self, 0)
    }
}
#[doc = "Tag register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tag0Spec;
impl crate::RegisterSpec for Tag0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_0::R`](R) reader structure"]
impl crate::Readable for Tag0Spec {}
#[doc = "`write(|w| ..)` method takes [`tag_0::W`](W) writer structure"]
impl crate::Writable for Tag0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tag_0 to value 0"]
impl crate::Resettable for Tag0Spec {}
