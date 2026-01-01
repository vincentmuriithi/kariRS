#[doc = "Register `SHORT_ADDR_1` reader"]
pub struct R(crate::R<SHORT_ADDR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORT_ADDR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORT_ADDR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORT_ADDR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORT_ADDR_1` writer"]
pub struct W(crate::W<SHORT_ADDR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORT_ADDR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SHORT_ADDR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORT_ADDR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHORT_ADDR_` reader - MAC Short Address"]
pub type SHORT_ADDR__R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHORT_ADDR_` writer - MAC Short Address"]
pub type SHORT_ADDR__W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SHORT_ADDR_1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_(&self) -> SHORT_ADDR__R {
        SHORT_ADDR__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_(&mut self) -> SHORT_ADDR__W<0> {
        SHORT_ADDR__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Transceiver MAC Short Address Register (High Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [short_addr_1](index.html) module"]
pub struct SHORT_ADDR_1_SPEC;
impl crate::RegisterSpec for SHORT_ADDR_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [short_addr_1::R](R) reader structure"]
impl crate::Readable for SHORT_ADDR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [short_addr_1::W](W) writer structure"]
impl crate::Writable for SHORT_ADDR_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR_1 to value 0"]
impl crate::Resettable for SHORT_ADDR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
