#[doc = "Register `CCLROUTEA` reader"]
pub struct R(crate::R<CCLROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCLROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCLROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCLROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCLROUTEA` writer"]
pub struct W(crate::W<CCLROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCLROUTEA_SPEC>;
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
impl From<crate::W<CCLROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCLROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT0` reader - CCL LUT0"]
pub type LUT0_R = crate::BitReader<bool>;
#[doc = "Field `LUT0` writer - CCL LUT0"]
pub type LUT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, bool, O>;
#[doc = "Field `LUT1` reader - CCL LUT1"]
pub type LUT1_R = crate::BitReader<bool>;
#[doc = "Field `LUT1` writer - CCL LUT1"]
pub type LUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, bool, O>;
#[doc = "Field `LUT2` reader - CCL LUT2"]
pub type LUT2_R = crate::BitReader<bool>;
#[doc = "Field `LUT2` writer - CCL LUT2"]
pub type LUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, bool, O>;
#[doc = "Field `LUT3` reader - CCL LUT3"]
pub type LUT3_R = crate::BitReader<bool>;
#[doc = "Field `LUT3` writer - CCL LUT3"]
pub type LUT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, CCLROUTEA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CCL LUT0"]
    #[inline(always)]
    pub fn lut0(&self) -> LUT0_R {
        LUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCL LUT1"]
    #[inline(always)]
    pub fn lut1(&self) -> LUT1_R {
        LUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCL LUT2"]
    #[inline(always)]
    pub fn lut2(&self) -> LUT2_R {
        LUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCL LUT3"]
    #[inline(always)]
    pub fn lut3(&self) -> LUT3_R {
        LUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCL LUT0"]
    #[inline(always)]
    #[must_use]
    pub fn lut0(&mut self) -> LUT0_W<0> {
        LUT0_W::new(self)
    }
    #[doc = "Bit 1 - CCL LUT1"]
    #[inline(always)]
    #[must_use]
    pub fn lut1(&mut self) -> LUT1_W<1> {
        LUT1_W::new(self)
    }
    #[doc = "Bit 2 - CCL LUT2"]
    #[inline(always)]
    #[must_use]
    pub fn lut2(&mut self) -> LUT2_W<2> {
        LUT2_W::new(self)
    }
    #[doc = "Bit 3 - CCL LUT3"]
    #[inline(always)]
    #[must_use]
    pub fn lut3(&mut self) -> LUT3_W<3> {
        LUT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer CCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cclroutea](index.html) module"]
pub struct CCLROUTEA_SPEC;
impl crate::RegisterSpec for CCLROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cclroutea::R](R) reader structure"]
impl crate::Readable for CCLROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cclroutea::W](W) writer structure"]
impl crate::Writable for CCLROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCLROUTEA to value 0"]
impl crate::Resettable for CCLROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
