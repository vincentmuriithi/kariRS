#[doc = "Register `PRR` reader"]
pub struct R(crate::R<PRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRR` writer"]
pub struct W(crate::W<PRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRR_SPEC>;
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
impl From<crate::W<PRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRUSART` reader - No Description."]
pub type PRUSART_R = crate::BitReader<bool>;
#[doc = "Field `PRUSART` writer - No Description."]
pub type PRUSART_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRUSI` reader - No Description."]
pub type PRUSI_R = crate::BitReader<bool>;
#[doc = "Field `PRUSI` writer - No Description."]
pub type PRUSI_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRTIM` reader - No Description."]
pub type PRTIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTIM` writer - No Description."]
pub type PRTIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PRR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn prusart(&self) -> PRUSART_R {
        PRUSART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn prusi(&self) -> PRUSI_R {
        PRUSI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - No Description."]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn prusart(&mut self) -> PRUSART_W<0> {
        PRUSART_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn prusi(&mut self) -> PRUSI_W<1> {
        PRUSI_W::new(self)
    }
    #[doc = "Bits 2:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn prtim(&mut self) -> PRTIM_W<2> {
        PRTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power reduction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prr](index.html) module"]
pub struct PRR_SPEC;
impl crate::RegisterSpec for PRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prr::R](R) reader structure"]
impl crate::Readable for PRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prr::W](W) writer structure"]
impl crate::Writable for PRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR to value 0"]
impl crate::Resettable for PRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
