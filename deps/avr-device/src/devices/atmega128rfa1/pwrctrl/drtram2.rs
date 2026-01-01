#[doc = "Register `DRTRAM2` reader"]
pub struct R(crate::R<DRTRAM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRTRAM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRTRAM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRTRAM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRTRAM2` writer"]
pub struct W(crate::W<DRTRAM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRTRAM2_SPEC>;
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
impl From<crate::W<DRTRAM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRTRAM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDRT` reader - Enable SRAM Data Retention"]
pub type ENDRT_R = crate::BitReader<bool>;
#[doc = "Field `ENDRT` writer - Enable SRAM Data Retention"]
pub type ENDRT_W<'a, const O: u8> = crate::BitWriter<'a, u8, DRTRAM2_SPEC, bool, O>;
#[doc = "Field `DRTSWOK` reader - DRT Switch OK"]
pub type DRTSWOK_R = crate::BitReader<bool>;
#[doc = "Field `DRTSWOK` writer - DRT Switch OK"]
pub type DRTSWOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, DRTRAM2_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader<bool>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::BitWriter<'a, u8, DRTRAM2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Enable SRAM Data Retention"]
    #[inline(always)]
    pub fn endrt(&self) -> ENDRT_R {
        ENDRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DRT Switch OK"]
    #[inline(always)]
    pub fn drtswok(&self) -> DRTSWOK_R {
        DRTSWOK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enable SRAM Data Retention"]
    #[inline(always)]
    #[must_use]
    pub fn endrt(&mut self) -> ENDRT_W<4> {
        ENDRT_W::new(self)
    }
    #[doc = "Bit 5 - DRT Switch OK"]
    #[inline(always)]
    #[must_use]
    pub fn drtswok(&mut self) -> DRTSWOK_W<5> {
        DRTSWOK_W::new(self)
    }
    #[doc = "Bit 6 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<6> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Retention Configuration Register of SRAM 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drtram2](index.html) module"]
pub struct DRTRAM2_SPEC;
impl crate::RegisterSpec for DRTRAM2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [drtram2::R](R) reader structure"]
impl crate::Readable for DRTRAM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drtram2::W](W) writer structure"]
impl crate::Writable for DRTRAM2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRTRAM2 to value 0"]
impl crate::Resettable for DRTRAM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
