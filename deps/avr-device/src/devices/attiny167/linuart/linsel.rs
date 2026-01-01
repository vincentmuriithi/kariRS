#[doc = "Register `LINSEL` reader"]
pub struct R(crate::R<LINSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINSEL` writer"]
pub struct W(crate::W<LINSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINSEL_SPEC>;
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
impl From<crate::W<LINSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINDX` reader - FIFO LIN Data Buffer Index bits"]
pub type LINDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINDX` writer - FIFO LIN Data Buffer Index bits"]
pub type LINDX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINSEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LAINC` reader - Auto Increment of Data Buffer Index (Active Low)"]
pub type LAINC_R = crate::BitReader<bool>;
#[doc = "Field `LAINC` writer - Auto Increment of Data Buffer Index (Active Low)"]
pub type LAINC_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - FIFO LIN Data Buffer Index bits"]
    #[inline(always)]
    pub fn lindx(&self) -> LINDX_R {
        LINDX_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Auto Increment of Data Buffer Index (Active Low)"]
    #[inline(always)]
    pub fn lainc(&self) -> LAINC_R {
        LAINC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO LIN Data Buffer Index bits"]
    #[inline(always)]
    #[must_use]
    pub fn lindx(&mut self) -> LINDX_W<0> {
        LINDX_W::new(self)
    }
    #[doc = "Bit 3 - Auto Increment of Data Buffer Index (Active Low)"]
    #[inline(always)]
    #[must_use]
    pub fn lainc(&mut self) -> LAINC_W<3> {
        LAINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Data Buffer Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linsel](index.html) module"]
pub struct LINSEL_SPEC;
impl crate::RegisterSpec for LINSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linsel::R](R) reader structure"]
impl crate::Readable for LINSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linsel::W](W) writer structure"]
impl crate::Writable for LINSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINSEL to value 0"]
impl crate::Resettable for LINSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
