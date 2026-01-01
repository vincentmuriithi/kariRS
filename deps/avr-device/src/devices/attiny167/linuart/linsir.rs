#[doc = "Register `LINSIR` reader"]
pub struct R(crate::R<LINSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINSIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINSIR` writer"]
pub struct W(crate::W<LINSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINSIR_SPEC>;
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
impl From<crate::W<LINSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINSIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LRXOK` reader - Receive Performed Interrupt"]
pub type LRXOK_R = crate::BitReader<bool>;
#[doc = "Field `LRXOK` writer - Receive Performed Interrupt"]
pub type LRXOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINSIR_SPEC, bool, O>;
#[doc = "Field `LTXOK` reader - Transmit Performed Interrupt"]
pub type LTXOK_R = crate::BitReader<bool>;
#[doc = "Field `LTXOK` writer - Transmit Performed Interrupt"]
pub type LTXOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINSIR_SPEC, bool, O>;
#[doc = "Field `LIDOK` reader - Identifier Interrupt"]
pub type LIDOK_R = crate::BitReader<bool>;
#[doc = "Field `LIDOK` writer - Identifier Interrupt"]
pub type LIDOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINSIR_SPEC, bool, O>;
#[doc = "Field `LERR` reader - Error Interrupt"]
pub type LERR_R = crate::BitReader<bool>;
#[doc = "Field `LERR` writer - Error Interrupt"]
pub type LERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINSIR_SPEC, bool, O>;
#[doc = "Field `LBUSY` reader - Busy Signal"]
pub type LBUSY_R = crate::BitReader<bool>;
#[doc = "Field `LBUSY` writer - Busy Signal"]
pub type LBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINSIR_SPEC, bool, O>;
#[doc = "Field `LIDST` reader - Identifier Status bits"]
pub type LIDST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIDST` writer - Identifier Status bits"]
pub type LIDST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINSIR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Receive Performed Interrupt"]
    #[inline(always)]
    pub fn lrxok(&self) -> LRXOK_R {
        LRXOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Performed Interrupt"]
    #[inline(always)]
    pub fn ltxok(&self) -> LTXOK_R {
        LTXOK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier Interrupt"]
    #[inline(always)]
    pub fn lidok(&self) -> LIDOK_R {
        LIDOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline(always)]
    pub fn lerr(&self) -> LERR_R {
        LERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy Signal"]
    #[inline(always)]
    pub fn lbusy(&self) -> LBUSY_R {
        LBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Identifier Status bits"]
    #[inline(always)]
    pub fn lidst(&self) -> LIDST_R {
        LIDST_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lrxok(&mut self) -> LRXOK_W<0> {
        LRXOK_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ltxok(&mut self) -> LTXOK_W<1> {
        LTXOK_W::new(self)
    }
    #[doc = "Bit 2 - Identifier Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lidok(&mut self) -> LIDOK_W<2> {
        LIDOK_W::new(self)
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lerr(&mut self) -> LERR_W<3> {
        LERR_W::new(self)
    }
    #[doc = "Bit 4 - Busy Signal"]
    #[inline(always)]
    #[must_use]
    pub fn lbusy(&mut self) -> LBUSY_W<4> {
        LBUSY_W::new(self)
    }
    #[doc = "Bits 5:7 - Identifier Status bits"]
    #[inline(always)]
    #[must_use]
    pub fn lidst(&mut self) -> LIDST_W<5> {
        LIDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Status and Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linsir](index.html) module"]
pub struct LINSIR_SPEC;
impl crate::RegisterSpec for LINSIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linsir::R](R) reader structure"]
impl crate::Readable for LINSIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linsir::W](W) writer structure"]
impl crate::Writable for LINSIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINSIR to value 0"]
impl crate::Resettable for LINSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
