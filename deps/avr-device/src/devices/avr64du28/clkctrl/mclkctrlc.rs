#[doc = "Register `MCLKCTRLC` reader"]
pub struct R(crate::R<MCLKCTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKCTRLC` writer"]
pub struct W(crate::W<MCLKCTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCTRLC_SPEC>;
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
impl From<crate::W<MCLKCTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDEN` reader - Clock Failure Detect Enable"]
pub type CFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CFDEN` writer - Clock Failure Detect Enable"]
pub type CFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKCTRLC_SPEC, bool, O>;
#[doc = "Field `CFDTST` writer - Clock Failure Detect Test"]
pub type CFDTST_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKCTRLC_SPEC, bool, O>;
#[doc = "Field `CFDSRC` reader - Clock Failure Detect Source"]
pub type CFDSRC_R = crate::FieldReader<u8, CFDSRC_A>;
#[doc = "Clock Failure Detect Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDSRC_A {
    #[doc = "0: Main Clock"]
    CLKMAIN = 0,
    #[doc = "1: XOSCHF"]
    XOSCHF = 1,
    #[doc = "2: XOSC32K"]
    XOSC32K = 2,
}
impl From<CFDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDSRC_A) -> Self {
        variant as _
    }
}
impl CFDSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFDSRC_A> {
        match self.bits {
            0 => Some(CFDSRC_A::CLKMAIN),
            1 => Some(CFDSRC_A::XOSCHF),
            2 => Some(CFDSRC_A::XOSC32K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKMAIN`"]
    #[inline(always)]
    pub fn is_clkmain(&self) -> bool {
        *self == CFDSRC_A::CLKMAIN
    }
    #[doc = "Checks if the value of the field is `XOSCHF`"]
    #[inline(always)]
    pub fn is_xoschf(&self) -> bool {
        *self == CFDSRC_A::XOSCHF
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == CFDSRC_A::XOSC32K
    }
}
#[doc = "Field `CFDSRC` writer - Clock Failure Detect Source"]
pub type CFDSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MCLKCTRLC_SPEC, u8, CFDSRC_A, 2, O>;
impl<'a, const O: u8> CFDSRC_W<'a, O> {
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn clkmain(self) -> &'a mut W {
        self.variant(CFDSRC_A::CLKMAIN)
    }
    #[doc = "XOSCHF"]
    #[inline(always)]
    pub fn xoschf(self) -> &'a mut W {
        self.variant(CFDSRC_A::XOSCHF)
    }
    #[doc = "XOSC32K"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(CFDSRC_A::XOSC32K)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detect Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Failure Detect Source"]
    #[inline(always)]
    pub fn cfdsrc(&self) -> CFDSRC_R {
        CFDSRC_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<0> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 1 - Clock Failure Detect Test"]
    #[inline(always)]
    #[must_use]
    pub fn cfdtst(&mut self) -> CFDTST_W<1> {
        CFDTST_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Failure Detect Source"]
    #[inline(always)]
    #[must_use]
    pub fn cfdsrc(&mut self) -> CFDSRC_W<2> {
        CFDSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkctrlc](index.html) module"]
pub struct MCLKCTRLC_SPEC;
impl crate::RegisterSpec for MCLKCTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclkctrlc::R](R) reader structure"]
impl crate::Readable for MCLKCTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkctrlc::W](W) writer structure"]
impl crate::Writable for MCLKCTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKCTRLC to value 0"]
impl crate::Resettable for MCLKCTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
