#[doc = "Register `LINBTR` reader"]
pub struct R(crate::R<LINBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINBTR` writer"]
pub struct W(crate::W<LINBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINBTR_SPEC>;
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
impl From<crate::W<LINBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBT` reader - LIN Bit Timing bits"]
pub type LBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LBT` writer - LIN Bit Timing bits"]
pub type LBT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINBTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `LDISR` reader - Disable Bit Timing Resynchronization"]
pub type LDISR_R = crate::BitReader<bool>;
#[doc = "Field `LDISR` writer - Disable Bit Timing Resynchronization"]
pub type LDISR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINBTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - LIN Bit Timing bits"]
    #[inline(always)]
    pub fn lbt(&self) -> LBT_R {
        LBT_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 7 - Disable Bit Timing Resynchronization"]
    #[inline(always)]
    pub fn ldisr(&self) -> LDISR_R {
        LDISR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - LIN Bit Timing bits"]
    #[inline(always)]
    #[must_use]
    pub fn lbt(&mut self) -> LBT_W<0> {
        LBT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Bit Timing Resynchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ldisr(&mut self) -> LDISR_W<7> {
        LDISR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linbtr](index.html) module"]
pub struct LINBTR_SPEC;
impl crate::RegisterSpec for LINBTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linbtr::R](R) reader structure"]
impl crate::Readable for LINBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linbtr::W](W) writer structure"]
impl crate::Writable for LINBTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINBTR to value 0"]
impl crate::Resettable for LINBTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
